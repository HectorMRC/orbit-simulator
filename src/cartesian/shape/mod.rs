use super::Cartesian;

mod arc;
pub use arc::*;

/// A succession of [Cartesian]s representing an arbitrary shape.
#[derive(Default)]
pub struct Shape {
    pub points: Vec<Cartesian>,
}

/// A continious shape that can be sampled into a discrete [Shape].
pub trait Sample {
    /// Converts the continuous shape into a discrete set of [Cartesian]s by dividing it into
    /// segments.
    fn sample(&self, segments: usize) -> Shape;
}
