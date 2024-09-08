use crate::{system::Body, Frequency, Velocity};

/// The gravitational constant as N⋅m^2⋅kg^−2.
pub const GRAVITATIONAL_CONSTANT: f64 = 6.67430e-11;

/// The orbit of an object around a central body.
pub trait Orbit {
    /// The orbital velocity of the object.
    fn velocity(&self, central_body: &Body) -> Velocity;
    /// The orbit's frequency.
    fn frequency(&self, central_body: &Body) -> Frequency;
}
