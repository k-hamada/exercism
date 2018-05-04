use std::fmt;

#[derive(Debug)]
pub struct Clock {
    minutes: i32
}

impl Clock {
    pub fn new(h: i32, m: i32) -> Clock {
        Clock { minutes: (h * 60 + m) % -1440 + 1440 }
    }

    pub fn add_minutes(&self, minute: i32) -> Clock {
        Clock::new(0, self.minutes + minute)
    }

    pub fn get_hour(&self) -> i32 {
        self.minutes / 60 % 24
    }

    pub fn get_minute(&self) -> i32 {
        self.minutes % 60
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.get_hour(), self.get_minute())
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.get_hour() == other.get_hour() &&
        self.get_minute() == other.get_minute()
    }
}
