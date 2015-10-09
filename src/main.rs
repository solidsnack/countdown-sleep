extern crate chrono;

fn main() {
    let s = Countdown::default();
    println!("Countdown Initiated!");
}

struct Countdown {
    /// The power word used to end the countdown
    liftoff: String,
    /// When the countdown ends
    deadline: chrono::datetime::DateTime<chrono::offset::utc::UTC>,
    /// Final countdown
    countdown: i8
}

/// Default is: 3...2...1...Go!
impl Default for Countdown {
    fn default() -> Countdown {
        let t = chrono::offset::utc::UTC::now()
              + chrono::duration::Duration::seconds(3);
        Countdown { liftoff: "Go!".to_string(), deadline: t, countdown: 3 }
    }
}
