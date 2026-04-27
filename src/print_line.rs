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
