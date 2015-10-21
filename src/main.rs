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
            Minute(t) => {
                println!("T minus {} minutes and counting...", t);
            },
            Second(t) => {
                println!("{}...", t);
            },
            Liftoff(s) => {
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

#[derive(Debug)]
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
        let (goal, diff) = self.step();
        nanosleep(&diff);
        return if diff < Duration::zero() { None } else { Some(goal) };
    }
}

impl Countdown {
    fn step(&mut self) -> (Mark, Duration) {
        let diff = self.deadline - UTC::now();
        let countdown = self.countdown as i64;
        let (cmp1m, cmpct, cmp1s) = (diff.num_minutes().cmp(&1),
                                     diff.num_seconds().cmp(&countdown),
                                     diff.num_seconds().cmp(&1));
        let (goal, dur) = match (cmp1m, cmpct, cmp1s) {
            (Greater, Greater, _) | (Equal, Greater, _) =>
                (Minute(diff.num_minutes() as u32),
                 diff - Duration::minutes(diff.num_minutes())),
            (_, Greater, _) =>
                (Second(self.countdown as u32),
                 diff - Duration::seconds(countdown)),
            (_, _, Greater) | (_, _, Equal) =>
                (Second(diff.num_seconds() as u32),
                 diff - Duration::seconds(diff.num_seconds())),
            (_, _, _) => (Liftoff(self.liftoff.clone()), diff)
        };
        return (goal, dur);
    }
}

/// Call nanosleep to sleep the given duration, handling interruptions.
fn nanosleep(dur: &Duration) {
    use libc::types::os::common::posix01::timespec as SecondsAndNanos;

    let seconds = dur.num_seconds();
    let nanos = (*dur - Duration::seconds(seconds)).num_nanoseconds()
                                                   .expect("Catastrophe!");

    let mut req = SecondsAndNanos{tv_sec: seconds, tv_nsec: nanos};
    let mut rem = SecondsAndNanos{tv_sec: 0, tv_nsec: 0};

    unsafe { libc::funcs::posix88::unistd::nanosleep(&req, &mut rem); }
    while rem.tv_sec > 0 || rem.tv_nsec > 0 {
        req.tv_sec = rem.tv_sec;
        req.tv_nsec = rem.tv_nsec;
        unsafe { libc::funcs::posix88::unistd::nanosleep(&req, &mut rem); }
    }
}
