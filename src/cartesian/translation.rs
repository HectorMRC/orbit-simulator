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

        let point = translation * Vector4::new(point.x(), point.y(), point.z(), 1.);
        [point.x, point.y, point.z].into()
    }
}


#[cfg(test)]
mod tests {
    use crate::{CartesianPoint, Translation};

    #[test]
    fn translation_must_not_fail() {
        struct Test {
            name: &'static str,
            vector: CartesianPoint,
            input: CartesianPoint,
            output: CartesianPoint,
        }

        vec![
            Test {
                name: "the negative of the input should move the point to the origin",
                vector: CartesianPoint::from([-1., -2., -3.]),
                input: CartesianPoint::from([1., 2., 3.]),
                output: CartesianPoint::from([0., 0., 0.]),
            },
            Test {
                name: "translation should be the sum of both vectors",
                vector: CartesianPoint::from([1., 2., 3.]),
                input: CartesianPoint::from([8., 7., 6.]),
                output: CartesianPoint::from([9., 9., 9.]),
            },
        ]
        .into_iter()
        .for_each(|test| {
            let rotated = Translation::default()
                .with_vector(test.vector)
                .translate(test.input);

            assert_eq!(
                rotated,
                test.output,
                "{}: got rotated = {:?}, want ± e = {:?}",
                test.name,
                rotated,
                test.output
            );
        });
    }
}
