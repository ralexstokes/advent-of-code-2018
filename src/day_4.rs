#[cfg(test)]
mod tests {

    use chrono::{DateTime, TimeZone, Timelike, Utc};
    use regex::Regex;
    use std::collections::hash_map::Entry;
    use std::collections::HashMap;
    use std::str::FromStr;

    use crate::load_and_parse_input;

    type Span = (DateTime<Utc>, DateTime<Utc>);
    type SleepRecord = Vec<Span>;
    type GuardID = u16;

    #[derive(Debug, Clone)]
    struct Guard {
        id: GuardID,
        minutes_asleep: SleepRecord,
    }

    impl Guard {
        fn new(id: GuardID, minutes_asleep: SleepRecord) -> Self {
            Self { id, minutes_asleep }
        }

        fn total_minutes_asleep(&self) -> u64 {
            self.minutes_asleep
                .iter()
                .map(|&(start_asleep, end_asleep)| {
                    let duration = end_asleep - start_asleep;
                    duration.num_minutes() as u64
                })
                .sum()
        }

        fn add_sleep(&mut self, span: Span) {
            self.minutes_asleep.push(span)
        }

        fn add_sleep_record(&mut self, durations: &SleepRecord) {
            for duration in durations {
                self.minutes_asleep.push(duration.clone())
            }
        }

        fn sleepiest_minute(&self) -> usize {
            let mut counter = HashMap::new();
            for (start_sleep, end_sleep) in &self.minutes_asleep {
                let start_minute = start_sleep.minute();
                let duration = *end_sleep - *start_sleep;
                let minutes_duration = duration.num_minutes() as u32;
                for minute in start_minute..start_minute + minutes_duration {
                    let entry = counter.entry(minute).or_insert(0);
                    *entry += 1
                }
            }
            let (minute, _) = counter.iter().max_by_key(|(_, &count)| count).unwrap();
            *minute as usize
        }

        // (minute, count)
        fn most_frequent_minute_asleep(&self) -> Option<(usize, usize)> {
            let mut freqs = HashMap::<usize, usize>::new();

            if self.minutes_asleep.len() == 0 {
                return None;
            }

            for (start_sleep, end_sleep) in &self.minutes_asleep {
                let start_minute = start_sleep.minute();
                let duration = *end_sleep - *start_sleep;
                let minutes_duration = duration.num_minutes() as u32;
                for minute in start_minute..start_minute + minutes_duration {
                    let entry = freqs.entry(minute as usize).or_insert(0);
                    *entry += 1
                }
            }

            Some(freqs.into_iter().max_by_key(|&(_, value)| value).unwrap())
        }
    }

    #[derive(Debug)]
    enum EventType {
        Begins(GuardID),
        FallsAsleep,
        WakesUp,
    }

    #[derive(Debug)]
    enum EventParseError {}

    impl FromStr for EventType {
        type Err = EventParseError;

        fn from_str(s: &str) -> Result<Self, EventParseError> {
            lazy_static! {
                static ref EventRE: Regex = Regex::new(r"^Guard #(\d+) begins shift$").unwrap();
            }
            if s.contains("falls asleep") {
                Ok(EventType::FallsAsleep)
            } else if s.contains("wakes up") {
                Ok(EventType::WakesUp)
            } else {
                let captures = EventRE.captures(s).unwrap();
                let id = (&captures[1]).parse().unwrap();
                Ok(EventType::Begins(id))
            }
        }
    }

    fn parse_event(input: String) -> (DateTime<Utc>, EventType) {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
            ^\[
            (?P<datetime>\d{4}-\d{2}-\d{2}\s{1}\d{2}:\d{2})
             \]
             \s{1}
            (?P<event>.*)$"
            )
            .unwrap();
        }

        let captures = RE.captures(&input).unwrap();

        let datetime = Utc
            .datetime_from_str(&captures["datetime"], "%Y-%m-%d %H:%M")
            .unwrap();
        let event = captures["event"].parse().unwrap();

        (datetime, event)
    }

    fn build_sleep_table(inputs: Vec<(DateTime<Utc>, EventType)>) -> HashMap<u16, Guard> {
        // note: we rely on `inputs` being sorted by date for the following to work
        let mut table = HashMap::<u16, Guard>::new();
        let mut current_guard: Option<Guard> = None;
        let mut current_sleep: Option<DateTime<Utc>> = None;
        for (datetime, event) in inputs {
            match event {
                EventType::Begins(id) => {
                    if let Some(current_guard) = current_guard.as_ref() {
                        let current_id = current_guard.id;
                        let sleeps = current_guard.minutes_asleep.len();
                        match table.entry(current_id) {
                            Entry::Occupied(mut entry) => {
                                let guard = entry.get_mut();
                                guard.add_sleep_record(&current_guard.minutes_asleep);
                            }
                            Entry::Vacant(entry) => {
                                entry.insert(current_guard.clone());
                            }
                        }
                    }
                    current_guard = Some(Guard::new(id, vec![]));
                }
                EventType::FallsAsleep => {
                    current_sleep = Some(datetime);
                }
                EventType::WakesUp => match current_guard.as_mut() {
                    Some(guard) => {
                        let span = (current_sleep.unwrap(), datetime);
                        guard.add_sleep(span)
                    }
                    None => {}
                },
            }
        }
        // get trailing event, sorry for copypasta
        if let Some(current_guard) = current_guard.as_ref() {
            let current_id = current_guard.id;
            let sleeps = current_guard.minutes_asleep.len();
            match table.entry(current_id) {
                Entry::Occupied(mut entry) => {
                    let guard = entry.get_mut();
                    guard.add_sleep_record(&current_guard.minutes_asleep);
                }
                Entry::Vacant(entry) => {
                    entry.insert(current_guard.clone());
                }
            }
        }
        table
    }

    #[test]
    fn can_find_guard_with_most_sleep() {
        let filename = "input/day_4.txt";
        let mut inputs = load_and_parse_input(filename, parse_event).unwrap();
        inputs.sort_by_key(|(d, _)| *d);

        let table = build_sleep_table(inputs);

        let (_, sleepiest_guard) = table
            .iter()
            .max_by_key(|(_, guard)| guard.total_minutes_asleep())
            .unwrap();

        let answer = sleepiest_guard.sleepiest_minute() * sleepiest_guard.id as usize;
        assert_eq!(answer, 26281);
    }

    #[test]
    fn can_find_most_frequent_minute() {
        let filename = "input/day_4.txt";
        let mut inputs = load_and_parse_input(filename, parse_event).unwrap();
        inputs.sort_by_key(|(d, _)| *d);

        let table = build_sleep_table(inputs);
        let result = table
            .iter()
            .filter_map(|(id, guard)| {
                guard
                    .most_frequent_minute_asleep()
                    .map(|(most_frequent_minute, count)| {
                        (*id as usize, most_frequent_minute, count)
                    })
            })
            .max_by_key(|&(_, _, count)| count)
            .unwrap();
        let answer = result.0 * result.1;
        assert_eq!(answer, 73001);
    }
}
