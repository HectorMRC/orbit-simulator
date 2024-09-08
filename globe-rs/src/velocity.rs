use crate::PositiveFloat;

/// The velocity at which an aritrary object moves throught space, which is always a positive
/// number.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Velocity(PositiveFloat);

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
