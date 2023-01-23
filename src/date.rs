use core::fmt;

pub struct DateTime {
    pub sec: i32,
    pub min: i32,
    pub hour: i32,
}

impl DateTime {
    pub fn new() -> Self {
        Self {
            sec: 0,
            min: 0,
            hour: 0
        }
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.hour, self.min, self.sec)
    }
}

pub fn convert_epoch_to_date(ts: i64, tm: &mut DateTime) {
    let day_clock = ts % 86400;
    tm.sec = (day_clock % 60) as i32;
    tm.min = ((day_clock % 3600) / 60) as i32;
    tm.hour = (day_clock / 3600) as i32;
}
