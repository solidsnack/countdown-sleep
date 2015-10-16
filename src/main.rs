extern crate chrono;
extern crate libc;

use std::cmp::Ordering::*;

use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;
use chrono::duration::Duration;


fn main() {
    // TODO: argument parsing
    let countdown = Countdown::default();
    println!("Countdown Initiated!");
    for mark in countdown {
        match mark {
            Mark::Minute(t) => {
                println!("T minus {} minutes and counting...", t);
            },
            Mark::Second(t) => {
                println!("{}...", t);
            },
            Mark::Liftoff(s) => {
                println!("{}", s);
            }
        }
    }
}

struct Countdown {
    /// The power word used to end the countdown
    liftoff: String,
    /// When the countdown ends
    deadline: DateTime<UTC>,
    /// Final countdown
    countdown: u8
}

enum Mark {
    Minute(u32),
    Second(u32),
    Liftoff(String)
}

use Mark::*;

/// Default is: 3...2...1...Go!
impl Default for Countdown {
    fn default() -> Countdown {
        let t = UTC::now() + Duration::minutes(2) + Duration::milliseconds(10);
        Countdown { liftoff: "Go!".to_string(), deadline: t, countdown: 3 }
    }
}

impl Iterator for Countdown {
    type Item = Mark;

    fn next(&mut self) -> Option<Mark> {
        type SecondsAndNanos = libc::types::os::common::posix01::timespec;

        let req = SecondsAndNanos{tv_sec: 1, tv_nsec: 0};
        let mut rem = SecondsAndNanos{tv_sec: 0, tv_nsec: 0};

        loop {
            let d = self.deadline - UTC::now();
            unsafe { libc::funcs::posix88::unistd::nanosleep(&req, &mut rem); }

            match (d.num_minutes().cmp(&1),
                   d.num_seconds().cmp(&(self.countdown as i64)),
                   d.num_seconds() % 60,
                   d.num_seconds().cmp(&1),
                   d.cmp(&Duration::seconds(0))) {
                (Greater, _, 0, _, _) | (Equal, _, 0, _, _) =>
                    return Some(Minute(d.num_minutes() as u32)),
                (_, Equal, _, _, _) | (_, Less, _, Greater, _) |
                (_, Less, _, Equal, _) =>
                    return Some(Second(d.num_seconds() as u32)),
                (_, _, _, _, Equal) | (_, _, _, Less, Greater) =>
                    return Some(Liftoff(self.liftoff.clone())),
                (_, _, _, _, Less) => return None,
                (_, _, _, _, _)    => {}
            }
        }
    }
}
