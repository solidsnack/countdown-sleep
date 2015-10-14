extern crate chrono;
extern crate libc;

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
    Minute(UTCTime),
    Second(UTCTime),
    Liftoff(String)
}

/// Default is: 3...2...1...Go!
impl Default for Countdown {
    fn default() -> Countdown {
        let t = chrono::offset::utc::UTC::now()
              + chrono::duration::Duration::seconds(3);
        Countdown { liftoff: "Go!".to_string(), deadline: t, countdown: 3 }
    }
}

impl Iterator for Countdown {
    type Item = Mark;

    fn next(&mut self) -> Option<Mark> {
        type SecondsAndNanos = libc::types::os::common::posix01::timespec;

        let req = SecondsAndNanos{tv_sec: 0, tv_nsec: 0};
        let mut rem = SecondsAndNanos{tv_sec: 0, tv_nsec: 0};

        unsafe { libc::funcs::posix88::unistd::nanosleep(&req, &mut rem); }
        Some(Mark::Liftoff(self.liftoff.clone()))
    }
}
