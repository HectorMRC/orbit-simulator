use std::ops::Neg;

use nalgebra::Matrix3;

use crate::Radiant;

use super::{Coords, Transform};

/// Implements the [geometric transformation](https://en.wikipedia.org/wiki/Rotation_matrix)
/// through which an arbitrary [Cartesian]s can be rotated given an axis and an angle of
/// rotation.
///
/// ## Statement
/// Being v a vector in ℝ3 and k a unit vector describing an axis of rotation about which v rotates
/// by an angle θ, the rotation transformation rotates v according to the right hand rule.
///
/// ## Example
/// ```
/// use std::f64::consts::FRAC_PI_2;
///
/// use globe_rs::{
///     cartesian::{
///         transform::{Rotation, Transform},
///         Coords,
///     },
/// };
///
/// // due precision error both values may not be exactly the same
/// const ABS_ERROR: f64 = 0.0000000000000001;
///
///
/// let rotated = Rotation::default()
///     .with_axis(Coords::from([1., 0., 0.]))
///     .with_theta(FRAC_PI_2.into())
///     .transform(Coords::from([0., 1., 0.]));
///
/// rotated
///     .into_iter()
///     .zip(Coords::from([0., 0., 1.]).into_iter())
///     .for_each(|(&got, &want)| {
///         assert!(
///             (got - want).abs() <= ABS_ERROR,
///             "point at y1 should be rotated around the x axis to z1",
///         );
///     });
/// ```
#[derive(Debug, Default, Clone, Copy)]
pub struct Rotation {
    /// The axis of rotation about which perform the transformation.
    pub axis: Coords,
    /// The angle of rotation.
    pub theta: Radiant,
}

impl Transform for Rotation {
    fn transform(&self, point: Coords) -> Coords {
        let sin_theta = self.theta.as_f64().sin();
        let cos_theta = self.theta.as_f64().cos();
        let sub_1_cos_theta = 1. - cos_theta;

        let x = self.axis.x();
        let y = self.axis.y();
        let z = self.axis.z();

        let rotation = Matrix3::new(
            cos_theta + x.powi(2) * sub_1_cos_theta,
            x * y * sub_1_cos_theta - z * sin_theta,
            x * z * sub_1_cos_theta + y * sin_theta,
            y * x * sub_1_cos_theta + z * sin_theta,
            cos_theta + y.powi(2) * sub_1_cos_theta,
            y * z * sub_1_cos_theta - x * sin_theta,
            z * x * sub_1_cos_theta - y * sin_theta,
            z * y * sub_1_cos_theta + x * sin_theta,
            cos_theta + z.powi(2) * sub_1_cos_theta,
        );

        (rotation * point.0).into()
    }
}

impl Neg for Rotation {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            theta: Radiant::from(-self.theta.as_f64()),
            axis: self.axis,
        }
    }
}

impl Rotation {
    pub fn with_axis(mut self, axis: Coords) -> Self {
        self.axis = axis.unit();
        self
    }

    pub fn with_theta(mut self, theta: Radiant) -> Self {
        self.theta = theta;
        self
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, PI};

    use crate::{
        cartesian::{
            transform::{Rotation, Transform},
            Coords,
        },
        tests::approx_eq,
        Radiant,
    };

    #[test]
    fn rotation_must_not_fail() {
        const ABS_ERROR: f64 = 0.0000000000000003;

        struct Test {
            name: &'static str,
            theta: Radiant,
            axis: Coords,
            input: Coords,
            output: Coords,
        }

        vec![
            Test {
                name: "full rotation on the x axis must not change the y point",
                theta: Radiant::from(2. * PI),
                axis: Coords::from([1., 0., 0.]),
                input: Coords::from([0., 1., 0.]),
                output: Coords::from([0., 1., 0.]),
            },
            Test {
                name: "half of a whole rotation on the x axis must change the y point",
                theta: Radiant::from(PI),
                axis: Coords::from([1., 0., 0.]),
                input: Coords::from([0., 1., 0.]),
                output: Coords::from([0., -1., 0.]),
            },
            Test {
                name: "a quarter of a whole rotation on the x axis must change the y point",
                theta: Radiant::from(FRAC_PI_2),
                axis: Coords::from([1., 0., 0.]),
                input: Coords::from([0., 1., 0.]),
                output: Coords::from([0., 0., 1.]),
            },
            Test {
                name: "full rotation on the z axis must not change the y point",
                theta: Radiant::from(2. * PI),
                axis: Coords::from([0., 0., 1.]),
                input: Coords::from([0., 1., 0.]),
                output: Coords::from([0., 1., 0.]),
            },
            Test {
                name: "half of a whole rotation on the z axis must change the y point",
                theta: Radiant::from(PI),
                axis: Coords::from([0., 0., 1.]),
                input: Coords::from([0., 1., 0.]),
                output: Coords::from([0., -1., 0.]),
            },
            Test {
                name: "a quarter of a whole rotation on the z axis must change the y point",
                theta: Radiant::from(FRAC_PI_2),
                axis: Coords::from([0., 0., 1.]),
                input: Coords::from([0., 1., 0.]),
                output: Coords::from([-1., 0., 0.]),
            },
            Test {
                name: "rotate over itself must not change the point",
                theta: Radiant::from(FRAC_PI_2),
                axis: Coords::from([0., 1., 0.]),
                input: Coords::from([0., 1., 0.]),
                output: Coords::from([0., 1., 0.]),
            },
        ]
        .into_iter()
        .for_each(|test| {
            let rotated = Rotation::default()
                .with_axis(test.axis)
                .with_theta(test.theta)
                .transform(test.input);

            rotated
                .into_iter()
                .zip(&test.output)
                .for_each(|(&got, &want)| {
                    assert!(
                        approx_eq(got, want, ABS_ERROR),
                        "{}: got rotated = {:?}, want ± e = {:?}",
                        test.name,
                        rotated,
                        test.output
                    );
                });
        });
    }
}
