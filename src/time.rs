use chrono::Timelike;

pub trait AndTime: Timelike {
    fn and_time(hours: u32, minutes: u32, seconds: u32) {
    }
}

impl <T: Timelike> AndTime for T {}