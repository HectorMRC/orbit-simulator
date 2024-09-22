use std::{fmt::Debug, ops::Div, time::Duration};

use serde::{Deserialize, Serialize};

use crate::PositiveFloat;

/// The frequency at which an specific event occurs per unit of time.
#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Frequency(PositiveFloat);

impl Div<Frequency> for f64 {
    type Output = Duration;

    fn div(self, rhs: Frequency) -> Self::Output {
        Duration::from_secs_f64(self / rhs.as_hz())
    }
}

impl Debug for Frequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Frequency").field(&format!("{} hz", self.0)).finish()
    }
}

impl Frequency {
    /// Returns a new frequency of hz hertz, which is the number of ocurrences per second.
    pub fn hz(hz: f64) -> Self {
        Self(hz.into())
    }

    /// Returns a [f64] representing the frequency in hertz.
    pub fn as_hz(&self) -> f64 {
        self.0 .0
    }
}
