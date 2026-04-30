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
    io::{self, BufRead, BufReader, Seek, SeekFrom},
    path::Path,
    sync::mpsc::{self, Receiver, TryRecvError},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[cfg(not(target_os = "windows"))]
use simcli::{CLI, OptTyp, OptVal};
#[cfg(target_os = "windows")]
use simcli::{CLI, OptTyp, OptVal, WildCardExpansion};

const VERSION: &str = env!("VERSION");

const NAME: &str = env!("NAME");

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

include!("print_line.rs");

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
) -> Result<(Vec<String>, u64), std::io::Error> {
    let mut file = File::open(path)?;
    let reader = BufReader::new(&file);
    let mut res = CircularBuffer::with_capacity(n);
    // TODO investigate how seek to the end of the file and then read
    for line in reader.lines() {
        let line = line?; // Handle potential errors
        if skip_empty && line.trim().is_empty() {
            continue;
        }
        res.push(line);
    }
    let current = file.stream_position()?;
    Ok(if res.head == 0 {
        (res.buffer, current)
    } else {
        let mut res2 = Vec::with_capacity(res.buffer.len());
        res2.extend_from_slice(&res.buffer[res.head..]);
        res2.extend_from_slice(&res.buffer[..res.tail]);
        (res2, current)
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut cli = CLI::new();
    cli.description("Where opts are:")
        .opt("n", OptTyp::Num)?
        .description("Number of shown lines")
        .opt("v", OptTyp::None)?
        .description("Version of the product")
        .opt("h", OptTyp::None)?
        .description("This help screen")
        .opt("f", OptTyp::None)?
        .description("Real time tail monitoring (only list file in the list)")
        .opt("c", OptTyp::None)?
        .description("Do not show and count empty lines in the out");
    #[cfg(target_os = "windows")]
    cli.process_wildcard(WildCardExpansion::All);
    if cli.get_errors().is_some() {
        eprintln!("{}", "Some unknown options are ignored".yellow())
    }
    let lns = match cli.get_opt("n") {
        Some(OptVal::Num(n)) => *n as usize,
        _ => 15usize,
    };
    if cli.get_opt("v") == Some(&OptVal::Empty) {
        #[allow(clippy::unit_arg)]
        return Ok(println!(
            "\n{} version {}, Copyright © {} D. Rogatkin",
            NAME.blue().bright().bold(),
            VERSION.green(),
            year_now().magenta().bright()
        ));
    } else if cli.get_opt("h") == Some(&OptVal::Empty) {
        return Ok(println!(
            "Usage: simtail [opts] <file path>[...<file path>]\n{}",
            cli.get_description().unwrap().bright().blue()
        ));
    } else if cli.args().is_empty() {
        return Err("No file specified".red().into());
    }
    let compact = cli.get_opt("c") == Some(&OptVal::Empty);
    let (tz_off, _dst) = simtime::get_local_timezone_offset_dst();
    let mut last_current = 0;
    for arg in cli.args() {
        match read_last_n_lines(arg, lns, compact) {
            Ok((lines, current)) => {
                println!(
                    "\nLast {lns} lines (or fewer if not available) of {}:",
                    arg.clone().green()
                );
                for line in lines {
                    print_ln(&line, tz_off)
                }
                last_current = current
            }
            Err(e) => eprintln!("Error reading file {}: {}", arg.clone().red(), e),
        }
    }
    if cli.get_opt("f").is_some()
        && let Some(arg) = cli.args().last()
    {
        monitor_file(arg, compact, last_current)
    } else {
        Ok(())
    }
}

fn monitor_file(path: &str, compact: bool, last_pos: u64) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(path)?;

    // Start at the end of the file to only read new data
    // TODO there is a gap in reading, so use the last size from the actual tail printing: last_pos
    let mut last_pos = if last_pos == 0 {
        file.seek(SeekFrom::End(0))?
    } else {
        last_pos
    };
    let stdin_channel = spawn_stdin_channel();
    let mut line = String::new();
    let (tz_off, _dst) = simtime::get_local_timezone_offset_dst();
    loop {
        thread::sleep(Duration::from_secs(2)); // Poll interval

        let meta = file.metadata()?;
        if meta.len() > last_pos {
            // Move to the last read position
            file.seek(SeekFrom::Start(last_pos))?;
            let mut reader = BufReader::new(&mut file);
            // Read new content
            while reader.read_line(&mut line)? > 0 {
                let trimmed = line.trim();
                if compact && trimmed.is_empty() {
                    continue;
                }
                print_ln(trimmed, tz_off);
                line.clear();
            }
            // Update last position to the current end
            last_pos = file.stream_position()?;
        }
        match stdin_channel.try_recv() {
            Ok(key) => {
                if key.contains("q") {
                    break Ok(());
                }
            } //println!("Received: {}", key),
            Err(TryRecvError::Empty) => (), //println!("Channel empty"),
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        }
    }
}

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            let _ = tx.send(buffer);
        }
    });
    rx
}
