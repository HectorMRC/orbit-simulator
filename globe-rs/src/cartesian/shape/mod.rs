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
    /// Converts the continuous shape into a discrete set of [Cartesian]s by dividing it into
    /// segments.
    fn sample(&self, segments: usize) -> Shape;
}
