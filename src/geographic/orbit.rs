use super::Altitude;

/// The gravitational constant.
pub const G: f64 = 6.67430e-11;

pub trait Orbit {
    fn velocity(&self, central_body: Body, distance: Altitude) -> f64;
}

/// An orbit in which the orbiting body moves in a perfect circle around the central body.
pub struct CircularOrbit;

impl Orbit for CircularOrbit {
    fn velocity(&self, central_body: Body, distance: Altitude) -> f64 {
        (G * f64::from(central_body.mass) / f64::from(distance)).sqrt()
    }
}

/// An orbit in which the orbiting body moves in an elliptical path around the central body.
pub struct EllipticalOrbit {
    pub periapsis: Altitude,
    pub apoapsis: Altitude,
}

impl Orbit for EllipticalOrbit {
    fn velocity(&self, central_body: Body, distance: Altitude) -> f64 {
        let semi_major_axis = (f64::from(self.periapsis) + f64::from(self.apoapsis)) / 2.;
        (G * f64::from(central_body.mass) * (2. / f64::from(distance) - 1. / semi_major_axis))
            .sqrt()
    }
}

/// An arbitrary [frequency](https://en.wikipedia.org/wiki/Frequency), which sign determines the
/// direction of rotation, being positive for counter-clockwise and negative for clockwise.
pub struct Frequency(f64);

impl From<f64> for Frequency {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<Frequency> for f64 {
    fn from(value: Frequency) -> Self {
        value.0
    }
}

/// An arbitrary mass, which is always positive.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Mass(f64);

impl From<f64> for Mass {
    fn from(value: f64) -> Self {
        Self(value.abs())
    }
}

impl From<Mass> for f64 {
    fn from(value: Mass) -> Self {
        value.0
    }
}

/// An arbitrary spherical body.
pub struct Body {
    /// The radius of the body.
    pub radius: Altitude,
    /// The frequency at which the body rotates around its own axis.
    pub frequency: Frequency,
    /// The mass of the body.
    pub mass: Mass,
}
