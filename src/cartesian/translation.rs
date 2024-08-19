use nalgebra::{Matrix4, Vector4};

use super::CartesianPoint;

/// Implements the [geometric transformation](https://en.wikipedia.org/wiki/Translation_(geometry))
/// through which an arbitrary [CartesianPoint]s can be translated given a translation vector.
#[derive(Default)]
pub struct Translation {
    pub vector: CartesianPoint,
}

impl Translation {
    pub fn with_vector(mut self, vector: CartesianPoint) -> Self {
        self.vector = vector;
        self
    }

    /// Performs the translation over the given point.
    pub fn translate(&self, point: CartesianPoint) -> CartesianPoint {
        let translation = Matrix4::new(
            1.,
            0.,
            0.,
            self.vector.x(),
            0.,
            1.,
            0.,
            self.vector.y(),
            0.,
            0.,
            1.,
            self.vector.z(),
            0.,
            0.,
            0.,
            1.,
        );

        let point = Vector4::new(point.x(), point.y(), point.z(), 1.);
        TryInto::<[f64; 3]>::try_into((translation * point).as_slice())
            .unwrap_or_default()
            .into()
    }
}
