use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut hours = hours;
        let mut minutes = minutes;
        if minutes >= 60 {
            hours += minutes / 60;
            minutes %= 60;
        }
        if minutes < 0 {
            hours -= 1;
            minutes += 60;
        }
        if hours >= 24 {
            hours %= 24;
        }
        if hours < 0 {
            hours += 24;
        }
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours = if self.hours < 10 {
            format!("0{}", self.hours)
        } else {
            format!("{}", self.hours)
        };
        let minutes = if self.minutes < 10 {
            format!("0{}", self.minutes)
        } else {
            format!("{}", self.minutes)
        };
        write!(f, "{}:{}", hours, minutes)
    }
}

fn main() {
    // Test 1
    let clock = Clock::new(8, 0);
    assert_eq!(clock.to_string(), "08:00");

    // Test 2
    let clock = Clock::new(11, 9);
    assert_eq!(clock.to_string(), "11:09");

    // Test 3
    let clock = Clock::new(24, 0);
    assert_eq!(clock.to_string(), "00:00");
}