use super::Altitude;

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

/// An arbitrary mass, which is always a positive number.
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
