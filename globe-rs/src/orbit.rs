use std::time::Duration;

use crate::{
    cartesian::{shape::Scale, Coords},
    system::Body,
    Frequency, Velocity,
};

/// The gravitational constant as N⋅m^2⋅kg^−2.
pub const GRAVITATIONAL_CONSTANT: f64 = 6.674010551359e-11;

/// The orbit of an object around a central body.
pub trait Orbit {
    /// The orbital velocity of the object.
    fn velocity<S: Scale>(&self, central_body: &Body) -> Velocity;

    /// The orbit's period.
    fn period<S: Scale>(&self, central_body: &Body) -> Duration;

    /// The orbit's frequency.
    fn frequency<S: Scale>(&self, central_body: &Body) -> Frequency {
        Frequency::hz(1. / self.period::<S>(central_body).as_secs_f64())
    }

    /// Returns the position of the orbiter after a given duration.
    fn orbit<S: Scale>(&self, time: Duration, central_body: &Body) -> Coords;
}
