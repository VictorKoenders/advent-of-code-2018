use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let file = env::args().nth(1).unwrap();
    let mut str = String::new();
    File::open(&file).unwrap().read_to_string(&mut str).unwrap();

    let mut guard_changes = Vec::new();
    for line in str.lines().filter(|s| !s.is_empty()) {
        let change: GuardChange = line
            .parse()
            .unwrap_or_else(|e| panic!("Could not parse line {:?}: {:?}", line, e));
        guard_changes.push(change);
    }
    guard_changes.sort();

    let mut sleep_map = HashMap::new();
    let mut last_sleep_change = None;
    for change in guard_changes {
        match (change.data.clone(), &mut last_sleep_change) {
            (GuardChangeType::BeginShift { .. }, _) => {
                let values = vec![true; 60];
                sleep_map.insert(change.clone(), values);
                last_sleep_change = Some(change);
            }
            (GuardChangeType::FallAsleep, Some(current_shift)) => {
                let entry = sleep_map
                    .get_mut(&current_shift)
                    .expect("Could not get entry for FallAsleep");
                for i in change.minute as usize..entry.len() {
                    entry[i] = false;
                }
            }
            (GuardChangeType::WakeUp, Some(current_shift)) => {
                let entry = sleep_map
                    .get_mut(&current_shift)
                    .expect("Could not get entry for FallAsleep");
                for i in change.minute as usize..entry.len() {
                    entry[i] = true;
                }
            }
            (x, y) => panic!("Unreachable state: {:?} {:?}", x, y),
        }
    }

    println!("Date ID    Minute");
    println!("           000000000011111111112222222222333333333344444444445555555555");
    println!("           012345678901234567890123456789012345678901234567890123456789");
    let mut keys = sleep_map.keys().collect::<Vec<_>>();
    keys.sort();
    let mut guard_sleep_pattern: HashMap<u16, Vec<u16>> = HashMap::new();
    let mut minute_count = vec![0; 60];

    for entry in keys {
        let guard_id = match entry.data {
                GuardChangeType::BeginShift { id } => id,
                _ => panic!(),
            };

        let values = &sleep_map[&entry];
        print!(
            "{:02}-{:02} #{:04} ",
            entry.month,
            entry.day,
            match entry.data {
                GuardChangeType::BeginShift { id } => id,
                _ => panic!(),
            }
        );

        let map = guard_sleep_pattern.entry(guard_id).or_insert_with(|| vec![0;60]);
        for (index, item) in values.iter().enumerate() {
            if *item {
                print!(".");
            } else {
                map[index] += 1;
                minute_count[index] += 1;
                print!("#");
            }
        }

        println!("");
    }

    let mut highest_diff = (0, 0, 0);

    for i in 0..60 {
        let mut guard_sleep_count_by_minute = Vec::new();
        for (id, pattern) in &guard_sleep_pattern {
            guard_sleep_count_by_minute.push((id, pattern[i]));
        }
        guard_sleep_count_by_minute.sort_by_key(|x| x.1);

        let last = guard_sleep_count_by_minute[guard_sleep_count_by_minute.len() - 1];
        let mut total_sleep_time_of_guard = 0;
        for minute in &guard_sleep_pattern[last.0] {
            total_sleep_time_of_guard += minute;
        }
        println!("Diff on minute {} is {} for guard {} (total: {})", i, last.1, last.0, total_sleep_time_of_guard);

        if last.1 > highest_diff.0 {
            highest_diff = (last.1, *last.0, i);
        }
    }
    println!("Highest diff happened on minute {} for guard {} ({} times)", highest_diff.2, highest_diff.1, highest_diff.0);
    println!("Answer is {}", highest_diff.2 * highest_diff.1 as usize);
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct GuardChange {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub minute: u16,
    pub data: GuardChangeType,
}

fn get_days_in_month(month: u16) -> u16 {
    match month {
        1 => 31,
        2 => 28,
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 31,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => unreachable!()
    }
}

impl std::str::FromStr for GuardChange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let year: u16 = s[1..5].parse().unwrap();
        let mut month: u16 = s[6..8].parse().unwrap();
        let mut day: u16 = s[9..11].parse().unwrap();
        let mut hour: u16 = s[12..14].parse().unwrap();
        let mut minute: u16 = s[15..17].parse().unwrap();

        if hour == 23 {
            hour = 0;
            minute = 0;
            day += 1;
            if day > get_days_in_month(month) {
                day = 1;
                month += 1;
            }
        }

        let remaining = &s[19..];
        let data = remaining.parse().unwrap();

        Ok(GuardChange {
            year,
            month,
            day,
            hour,
            minute,
            data,
        })
    }
}

impl PartialOrd for GuardChange {
    fn partial_cmp(&self, other: &GuardChange) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}
impl Ord for GuardChange {
    fn cmp(&self, other: &GuardChange) -> Ordering {
        let values = &[
            (self.year, other.year),
            (self.month, other.month),
            (self.day, other.day),
            (self.hour, other.hour),
            (self.minute, other.minute),
        ];
        for (left, right) in values {
            match left.cmp(right) {
                Ordering::Equal => continue,
                x => return x,
            }
        }
        match (&self.data, &other.data) {
            (GuardChangeType::BeginShift { .. }, _) => Ordering::Less,
            (_, GuardChangeType::BeginShift { .. }) => Ordering::Greater,
            _ => {
                panic!("GuardChange eq: {:?} {:?}", self, other);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum GuardChangeType {
    BeginShift { id: u16 },
    FallAsleep,
    WakeUp,
}

impl std::str::FromStr for GuardChangeType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "wakes up" => GuardChangeType::WakeUp,
            "falls asleep" => GuardChangeType::FallAsleep,
            _ => {
                let s = &s[7..];
                let index = s.bytes().position(|b| b == b' ').unwrap();
                let s = &s[..index];
                GuardChangeType::BeginShift {
                    id: s.parse().unwrap(),
                }
            }
        })
    }
}
