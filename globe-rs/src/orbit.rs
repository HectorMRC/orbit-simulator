use std::time::Duration;

use crate::{
    cartesian::{shape::Sample, Coords},
    system::Body,
    Distance, Frequency, Radian, Velocity,
};

/// The gravitational constant as N⋅m^2⋅kg^−2.
pub const GRAVITATIONAL_CONSTANT: f64 = 6.674010551359e-11;

/// The orbit of an object around a central body.
pub trait Orbit: Copy + Sample {
    /// The minimum velocity of the object across the orbit.
    fn min_velocity(&self, orbitee: &Body) -> Velocity;

    /// The maximum velocity of the object across the orbit.
    fn max_velocity(&self, orbitee: &Body) -> Velocity;

    /// The orbital velocity of the object at ha given theta.
    fn velocity_at(&self, time: Duration, orbitee: &Body) -> Velocity;

    /// Returns the position of the object at the given theta of the orbit after
    /// a given duration.
    fn position_at(&self, time: Duration, orbitee: &Body) -> Coords;

    /// Returns the radiant of the orbit at which is located the object.
    fn theta_at(&self, time: Duration, orbitee: &Body) -> Radian;

    /// The orbit's period.
    fn period(&self, orbitee: &Body) -> Duration;

    /// The orbit's frequency.
    fn frequency(&self, orbitee: &Body) -> Frequency {
        Frequency::hz(1. / self.period(orbitee).as_secs_f64())
    }

    /// Returns the perimeter of the orbit.
    fn perimeter(&self) -> Distance;

    /// Returns the position, relative to the orbit's center, in which the
    /// orbitee is located.
    fn focus(&self) -> Coords;

    /// Returns the distance from the orbit's focus to the outer-most orbit.
    fn radius(&self) -> Distance;
}
