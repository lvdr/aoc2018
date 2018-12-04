use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;

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
        let mut comp = self.year.cmp(&other.year);
        if comp != Ordering::Equal {
            return comp;
        }
        comp = self.month.cmp(&other.month);
        if comp != Ordering::Equal {
            return comp;
        }
        comp = self.day.cmp(&other.day);
        if comp != Ordering::Equal {
            return comp;
        }
        comp = self.hour.cmp(&other.hour);
        if comp != Ordering::Equal {
            return comp;
        }
        self.minute.cmp(&other.minute)
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

fn main() {
    let mut input = String::new();
    let mut f = File::open("input").expect("Failed to open input.");
    f.read_to_string(&mut input).expect("Failed to read input.");

    let lines : Vec<&str> = input.trim()
                                .split("\n")
                                .map(|x| x.trim())
                                .collect();

    let mut log = BTreeSet::new();

    for line in lines {
        log.insert(parse_log_line(line));
    }

    let mut guard_naps = HashMap::new();

    let mut on_duty: u32 = 0;
    let mut sleep_start: Date = Date {year: 0, month: 0, day: 0, hour: 0, minute: 0};
    for entry in log {
        match entry.event {
            LogEvent::FallAsleep => sleep_start = entry.timestamp,
            LogEvent::WakeUp => guard_naps.entry(on_duty)
                                          .or_insert(Vec::new())
                                          .push(TimeRange { start : sleep_start, end: entry.timestamp}),
            LogEvent::StartShift(guard) => on_duty = guard,
        }
    }

    let mut max_napper = 0;
    let mut max_naps = 0;
    let mut sleepiest_minute = 0;

    let mut most_reliable_napper = 0;
    let mut most_reliable_naps = 0;
    let mut most_reliable_minute = 0;

    let mut guard_naps_by_minute = HashMap::new();
    for (guard, naps) in guard_naps {
        guard_naps_by_minute.insert(guard, [0; 60]);

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

        let mut total_guard_naps = 0;
        let mut most_napped_minute = 0;
        let mut most_naps_in_minute = 0;
        for i in 0..60 {
            guard_naps_by_minute.get_mut(&guard).unwrap()[i as usize] += full_hours;
            total_guard_naps += guard_naps_by_minute[&guard][i as usize];
            if guard_naps_by_minute[&guard][i as usize] > most_naps_in_minute {
                most_napped_minute = i;
                most_naps_in_minute = guard_naps_by_minute[&guard][i as usize];
            }
        }

        if total_guard_naps > max_naps {
            max_naps = total_guard_naps;
            max_napper = guard;
            sleepiest_minute = most_napped_minute;
        }

        if most_naps_in_minute > most_reliable_naps {
            most_reliable_napper = guard;
            most_reliable_naps = most_naps_in_minute;
            most_reliable_minute = most_napped_minute;
        }
    }

    println!("Part 1: {}, Part 2: {}", max_napper*sleepiest_minute, most_reliable_napper*most_reliable_minute);
}

