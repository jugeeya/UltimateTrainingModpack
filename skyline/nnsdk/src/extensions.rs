use super::*;

impl TimeSpan {
    pub fn nano(nanoseconds: u64) -> Self {
        TimeSpan {
            nanoseconds
        }
    }
    
    pub fn milli(milliseconds: u64) -> Self {
        TimeSpan {
            nanoseconds: milliseconds * 1000000
        }
    }

    pub fn secs(seconds: u64) -> Self {
        TimeSpan {
            nanoseconds: seconds * 1000000000u64
        }
    }

    pub fn minutes(minutes: u64) -> Self {
        TimeSpan::secs(minutes * 60)
    }

    pub fn hours(hours: u64) -> Self {
        TimeSpan::minutes(hours * 60)
    }
}
