use std::fmt::Debug;

use crate::PositiveFloat;

/// The velocity at which an aritrary object moves throught space, which is always a positive
/// number.
#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Velocity(PositiveFloat);

impl Debug for Velocity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Velocity").field(&format!("{} m/s", self.0)).finish()
    }
}

impl Velocity {
    /// Returns a new velocity of v meters per second.
    pub fn meters_sec(v: f64) -> Self {
        Self(v.into())
    }

    /// Returns a [f64] representing the velocity in meters per second.
    pub fn as_meters_sec(&self) -> f64 {
        self.0 .0
    }
}
