use crate::PositiveFloat;

/// The velocity at which an aritrary object moves throught space, which is always a positive number.
pub struct Velocity(PositiveFloat);

impl Velocity {
    /// Returns a new mass of v meters per second.
    pub fn meters_sec(v: f64) -> Self {
        Self(v.into())
    }

    /// Returns an [f64] representing the velocity in meters per second.
    pub fn as_meters_sec(&self) -> f64 {
        self.0.into()
    }
}
