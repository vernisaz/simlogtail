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

/// Reads a file and returns the first `n` lines as a vector of strings.
///
/// # Arguments
///
/// * `path` - A reference to the file path.
/// * `n` - The number of lines to read from the begining of the file.
///
/// # Returns
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
    cli.description("Where opts:")
        .opt("n", OptTyp::Num)?
        .description("Number of shown lines")
        .opt("v", OptTyp::None)?
        .description("Version")
        .opt("h", OptTyp::None)?
        .opt("c", OptTyp::None)?
        .description("Do not show empty lines in the out");
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
    let lns = match cli.get_opt("n") {
        Some(OptVal::Num(n)) => *n as usize,
        _ => 15usize,
    };
    match read_first_n_lines(cli.args().first().unwrap(), lns, compact) {
        Ok(lines) => {
            println!(
                "\nFirst {lns} lines (or fewer if not available) of {}:",
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
