use crate::Radiant;

use super::Coords;

mod arc;
pub use arc::*;

mod ellipse;
pub use ellipse::*;

/// A succession of [Cartesian]s representing an arbitrary shape.
#[derive(Default)]
pub struct Shape {
    pub points: Vec<Coords>,
}

/// A continious shape that can be sampled into a discrete [Shape].
pub trait Sample {
    /// Samples the continuous shape as a discrete set of [Cartesian]s by dividing it into
    /// segments.
    fn sample(&self, segments: usize) -> Shape;
}

/// A shape that can be divided into sectors.
pub trait WithSector {
    /// Determines the radiant at which the sector begins. 
    fn with_initial_theta(self, theta: Radiant) -> Self;

    /// Determines the size in radiants of the sector.
    fn with_theta(self, theta: Radiant) -> Self;
}
