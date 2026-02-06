extern crate simcolor;

use simcolor::Colorized;
use std::{env, error::Error, fs, path::Path};
extern crate simtime;
mod cli;
use crate::cli::{CLI, OptTyp, OptVal};

const VERSION: &str = env!("VERSION");
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
    let contents = fs::read_to_string(path)?;
    let lines: Vec<_> = contents.lines().collect();
    if skip_empty {
        let mut lines = lines.into_iter().rev();
        let mut res = Vec::new();
        while let Some(line) = lines.next()
            && res.len() < n
        {
            if !line.trim().is_empty() {
                res.push(line)
            }
        }
        Ok(res.into_iter().rev().map(|s| s.to_string()).collect())
    } else {
        let start_index = lines.len().saturating_sub(n);
        Ok(lines[start_index..]
            .iter()
            .map(|&s| s.to_string())
            .collect())
    }
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
        .description("Number lines")
        .opt("v", OptTyp::None)?
        .description("Version")
        .opt("h", OptTyp::None)?
        .opt("c", OptTyp::None)?
        .description("Compact empty lines in the tail");
    #[cfg(test)]
    test_cli(&mut cli);
    let lns = match cli.get_opt("n") {
        Some(OptVal::Num(n)) => *n as usize,
        _ => 15usize,
    };
    if cli.get_opt("v") == Some(&OptVal::Empty) {
        return Ok(println!("\nVersion {}", VERSION.green()));
    } else if cli.get_opt("h") == Some(&OptVal::Empty) || cli.args().len() != 1 {
        let message = format!(
            "Usage: simtail [opts] <file path>\n{}",
            cli.get_description().unwrap().bright().blue()
        );
        return Err(Box::new(message.bold()));
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
            .bold(),
        )),
    }
}
