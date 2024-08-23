use super::CartesianPoint;

mod arc;
pub use arc::*;

/// A succession of [CartesianPoint]s representing an arbitrary shape.
#[derive(Default)]
pub struct Shape {
    pub points: Vec<CartesianPoint>,
}

/// A continious shape that can be sampled into a discrete [Shape].
pub trait Sample {
    /// Converts the continuous shape into a discrete set of [CartesianPoint]s by dividing it into segments.
    fn sample(&self, segments: usize) -> Shape;
}
