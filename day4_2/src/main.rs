#[macro_use] extern crate scan_fmt;

use std::fs::File;
use std::io::Read;
use std::cmp::Ordering;
use std::collections::HashMap;

enum Activity {
    Begin,
    Sleep,
    Wake,
}

struct Record {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    id: u32,
    activity: Activity,
}

impl Ord for Record {
    fn cmp(&self, other: &Record) -> Ordering {
        if self.year < other.year {
            Ordering::Less
        } else if self.year > other.year {
            Ordering::Greater
        } else if self.month < other.month {
            Ordering::Less
        } else if self.month > other.month {
            Ordering::Greater
        } else if self.day < other.day {
            Ordering::Less
        } else if self.day > other.day {
            Ordering::Greater
        } else if self.hour < other.hour {
            Ordering::Less
        } else if self.hour > other.hour {
            Ordering::Greater
        } else if self.minute < other.minute {
            Ordering::Less
        } else if self.minute > other.minute {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Record) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Record {
    fn eq(&self, other: &Record) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Record {}

fn build_record(id: u32, year: u32, month: u32, day: u32, hour: u32, minute: u32, activity: Activity) -> Record {
    Record {
        id: id,
        year: year,
        month: month,
        day: day,
        hour: hour,
        minute: minute,
        activity: activity,
    }
}

fn main() {
    let mut file = File::open("/home/jie/projects/rust/advent/day4_1/input")
        .expect("Error cannot open the file");
    let mut contents = String::new();
    let mut records = Vec::new();
    let mut all_sleep_minutes = HashMap::new();

    file.read_to_string(&mut contents)
        .expect("Error reading input file");

    for line in contents.lines() {
        //println!("{}", line);

        let (y, mon, d, h, min, i) = scan_fmt!(line, "[{}-{}-{} {}:{}] Guard #{} begins shift", u32, u32, u32, u32, u32, u32);
        match (y, mon, d, h, min, i) {
            (Some(year), Some(month), Some(day), Some(hour), Some(minute), Some(id)) => {
                let mut record = build_record(id, year, month, day, hour, minute, Activity::Begin);
                records.push(record);
                continue;
            }
            _ => (),
        }

        let (y, mon, d, h, min, a) = scan_fmt!(line, "[{}-{}-{} {}:{}] {/.*/}", u32, u32, u32, u32, u32, String);
        match (y, mon, d, h, min, a) {
            (Some(year), Some(month), Some(day), Some(hour), Some(minute), Some(act)) => {
                let mut record;
                if act == "falls asleep" {
                    record = build_record(0, year, month, day, hour, minute, Activity::Sleep);
                } else {
                    record = build_record(0, year, month, day, hour, minute, Activity::Wake);
                }
                records.push(record);
                continue;
            }
            _ => (),
        }

        println!("Bad record: {}", line);
    }

    records.sort();
/*
    for r in records {
        match r.activity {
            Activity::Begin => {
                println!("{} {} {} {}:{} #{} Begin", r.year, r.month, r.day, r.hour, r.minute, r.id);
            }

            Activity::Sleep => {
                println!("{} {} {} {}:{} Sleep", r.year, r.month, r.day, r.hour, r.minute);
            }

            Activity::Wake => {
                println!("{} {} {} {}:{} Wake", r.year, r.month, r.day, r.hour, r.minute);
            }
        }
    }
*/
/*
    let mut last_activity = Activity::Wake;
    for r in records {
        match r.activity {
            Activity::Begin => {
                match last_activity {
                    Activity::Sleep => { println!("{} {} {} {}:{} Begin", r.year, r.month, r.day, r.hour, r.minute); }
                    _ => (),
                }
            }
            _ => (),
        }
        last_activity = r.activity;
    }
*/
    let mut current_id = 0;
    let mut fall_asleep_minute = 0;
    for r in records {
        match r.activity {
            Activity::Begin => {
                current_id = r.id;
            }
            Activity::Sleep => {
                fall_asleep_minute = r.minute;
            }
            Activity::Wake => {
                let guard_sleep_minutes = all_sleep_minutes.entry(current_id).or_insert(vec![0; 60]);
                let start = fall_asleep_minute as usize;
                let end = r.minute as usize;
                for i in start..end {
                    guard_sleep_minutes[i] += 1;
                }
            }
        }
    }

    let mut laziest_id = 0;
    let mut laziest_minute = 0;
    let mut max_count = 0;
    for (id, sleep_minutes) in &all_sleep_minutes {
        for i in 0..60 {
            if sleep_minutes[i] > max_count {
                max_count = sleep_minutes[i];
                laziest_id = *id;
                laziest_minute = i as u32;
            }
        }
    }

    println!("{} x {} = {}", laziest_id, laziest_minute, laziest_id * laziest_minute);
}
