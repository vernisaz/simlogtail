extern crate simcolor;

use simcolor::Colorized;
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};
extern crate simtime;
mod cli;
use crate::cli::{CLI, OptTyp, OptVal};

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
    let mut res = CircularBuffer::<String>::with_capacity(n);
    for line in reader.lines() {
        let line = line?; // Handle potential errors
        if skip_empty && line.trim().is_empty() {
            continue;
        }
        res.push(line);
    }
    Ok(res.buffer)
}

#[cfg(test)]
fn test_cli(cli: &mut CLI) {
    match cli.opt("D", OptTyp::InStr) {
        Ok(ref mut cli) => {
            cli.description("A definition as name=value");
        }
        _ => (),
    }
    let _ = cli.opt("c", OptTyp::None).inspect_err(|e| eprintln!("{e}"));
    let d_o = cli.get_opt("D");
    if let Some(OptVal::Arr(d_o)) = d_o {
        for (i, d) in d_o.into_iter().enumerate() {
            eprintln!("opt[{i}] {}={}", d.0, d.1);
        }
    } else {
        eprintln!("no def found")
    }
    let _ = cli.opt("X", OptTyp::Str).inspect_err(|e| eprintln!("{e}"));
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut cli = CLI::new();
    cli.description("Where opts:")
        .opt("n", OptTyp::Num)?
        .description("Number of shown lines")
        .opt("v", OptTyp::None)?
        .description("Version")
        .opt("h", OptTyp::None)?
        .opt("c", OptTyp::None)?
        .description("Do not show empty lines in the out");
    #[cfg(test)]
    test_cli(&mut cli);
    let lns = match cli.get_opt("n") {
        Some(OptVal::Num(n)) => *n as usize,
        _ => 15usize,
    };
    if cli.get_opt("v") == Some(&OptVal::Empty) {
        #[allow(clippy::unit_arg)]
        return Ok(println!("\nVersion {}", VERSION.green()));
    } else if cli.get_opt("h") == Some(&OptVal::Empty) || cli.args().len() != 1 {
        return Err(Box::new(
            format!(
                "Usage: simtail [opts] <file path>\n{}",
                cli.get_description().unwrap().bright().blue()
            )
            .default(),
        ));
    }
    let compact = cli.get_opt("c") == Some(&OptVal::Empty);

    match read_last_n_lines(cli.args().first().unwrap(), lns, compact) {
        Ok(lines) => {
            println!(
                "\nLast {lns} lines (or fewer if not available) of {}:",
                &cli.args()[0].clone().green()
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
                                Some(format!(
                                    "{before} {} {tail}",
                                    format!("{m}-{d:02}-{y} {h}:{mm:02}:{s:02}")
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
            Ok(())
        }
        Err(e) => Err(Box::new(
            format!(
                "Error reading file {} : {}",
                cli.args().first().unwrap().clone().red(),
                e
            )
            .default(),
        )),
    }
}
