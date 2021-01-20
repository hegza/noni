use chrono::Weekday;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::{time::Clock, Time};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TriggerRule {
    /// This task triggers only once, after which it is deactivated or deleted.
    OneShot(Time),
    /// This task can trigger repeatedly, possibly suspending subsequent triggers when first triggered.
    Repeat(RepeatRule),
    /// This task is never triggered.
    NoRule,
}

/// This task can trigger repeatedly, possibly suspending subsequent triggers when first triggered. It's also possible to block or delay triggering during specific time-spans.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RepeatRule {
    stacks: bool,
    time_rule: TimeRule,
    except_on: Vec<GregorianSpan>,
    delay_on: Vec<GregorianSpan>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TimeRule {
    /// Trigger after a specific time has passed from the previous completion (or triggering, if the event is stackable).
    #[serde(with = "humantime_serde")]
    FixedTimeSpan(Duration),
    /// Trigger at specific gregorian times of significance.
    Gregorian(GregorianRule),
}

/// Trigger at specific gregorian times of significance.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GregorianRule {
    Annual { month: u32, day: u32, clock: Clock },
    Monthly { day: u32, clock: Clock },
    Weekly { weekday: Weekday, clock: Clock },
    Daily(Clock),
}

/// Specific time-spans of gregorian significance.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum GregorianSpan {
    Weekend,
    /// All days except the weekend
    Weekday,
    Month(u8),
    Season(Season),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Season {
    /// December, January, February
    Winter,
    /// March, April, May
    Spring,
    /// June, July, August
    Summer,
    /// September, October, November
    Autumn,
}
