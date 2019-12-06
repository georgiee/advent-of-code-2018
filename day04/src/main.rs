extern crate chrono;

use regex::Regex;
use chrono::{NaiveDateTime, Timelike, Duration};
use std::collections::HashMap;
use std::fmt;
use std::ops::Add;
use std::fs::read_to_string;

const PROBE:&str = "\
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
";

#[derive(Debug)]
enum SecurityEvent {
    Sleep,
    Wakeup,
    Start {
        guard: String
    }
}

#[derive(Debug)]
struct SecurityAction {
    date: NaiveDateTime,
    event: SecurityEvent
}
struct Guard {
    id: u32,
    minutes: [u32; 60]
}
impl fmt::Display for Guard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "id: {}:\n{:?}, \nsleeping duration: {} \nthe minute most asleep: {} @ min {}",
                self.id, &self.minutes[..], self.total_asleep(), self.most_asleep().1, self.most_asleep().0)
    }
}
impl Guard {
    fn new(id: &String) -> Guard {
        let (_, value) = id.split_at(1);
        let value = value.parse().unwrap_or(0);

        Guard {
            id: value,
            minutes: [0;60]
        }
    }
    fn sleep(&mut self, from: usize, duration: usize) {
        let to = from + duration;
        let update = &mut self.minutes[from..to];
        for element in update.iter_mut() {
            *element += 1;
        }
    }
    fn total_asleep(&self) -> u32 {
        return self.minutes.iter().sum();
    }

    fn most_asleep(&self) -> (u32, u32) {
        let max = self.minutes.iter().max().unwrap_or(&0);
        let index = self.minutes.iter().position(|&value| value == *max).unwrap_or(0);
        return (index as u32, *max as u32);
    }

}
fn main() {
    let file_content = read_to_string("input.txt")
        .expect("Error reading file");

    part01(&file_content);
    part02(&file_content);
}

fn part02(input:&str) {
    let actions = parse_events(input);
    let guard_map = generate_guard_map(actions);

    let most_sleeping_guard_by_most_asleep = guard_map.values()
        .max_by(|&a, &b| a.most_asleep().1.cmp(&b.most_asleep().1))
        .unwrap();
    let result_by_most_asleep = most_sleeping_guard_by_most_asleep.id * most_sleeping_guard_by_most_asleep.most_asleep().0;
    println!("\nmost_sleeping_guard2 {} -> result {}", most_sleeping_guard_by_most_asleep, result_by_most_asleep);
}

fn part01(input:&str) {
    let actions = parse_events(input);
    let guard_map = generate_guard_map(actions);

    let most_sleeping_guard = guard_map.values()
            .max_by(|&a, &b| a.total_asleep().cmp(&b.total_asleep()))
            .unwrap();
    let result = most_sleeping_guard.id * most_sleeping_guard.most_asleep().0;
    println!("\nmost_sleeping_guard1 {} -> result: {}", most_sleeping_guard, result);
}

fn generate_guard_map(actions:Vec<SecurityAction>) -> HashMap<String, Guard> {
    let mut guard_map:HashMap<String, Guard> = HashMap::new();

    let mut last_sleep_minute = 0;
    let mut current_guard_id:&str = "na";

    for action in actions.iter() {
        match &action.event {
            SecurityEvent::Start { guard } => {
                current_guard_id = &guard;

                guard_map
                    .entry(String::from(guard))
                    .or_insert(Guard::new(guard));
                last_sleep_minute = 0;
            }
            SecurityEvent::Sleep => {
                last_sleep_minute = action.date.time().minute();
            }
            SecurityEvent::Wakeup => {
                let duration = action.date.time().minute() - last_sleep_minute;
                let guard = guard_map.get_mut(current_guard_id).unwrap();
                guard.sleep(last_sleep_minute as usize, duration as usize);
            }
            _ => {}
        }
    }

    return guard_map;
}

fn parse_events(input: &str) -> Vec<SecurityAction> {
    let mut actions:Vec<SecurityAction> = input.lines()
        .map(|line| parse_action(&line))
        .collect();
    actions.sort_by(|a,b|a.date.cmp(&b.date));

    return actions;
}

fn parse_action(line:&str) -> SecurityAction {
    let re = Regex::new(r"^\[(?P<date>.*)\] (?P<action>.*)$").unwrap();
    let captures= re.captures(line).unwrap();

    let date = &captures["date"];
    let mut datetime = NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M").unwrap();
    if datetime.hour() == 23 {
        let diff = 60 - datetime.minute() as i64;
        datetime = datetime + Duration::seconds(diff * 60);
//        println!("fix datetime {} {}", datetime, diff);

    }

    let event = parse_event(&captures["action"]);

    let action = SecurityAction {
        date: datetime,
        event: event.unwrap()
    };

    return action;

}

fn parse_event(action:&str) -> Option<SecurityEvent> {
    let match_guard: Regex = Regex::new(r"(?P<id>#\d{0,}) begins shift").unwrap();
    let match_sleep: Regex = Regex::new(r"falls asleep").unwrap();
    let match_wakeup: Regex = Regex::new(r"wakes up").unwrap();


    if match_sleep.is_match(action) {
        return Some(SecurityEvent::Sleep);
    }else if match_wakeup.is_match(action) {
        return Some(SecurityEvent::Wakeup);
    }else if match_guard.is_match(action) {
        let caps = match_guard.captures(action).unwrap();
        return Some(SecurityEvent::Start {guard: String::from(&caps["id"])});
    }

    return None;
}
