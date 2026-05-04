#![allow(warnings)]

use chrono::{DateTime, Local};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub enum TimeState {
    Work,
    ShortBreak,
    LongBreak,
}
