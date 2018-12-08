use std::cmp::Ordering;
use std::cmp;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordered() {
        let input = String::from("[1518-11-01 00:00] Guard #10 begins shift\n\
                                  [1518-11-01 00:05] falls asleep\n\
                                  [1518-11-01 00:25] wakes up\n\
                                  [1518-11-01 00:30] falls asleep\n\
                                  [1518-11-01 00:55] wakes up\n\
                                  [1518-11-01 23:58] Guard #99 begins shift\n\
                                  [1518-11-02 00:40] falls asleep\n\
                                  [1518-11-02 00:50] wakes up\n\
                                  [1518-11-03 00:05] Guard #10 begins shift\n\
                                  [1518-11-03 00:24] falls asleep\n\
                                  [1518-11-03 00:29] wakes up\n\
                                  [1518-11-04 00:02] Guard #99 begins shift\n\
                                  [1518-11-04 00:36] falls asleep\n\
                                  [1518-11-04 00:46] wakes up\n\
                                  [1518-11-05 00:03] Guard #99 begins shift\n\
                                  [1518-11-05 00:45] falls asleep\n\
                                  [1518-11-05 00:55] wakes up");
        let log = parse_input(input);
        let guard_naps = parse_log(log);
        let guard_naps_by_minute = naps_by_minute(guard_naps);
        assert_eq!(get_sleepiest_guard(&guard_naps_by_minute), (10, 24));
        assert_eq!(get_reliable_guard(&guard_naps_by_minute), (99, 45));
    }

    #[test]
    fn test_unordered() {
        let input = String::from("[1518-11-04 00:36] falls asleep\n\
                                  [1518-11-04 00:46] wakes up\n\
                                  [1518-11-01 00:30] falls asleep\n\
                                  [1518-11-01 00:00] Guard #10 begins shift\n\
                                  [1518-11-01 00:05] falls asleep\n\
                                  [1518-11-01 00:25] wakes up\n\
                                  [1518-11-01 00:55] wakes up\n\
                                  [1518-11-03 00:24] falls asleep\n\
                                  [1518-11-03 00:29] wakes up\n\
                                  [1518-11-05 00:03] Guard #99 begins shift\n\
                                  [1518-11-01 23:58] Guard #99 begins shift\n\
                                  [1518-11-02 00:40] falls asleep\n\
                                  [1518-11-02 00:50] wakes up\n\
                                  [1518-11-03 00:05] Guard #10 begins shift\n\
                                  [1518-11-04 00:02] Guard #99 begins shift\n\
                                  [1518-11-05 00:45] falls asleep\n\
                                  [1518-11-05 00:55] wakes up");
        let log = parse_input(input);
        let guard_naps = parse_log(log);
        let guard_naps_by_minute = naps_by_minute(guard_naps);
        assert_eq!(get_sleepiest_guard(&guard_naps_by_minute), (10, 24));
        assert_eq!(get_reliable_guard(&guard_naps_by_minute), (99, 45));
    }
}


#[derive(Debug, Copy, Clone, Eq)]
struct Date {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

impl Ord for Date {
    fn cmp(&self, other: &Date) -> Ordering {
        self.year.cmp(&other.year)
            .then(self.month.cmp(&other.month))
            .then(self.day.cmp(&other.day))
            .then(self.hour.cmp(&other.hour))
            .then(self.minute.cmp(&other.minute))
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Date) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Date) -> bool {
        self.year == other.year &&
            self.month == other.month &&
            self.day == other.day &&
            self.hour == other.hour &&
            self.minute == other.minute
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum LogEvent {
    FallAsleep,
    WakeUp,
    StartShift(u32),
}

#[derive(Debug, Copy, Clone, Eq)]
struct LogEntry {
    timestamp: Date,
    event: LogEvent,
}

impl Ord for LogEntry {
    fn cmp(&self, other: &LogEntry) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl PartialOrd for LogEntry {
    fn partial_cmp(&self, other: &LogEntry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for LogEntry {
    fn eq(&self, other: &LogEntry) -> bool {
        self.timestamp == other.timestamp
    }
}

#[derive(Debug, Copy, Clone)]
struct TimeRange {
    start: Date,
    end: Date,
}

fn parse_log_line(line : &str) -> LogEntry {
    let year = line[1..5].parse::<u32>()
                         .expect("Failed to parse year");
    let month = line[6..8].parse::<u32>()
                          .expect("Failed to parse month");
    let day = line[9..11].parse::<u32>()
                         .expect("Failed to parse day");
    let hour = line[12..14].parse::<u32>()
                           .expect("Failed to parse hour");
    let minute = line[15..17].parse::<u32>()
                             .expect("Failed to parse minute");

    let event_ln = &line[19..];
    let event;
    if event_ln.starts_with("falls asleep") {
        event = LogEvent::FallAsleep;
    } else if event_ln.starts_with("wakes up") {
        event = LogEvent::WakeUp;
    } else {
        let guard = event_ln.split("#").nth(1).unwrap()
                         .split(" ").nth(0).unwrap()
                         .parse::<u32>()
                         .expect("Failed to parse guard ID");
        event = LogEvent::StartShift(guard);
    }
    let timestamp = Date { year: year, month: month, day: day,
                           hour: hour, minute: minute };
    LogEntry { event: event, timestamp: timestamp }
}

fn parse_input(input: String) -> BTreeSet<LogEntry> {
    let lines : Vec<&str> = input.trim()
                                 .split("\n")
                                 .map(|x| x.trim())
                                 .collect();
    let mut log = BTreeSet::new();
    for line in lines {
        log.insert(parse_log_line(line));
    }
    log
}

fn parse_log(log: BTreeSet<LogEntry>) -> HashMap<u32, Vec<TimeRange>> {
    let mut guard_naps = HashMap::new();
    let mut on_duty = None;
    let mut sleep_start = None;
    for entry in log {
        match entry.event {
            LogEvent::FallAsleep => sleep_start = Some(entry.timestamp),
            LogEvent::WakeUp =>
                guard_naps.entry(on_duty.expect("No guard on duty at wake up"))
                          .or_insert(Vec::new())
                          .push(TimeRange {start : sleep_start
                                .expect("Woke up from nonexistent nap"),
                                end: entry.timestamp}),
            LogEvent::StartShift(guard) => on_duty = Some(guard)
        }
    }
    guard_naps
}

fn naps_by_minute(guard_naps: HashMap<u32, Vec<TimeRange>>)
    -> HashMap<u32, Vec<i32>> {
    let mut guard_naps_by_minute = HashMap::new();
    for (guard, naps) in guard_naps {
        guard_naps_by_minute.insert(guard, vec![0 as i32; 60]);

        let mut full_hours = 0;
        for nap in naps {
            if nap.start.minute < nap.end.minute {
                for i in nap.start.minute..nap.end.minute {
                    guard_naps_by_minute.get_mut(&guard).unwrap()[i as usize] += 1;
                }
            } else {
                for i in nap.start.minute..nap.end.minute {
                    guard_naps_by_minute.get_mut(&guard).unwrap()[i as usize] -= 1;
                }
            }
            full_hours += nap.end.hour - nap.start.hour;
        }

        for i in 0..60 {
            guard_naps_by_minute.get_mut(&guard).unwrap()[i as usize]
                += full_hours as i32;
        }
    }
    guard_naps_by_minute
}

fn get_sleepiest_guard(npm: &HashMap<u32, Vec<i32>>) -> (u32, u32) {
    let mut max_naps = (0, 0);
    for (guard, minutes) in npm {
        let total_naps = minutes.iter().sum();
        max_naps = cmp::max(max_naps, (total_naps, *guard))
    }

    let sleepiest_guard = max_naps.1;
    let mut sleepiest_minute = (0, 0);
    for i in 0..60 {
        sleepiest_minute = cmp::max((npm[&sleepiest_guard][i], i),
                                    sleepiest_minute);
    }
    (sleepiest_guard, sleepiest_minute.1 as u32)
}

fn get_reliable_guard(npm: &HashMap<u32, Vec<i32>>) -> (u32, u32) {
    let mut sleepiest_guard = (0, 0, 0);
    for (guard, minutes) in npm {
        let mut sleepiest_minute = (0, 0);
        for i in 0..60 {
            sleepiest_minute = cmp::max((minutes[i], i), sleepiest_minute);
        }
        let this_guard = (sleepiest_minute.0, sleepiest_minute.1 as u32, *guard);
        sleepiest_guard = cmp::max(sleepiest_guard, this_guard);
    }
    (sleepiest_guard.2, sleepiest_guard.1)
}

fn main() {
    let mut input = String::new();
    let mut f = File::open("input").expect("Failed to open input.");
    f.read_to_string(&mut input).expect("Failed to read input.");

    let log = parse_input(input);
    let guard_naps = parse_log(log);

    let guard_naps_by_minute = naps_by_minute(guard_naps);
    let (sleepiest_guard, sleepiest_minute)
        = get_sleepiest_guard(&guard_naps_by_minute);
    let (reliable_guard, reliable_minute)
        = get_reliable_guard(&guard_naps_by_minute);

    println!("Part 1: {}, Part 2: {}", sleepiest_guard*sleepiest_minute,
                                       reliable_guard*reliable_minute);
}

