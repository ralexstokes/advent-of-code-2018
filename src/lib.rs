use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

mod day_1;
mod day_2;

#[allow(dead_code)]
fn load_and_parse_input<T, P: AsRef<Path>, F: Fn(String) -> T>(
    path: P,
    parser: F,
) -> io::Result<Vec<T>> {
    let f = File::open(path)?;
    let file = BufReader::new(f);
    let input = file
        .lines()
        .map(|line| parser(line.unwrap()))
        .collect::<Vec<_>>();
    Ok(input)
}
