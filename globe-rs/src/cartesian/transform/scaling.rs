use nalgebra::Matrix3;

use super::{Coords, Transform};

/// Implements the [geometric transformation](https://en.wikipedia.org/wiki/Scaling_(geometry))
/// through which an arbitrary [Cartesian]s can be scaled given a scale factor.
#[derive(Default, Clone, Copy)]
pub struct Scaling {
    pub factor: f64,
}

impl Transform for Scaling {
    fn transform(&self, point: Coords) -> Coords {
        let scaling = Matrix3::new(
            self.factor,
            0.,
            0.,
            0.,
            self.factor,
            0.,
            0.,
            0.,
            self.factor,
        );

        Coords::from(scaling * point.0)
    }
}

impl Scaling {
    pub fn with_factor(mut self, factor: f64) -> Self {
        self.factor = factor;
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::cartesian::{
        transform::{Scaling, Transform},
        Coords,
    };

    #[test]
    fn scaling_must_not_fail() {
        struct Test {
            name: &'static str,
            factor: f64,
            input: Coords,
            output: Coords,
        }

        vec![
            Test {
                name: "factor of 1 should not change the point",
                factor: 1.,
                input: Coords::from([0., 1., 0.]),
                output: Coords::from([0., 1., 0.]),
            },
            Test {
                name: "factor of 2 should duplicate the magnitude of the point",
                factor: 2.,
                input: Coords::from([0., 1., 0.]),
                output: Coords::from([0., 2., 0.]),
            },
            Test {
                name: "factor of a half should divide the magnitude by two",
                factor: 0.5,
                input: Coords::from([0., 1., 0.]),
                output: Coords::from([0., 0.5, 0.]),
            },
        ]
        .into_iter()
        .for_each(|test| {
            let rotated = Scaling::default()
                .with_factor(test.factor)
                .transform(test.input);

            assert_eq!(
                rotated, test.output,
                "{}: got rotated = {:?}, want ± e = {:?}",
                test.name, rotated, test.output
            );
        });
    }
}
