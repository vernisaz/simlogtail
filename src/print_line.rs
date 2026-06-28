/// Prints a string with possible timestamp in it in provided time zone
///
///
fn print_ln(line: &str, tz_off: i16) {
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
/// Reads n bytes from the specified position
///
///
pub fn read_n_bytes_from(
    path: impl AsRef<Path>,
    num_bytes: usize,
    offset: u64,
) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(&path)?;
    // Move the cursor to the desired offset
    file.seek(SeekFrom::Start(offset))?;

    // Prepare a buffer of the desired size
    let mut buffer = vec![0u8; num_bytes];

    // Read exactly `num_bytes` into the buffer
    let bytes_read = file.read(&mut buffer)?;

    // If fewer bytes were read (e.g., end of file), truncate the buffer
    buffer.truncate(bytes_read);

    Ok(buffer)
}
#[inline]
pub fn year_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        / 31556952
        + 1970 // since 1970 isn't leap, substract 43,200 from now
}
