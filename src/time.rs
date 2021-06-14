use chrono::{DateTime, Local, NaiveTime, Timelike};
use serde::{Deserialize, Serialize, Serializer};

pub type LocalTime = DateTime<Local>;

/// The time type for the crate. Implements custom serialization among other things. Full timestamp still exists at runtime.
#[derive(Clone, Debug, Deserialize)]
pub struct Time(pub LocalTime);

/// Just the time without the date component
#[derive(Clone, Debug, Deserialize)]
pub struct Clock(pub NaiveTime);

impl Serialize for Time {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Time with truncated nanoseconds
        s.serialize_newtype_struct("Time", &self.0.with_nanosecond(0).unwrap_or(self.0))
    }
}

impl Serialize for Clock {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Time with truncated nanoseconds
        s.serialize_newtype_struct("Clock", &self.0.with_nanosecond(0).unwrap_or(self.0))
    }
}
