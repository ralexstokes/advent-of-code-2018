use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

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

fn parse_frequency(s: String) -> i64 {
    str::parse::<i64>(&s).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;

    #[test] // part 1
    fn can_find_sum_frequency() {
        let filename = "input/day_1.txt";
        let input = load_and_parse_input(filename, parse_frequency).unwrap();

        let result: i64 = input.iter().sum();

        assert_eq!(result, 484);
    }

    #[test] // part 2
    fn can_find_duplicates() {
        let filename = "input/day_1.txt";
        let input = load_and_parse_input(filename, parse_frequency).unwrap();

        let mut seen = HashSet::new();
        let mut current_frequency = 0;

        seen.insert(current_frequency);

        for &entry in input.iter().cycle() {
            current_frequency += entry;
            let contains = !seen.insert(current_frequency);
            if contains {
                break;
            }
        }

        assert_eq!(current_frequency, 367);
    }

    #[test]
    fn can_find_duplicates_with_iters() {
        let filename = "input/day_1.txt";
        let input = load_and_parse_input(filename, parse_frequency).unwrap();

        let mut seen = HashSet::new();
        let current_frequency = 0;

        seen.insert(current_frequency);

        let result = input
            .iter()
            .cycle()
            .scan(current_frequency, |current_frequency, &next| {
                *current_frequency += next;

                Some(*current_frequency)
            }).find(|&f| !seen.insert(f));

        assert_eq!(result, Some(367));
    }
}