use crate::Distance;

use super::Coords;

mod arc;
pub use arc::*;

mod ellipse;
pub use ellipse::*;

/// Used to convert cartesian units into actual distances.
pub trait Scale {
    fn distance(value: f64) -> Distance;
}

/// An scale in which the units are in kilometers.
pub struct Kilometric;

impl Scale for Kilometric {
    fn distance(value: f64) -> Distance {
        Distance::km(value)
    }
}

/// A succession of [Cartesian]s representing an arbitrary shape.
#[derive(Default)]
pub struct Shape {
    pub points: Vec<Coords>,
}

/// A continious shape that can be sampled into a discrete [Shape].
pub trait Sample {
    /// Converts the continuous shape into a discrete set of [Cartesian]s by dividing it into
    /// segments.
    fn sample(&self, segments: usize) -> Shape;
}
