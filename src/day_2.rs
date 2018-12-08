#[cfg(test)]
mod tests {
    use crate::load_and_parse_input;
    use std::collections::HashMap;

    #[derive(Debug)]
    struct Duplicates {
        has_two: bool,
        has_three: bool,
    }

    fn count_duplicates(input: String) -> Duplicates {
        let mut counter = HashMap::new();

        for c in input.chars() {
            let entry = counter.entry(c).or_insert(0);
            *entry += 1;
        }

        let mut has_two = false;
        let mut has_three = false;
        for &v in counter.values() {
            if v == 2 && v != 3 {
                has_two = true;
            }
            if v == 3 && v != 2 {
                has_three = true;
            }

            if has_two && has_three {
                break;
            }
        }
        Duplicates { has_two, has_three }
    }

    #[test]
    fn can_find_checksum() {
        // note: this code is not very efficient if you happen to be aiming to learn something from it
        // in particular we loop over the input multiple times
        let filename = "input/day_2.txt";
        let counts = load_and_parse_input(filename, count_duplicates).unwrap();
        let totals = counts
            .iter()
            .fold((0, 0), |(mut next_two, mut next_three), next| {
                if next.has_two {
                    next_two += 1;
                }

                if next.has_three {
                    next_three += 1;
                }

                (next_two, next_three)
            });
        let checksum = totals.0 * totals.1;
        assert_eq!(checksum, 4980);
    }

    #[allow(dead_code)]
    fn test_input() -> Vec<String> {
        let input = vec![
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ];
        input.iter().map(|&s| s.into()).collect::<Vec<_>>()
    }

    // this is wonky
    fn diff_by(count: usize, a: &str, b: &str) -> Option<usize> {
        let diffs = a
            .chars()
            .zip(b.chars())
            .enumerate()
            .filter_map(|(i, (c, d))| if c != d { Some(i) } else { None })
            .collect::<Vec<_>>();
        if diffs.len() == count {
            Some(diffs[0])
        } else {
            None
        }
    }

    #[test]
    fn can_diff_by() {
        let result = diff_by(1, "abc", "abc");
        assert!(result.is_none());
        let result = diff_by(1, "adc", "abc");
        assert_eq!(result.unwrap(), 1);
        let result = diff_by(1, "abc", "def");
        assert!(result.is_none());
    }

    #[test]
    fn can_find_correct_ids() {
        let filename = "input/day_2.txt";
        let inputs = load_and_parse_input(filename, |x| x).unwrap();

        let mut result = None;
        for (i, input) in inputs.iter().enumerate() {
            for (j, candidate) in inputs.iter().enumerate() {
                if i == j {
                    continue;
                }

                if let Some(pos) = diff_by(1, input, candidate) {
                    let mut input = input.clone();
                    input.remove(pos);
                    result = Some(input);
                    break;
                }
            }
            if result.is_some() {
                break;
            }
        }
        assert_eq!(result, Some("qysdtrkloagnfozuwujmhrbvx".into()));
    }
}
