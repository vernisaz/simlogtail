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
    time::{SystemTime, UNIX_EPOCH},
};

use crate::simcli::{CLI, OptTyp, OptVal};

const VERSION: &str = env!("VERSION");

const NAME: &str = env!("NAME");

include!("print_line.rs");

/// Reads a file and returns the first `n` lines as a vector of strings.
///
/// # Arguments
///
/// * `path` - A reference to the file path.
/// * `n` - The number of lines to read from the begining of the file.
/// * 'flag' - bool flag if skip empty lines in Result
///
/// # Returns `Result<Vec<String>, Error>`
///
/// A `Result` containing a `Vec<String>` of the first `n` lines,
/// or an `io::Error` if the file cannot be read.
pub fn read_first_n_lines<P: AsRef<Path>>(
    path: P,
    n: usize,
    skip_empty: bool,
) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut res = Vec::with_capacity(n);
    let mut count = 0_usize;
    for line in reader.lines() {
        if count >= n {
            break;
        }
        let line = line?; // Handle potential errors
        if skip_empty && line.trim().is_empty() {
            continue;
        }
        res.push(line);
        count += 1
    }
    Ok(res)
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
        .opt("c", OptTyp::None)?
        .description("Do not show and count empty lines in the out");
    if cli.get_errors().is_some() {
        eprintln!("{}", "Some unknown options are ignored".yellow())
    }
    if cli.get_opt("v") == Some(&OptVal::Empty) {
        #[allow(clippy::unit_arg)]
        return Ok(println!(
            "\n{} version {}, Copyright © {} D. Rogatkin",
            NAME.blue().bright().bold(),
            VERSION.green(),
            year_now().bright().magenta()
        ));
    } else if cli.get_opt("h") == Some(&OptVal::Empty) {
        return Ok(println!(
            "Usage: simhead [opts] <file path>[ ...<file path>]\n{}",
            cli.get_description().unwrap().bright().blue()
        ));
    } else if cli.args().is_empty() {
        return Err("No file specified".red().into());
    }
    let compact = cli.get_opt("c") == Some(&OptVal::Empty);
    let lns = match cli.get_opt("n") {
        Some(OptVal::Num(n)) => *n as usize,
        _ => 15usize,
    };
    let (tz_off, _dst) = simtime::get_local_timezone_offset_dst();
    for arg in cli.args() {
        match read_first_n_lines(arg, lns, compact) {
            Ok(lines) => {
                println!(
                    "\nFirst {lns} lines (or fewer if not available) of {}:",
                    &arg.clone().green()
                );
                for line in lines {
                    print_ln(&line, tz_off)
                }
            }
            Err(e) => eprintln!("Error reading file {}: {}", arg.clone().red(), e),
        }
    }
    Ok(())
}
