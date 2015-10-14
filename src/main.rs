extern crate chrono;
extern crate libc;

use chrono::Timelike;

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

type UTCTime = chrono::datetime::DateTime<chrono::offset::utc::UTC>;

struct Countdown {
    /// The power word used to end the countdown
    liftoff: String,
    /// When the countdown ends
    deadline: UTCTime,
    /// Final countdown
    countdown: i8
}

enum Mark {
    Minute(u32),
    Second(u32),
    Liftoff(String)
}

/// Default is: 3...2...1...Go!
impl Default for Countdown {
    fn default() -> Countdown {
        let t = chrono::offset::utc::UTC::now()
              + chrono::duration::Duration::minutes(2);
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
            let t = chrono::offset::utc::UTC::now();
            unsafe { libc::funcs::posix88::unistd::nanosleep(&req, &mut rem); }

            if t.timestamp() == self.deadline.timestamp() {
                return Some(Mark::Liftoff(self.liftoff.clone()));
            } else if t > self.deadline {
                return None
            } else if t.time().second() == self.deadline.time().second() &&
                      t.time().minute() < self.deadline.time().minute() {
                let m = self.deadline.time().minute() - t.time().minute();
                return Some(Mark::Minute(m));
            } else if self.deadline.minute() == t.time().minute() &&
                      self.deadline.time().second() > t.time().second() &&
                      self.deadline.time().second() - t.time().second() <= self.countdown as u32 {
                let s = self.deadline.time().second() - t.time().second();
                return Some(Mark::Second(s));
            }
        }
    }
}
