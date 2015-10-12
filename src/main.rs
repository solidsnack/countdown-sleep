extern crate chrono;

fn main() {
    // TODO: argument parsing
    let countdown = Countdown::default();
    println!("Countdown Initiated!");
    for mark in countdown {
        match mark {
            Minute(t) => {
                println!("T minus {} minutes and counting...",
                         /* round time difference to minutes */);
            },
            Second(t) => {
                println!("{}...", /* round time difference to seconds */);
            },
            Liftoff(s) => {
                println!(s);
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
    fn next(&mut self) -> Option<Mark> {
        // thread.sleep()
        // yield next thing
    }
}
