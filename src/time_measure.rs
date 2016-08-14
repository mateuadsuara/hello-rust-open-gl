extern crate time;
use self::time::{Timespec, Duration};

pub struct TimeMeasure {
    start_time: Timespec,
}

pub struct TimeDuration {
    time: Duration
}

impl TimeMeasure {
    pub fn start() -> TimeMeasure {
        TimeMeasure {
            start_time: time::get_time()
        }
    }

    pub fn end(&self) -> TimeDuration {
        TimeDuration {
            time: time::get_time() - self.start_time
        }
    }
}

impl TimeDuration {
    pub fn period(&self, period_time_in_seconds: f32) -> f32 {
        (((self.time.num_milliseconds() as f32) / 1000.0) % period_time_in_seconds ) / period_time_in_seconds
    }

    pub fn current_second(&self) -> i64 {
        self.time.num_seconds()
    }
}

