use super::Coords;

mod rotation;
pub use rotation::*;

mod scaling;
pub use scaling::*;

mod translation;
pub use translation::*;

/// A geometric transformation.
pub trait Transform: Copy {
    /// Performs the geometric transformation over the given point.
    fn transform(&self, point: Coords) -> Coords;
}
