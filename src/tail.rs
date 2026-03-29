//! it is an example of Rust implementation of a popular Unix utility - tail
//! ## Purpose
//! I work on different systems where `tail` like command can be not available. Another reason
//! is
//! my log file entries contain a timestamp in milliseconds since the epoch. The program converts
//! the information in a human readable format.

extern crate simcli;
extern crate simcolor;
extern crate simtime;
use simcolor::Colorized;
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::simcli::{CLI, OptTyp, OptVal};

const VERSION: &str = env!("VERSION");

struct CircularBuffer<T> {
    buffer: Vec<T>,
    head: usize,
    tail: usize,
    capacity: usize,
    size: usize,
}

impl<T> CircularBuffer<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        CircularBuffer {
            buffer: Vec::with_capacity(capacity),
            head: 0,
            tail: 0,
            capacity,
            size: 0,
        }
    }

    pub fn push(&mut self, element: T) {
        if self.tail == self.buffer.len() {
            self.buffer.push(element)
        } else {
            self.buffer[self.tail] = element;
        }
        if self.size == self.capacity {
            self.head = (self.head + 1) % self.capacity; // Overwrite oldest
        } else {
            self.size += 1;
        }
        self.tail = (self.tail + 1) % self.capacity; // Move tail
    }
}

/// Reads a file and returns the last `n` lines as a vector of strings.
///
/// # Arguments
///
/// * `path` - A reference to the file path.
/// * `n` - The number of lines to read from the end of the file.
///
/// # Returns
///
/// A `Result` containing a `Vec<String>` of the last `n` lines,
/// or an `io::Error` if the file cannot be read.
pub fn read_last_n_lines<P: AsRef<Path>>(
    path: P,
    n: usize,
    skip_empty: bool,
) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut res = CircularBuffer::with_capacity(n);
    // TODO investigate how seek to the end of the file and then read
    for line in reader.lines() {
        let line = line?; // Handle potential errors
        if skip_empty && line.trim().is_empty() {
            continue;
        }
        res.push(line);
    }
    Ok(if res.head == 0 {
        res.buffer
    } else {
        let mut res2 = Vec::with_capacity(res.buffer.len());
        res2.extend_from_slice(&res.buffer[res.head..]);
        res2.extend_from_slice(&res.buffer[..res.tail]);
        res2
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut cli = CLI::new();
    cli.description("Where opts are:")
        .opt("n", OptTyp::Num)?
        .description("Number of shown lines")
        .opt("v", OptTyp::None)?
        .description("Version")
        .opt("h", OptTyp::None)?
        .opt("c", OptTyp::None)?
        .description("Do not show empty lines in the out");
    if cli.get_errors().is_some() {
        eprintln!("{}", "Some unknown options are ignored".yellow())
    }
    let lns = match cli.get_opt("n") {
        Some(OptVal::Num(n)) => *n as usize,
        _ => 15usize,
    };
    if cli.get_opt("v") == Some(&OptVal::Empty) {
        #[allow(clippy::unit_arg)]
        return Ok(println!("\nVersion {}", VERSION.green()));
    } else if cli.get_opt("h") == Some(&OptVal::Empty) {
        return Err(Box::new(
            format!(
                "Usage: simtail [opts] <file path>[...<file path>]\n{}",
                cli.get_description().unwrap().bright().blue()
            )
            .default(),
        ));
    } else if cli.args().is_empty() {
        return Err("No file specified".red().into());
    }
    let compact = cli.get_opt("c") == Some(&OptVal::Empty);
    for arg in cli.args() {
        match read_last_n_lines(arg, lns, compact) {
            Ok(lines) => {
                println!(
                    "\nLast {lns} lines (or fewer if not available) of {}:",
                    arg.clone().green()
                );
                let (tz_off, _dst) = simtime::get_local_timezone_offset_dst();
                for line in lines {
                    match line.split_once('[').and_then(|(before, after)| {
                        after
                            .split_once(']')
                            .and_then(|(date, tail)| match date.parse::<i64>() {
                                Ok(date) => {
                                    let (y, m, d, h, mm, s, _) = simtime::get_datetime(
                                        1970,
                                        (date / 1000i64 + (tz_off as i64) * 60i64) as u64,
                                    );
                                    let ms = date % 1000;
                                    Some(format!(
                                        "{before} {} {tail}",
                                        format!("{m}-{d:02}-{y} {h}:{mm:02}:{s:02}.{ms:03}")
                                            .blue()
                                            .on()
                                            .bright()
                                            .yellow()
                                    ))
                                }
                                _ => None,
                            })
                    }) {
                        Some(line) => println!("{}", line),
                        _ => println!("{}", line),
                    }
                }
            }
            Err(e) => eprintln!("Error reading file {} : {}", arg.clone().red(), e),
        }
    }
    Ok(())
}
