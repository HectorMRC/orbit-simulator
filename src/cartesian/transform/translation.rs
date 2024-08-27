use std::ops::Neg;

use nalgebra::{Matrix4, Vector4};

use super::{Cartesian, Transform};

/// Implements the [geometric transformation](https://en.wikipedia.org/wiki/Translation_(geometry))
/// through which an arbitrary [Cartesian]s can be translated given a translation vector.
#[derive(Default, Clone, Copy)]
pub struct Translation {
    pub vector: Cartesian,
}

impl Neg for Translation {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            vector: -self.vector,
        }
    }
}

impl Transform for Translation {
    fn transform(&self, point: Cartesian) -> Cartesian {
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

impl Translation {
    pub fn with_vector(mut self, vector: Cartesian) -> Self {
        self.vector = vector;
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::{
        transform::{Transform, Translation},
        Cartesian,
    };

    #[test]
    fn translation_must_not_fail() {
        struct Test {
            name: &'static str,
            vector: Cartesian,
            input: Cartesian,
            output: Cartesian,
        }

        vec![
            Test {
                name: "the negative of the input should move the point to the origin",
                vector: Cartesian::from([-1., -2., -3.]),
                input: Cartesian::from([1., 2., 3.]),
                output: Cartesian::from([0., 0., 0.]),
            },
            Test {
                name: "translation should be the sum of both vectors",
                vector: Cartesian::from([1., 2., 3.]),
                input: Cartesian::from([8., 7., 6.]),
                output: Cartesian::from([9., 9., 9.]),
            },
        ]
        .into_iter()
        .for_each(|test| {
            let translated = Translation::default()
                .with_vector(test.vector)
                .transform(test.input);

            assert_eq!(
                translated, test.output,
                "{}: got rotated = {:?}, want ± e = {:?}",
                test.name, translated, test.output
            );
        });
    }
}
