use std::error::Error;
use std::env;
use std::fs;
use std::path::Path;
extern crate simtime;
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
pub fn read_last_n_lines<P: AsRef<Path>>(path: P, n: usize) -> Result<Vec<String>, std::io::Error> {
    let contents = fs::read_to_string(path)?;
    let lines: Vec<&str> = contents.lines().collect();

    let start_index = lines.len().saturating_sub(n);
    let last_n_lines: Vec<String> = lines[start_index..]
        .iter()
        .map(|&s| s.to_string())
        .collect();

    Ok(last_n_lines)
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    args.next();
    args.next().and_then(|file_path:String| 
       Some( match read_last_n_lines(&file_path, 15) { // Requesting more lines than available
                Ok(lines) => {
                    println!("\nLast 15 lines (or fewer if not available) of {}:", file_path);
                    let (tz_off, _dst) = simtime::get_local_timezone_offset_dst();
                    for line in lines {
                        match line.split_once('[').
                        and_then(|(before,after)| {after.split_once(']')
                        .and_then(|(date,tail)| {
                            match date.parse::<i64>() {
                                Ok(date) => { let (y,m,d,h,mm,s,_) = simtime::get_datetime(1970, (date/1000i64 + (tz_off as i64) *60i64) as u64);
                                    Some(format!("{before} {m}-{d:02}-{y} {h}:{mm:02}:{s:02} {tail}"))},
                                _ => None,
                            } }) }) {
                                Some(line) => println!("{}", line),
                                _ => println!("{}", line)
                            }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading file: {}", e);
                }
            }
        )
    ).ok_or("no input file".into())
}