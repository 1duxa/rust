#[allow(dead_code)]

pub mod clock{
use std::fmt::Display;

#[derive(Debug)]
pub struct Clock{
    hours:i32,
    minutes:i32
}

impl Clock {
    pub fn new(mut hours: i32, mut minutes: i32) -> Self {
        hours += minutes.div_euclid(60);
        minutes = minutes.rem_euclid(60);
        hours = hours.rem_euclid(24);

        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let total_minutes = self.minutes + minutes;
        Clock::new(self.hours, total_minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
}