use std::{
    f64::consts::{FRAC_PI_2, PI},
    ops::{Add, Sub},
};

use nalgebra::{Matrix3, Vector3};

use crate::GeographicPoint;

/// Represents an arbitrary point in space using the cartesian system of coordinates.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct CartesianPoint(Vector3<f64>);

impl<T> From<T> for CartesianPoint
where
    T: Into<Vector3<f64>>,
{
    fn from(value: T) -> Self {
        CartesianPoint(value.into())
    }
}

impl PartialOrd for CartesianPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Add<f64> for CartesianPoint {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self::from(self.0.add_scalar(rhs))
    }
}

impl Sub<f64> for CartesianPoint {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        Self::from(self.0.add_scalar(-rhs))
    }
}

impl From<GeographicPoint> for CartesianPoint {
    fn from(point: GeographicPoint) -> Self {
        let radial_distance = match point.altitude.as_f64() {
            altitude if altitude == 0. => 1.,
            altitude => altitude,
        };

        let theta = FRAC_PI_2 - point.latitude.as_f64();
        let phi = point.longitude.as_f64();

        // improves sin & cos precision for exact numbers
        let precise_sin_cos = |rad: f64| -> (f64, f64) {
            if rad.abs() == FRAC_PI_2 {
                return (rad.signum(), 0.);
            } else if rad.abs() == PI {
                return (0., -1.);
            } else if rad == 0. {
                return (0., 1.);
            }

            (rad.sin(), rad.cos())
        };

        let (theta_sin, theta_cos) = precise_sin_cos(theta);
        let (phi_sin, phi_cos) = precise_sin_cos(phi);

        Vector3::new(
            radial_distance * theta_sin * phi_cos,
            radial_distance * theta_sin * phi_sin,
            radial_distance * theta_cos,
        )
        .into()
    }
}

impl CartesianPoint {
    pub fn with_x(mut self, x: f64) -> Self {
        self.0[0] = x;
        self
    }

    pub fn with_y(mut self, y: f64) -> Self {
        self.0[1] = y;
        self
    }

    pub fn with_z(mut self, z: f64) -> Self {
        self.0[2] = z;
        self
    }

    pub fn x(&self) -> f64 {
        self.0[0]
    }

    pub fn y(&self) -> f64 {
        self.0[1]
    }

    pub fn z(&self) -> f64 {
        self.0[2]
    }

    /// Returns the [CartesianPoint] representing the unitary vector of self.
    pub fn unit(&self) -> Self {
        self.0.normalize().into()
    }

    /// Returns the distance between self and the given point.
    pub fn distance(&self, other: &CartesianPoint) -> f64 {
        let square_diff = |p1: f64, p2: f64| -> f64 { (p1 - p2).powi(2) };

        (square_diff(self.x(), other.x())
            + square_diff(self.y(), other.y())
            + square_diff(self.z(), other.z()))
        .sqrt()
    }

    /// Performs the cartesian product between self and the given point.
    pub fn cross(&self, other: &CartesianPoint) -> Self {
        self.0.cross(&other.0).into()
    }
}

/// Implements the [Rodrigues' rotation formula](https://en.wikipedia.org/wiki/Rodrigues%27_rotation_formula)
/// through which an arbitrary [CartesianPoint]s can be rotated given an axis and an angle of
/// rotation.
///
/// ## Statement
/// Being v a vector in ℝ3 and k a unit vector describing an axis of rotation about which v rotates
/// by an angle θ, the Rodrigues' rotation formula rotates v according to the right hand rule.
///
/// ## Example
/// ```
/// use std::f64::consts::FRAC_PI_2;
/// use globe_rs::{approx_eq, CartesianPoint, RotationMatrix};
///
/// let rotated = RotationMatrix::default()
///     .with_axis(CartesianPoint::from([1., 0., 0.]))
///     .with_theta(FRAC_PI_2)
///     .rotate(&CartesianPoint::from([0., 1., 0.]));
///
/// // due precision error both values may not be exactly the same  
/// let epsilon = 0.0000000000000001;
///
/// assert!(
///     approx_eq(rotated, CartesianPoint::from([0., 0., 1.]), epsilon),
///     "point at y1 should be rotated around the x axis to z1"
/// );
/// ```
#[derive(Debug, Default)]
pub struct RotationMatrix {
    axis: CartesianPoint,
    theta: f64,
}

impl RotationMatrix {
    pub fn with_axis(mut self, axis: CartesianPoint) -> Self {
        self.axis = axis.unit();
        self
    }

    pub fn with_theta(mut self, theta: f64) -> Self {
        self.theta = theta;
        self
    }

    pub fn rotate(&self, point: &CartesianPoint) -> CartesianPoint {
        let sin_theta = self.theta.sin();
        let cos_theta = self.theta.cos();
        let sub_1_cos_theta = 1. - cos_theta;

        let x = self.axis.x();
        let y = self.axis.y();
        let z = self.axis.z();

        let matrix = Matrix3::new(
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

        (matrix * point.0).into()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, PI};

    use crate::{approx_eq, CartesianPoint, GeographicPoint, Latitude, Longitude, RotationMatrix};

    #[test]
    fn cartesian_from_geographic_must_not_fail() {
        struct Test {
            name: &'static str,
            input: GeographicPoint,
            output: CartesianPoint,
        }

        vec![
            Test {
                name: "north point",
                input: GeographicPoint::default().with_latitude(Latitude::from(FRAC_PI_2)),
                output: CartesianPoint::from([0., 0., 1.]),
            },
            Test {
                name: "south point",
                input: GeographicPoint::default().with_latitude(Latitude::from(-FRAC_PI_2)),
                output: CartesianPoint::from([0., 0., -1.]),
            },
            Test {
                name: "east point",
                input: GeographicPoint::default().with_longitude(Longitude::from(FRAC_PI_2)),
                output: CartesianPoint::from([0., 1., 0.]),
            },
            Test {
                name: "weast point",
                input: GeographicPoint::default().with_longitude(Longitude::from(-FRAC_PI_2)),
                output: CartesianPoint::from([0., -1., 0.]),
            },
            Test {
                name: "front point",
                input: GeographicPoint::default(),
                output: CartesianPoint::from([1., 0., 0.]),
            },
            Test {
                name: "back point as negative bound",
                input: GeographicPoint::default().with_longitude(Longitude::from(-PI)),
                output: CartesianPoint::from([-1., 0., 0.]),
            },
            Test {
                name: "back point as positive bound",
                input: GeographicPoint::default().with_longitude(Longitude::from(PI)),
                output: CartesianPoint::from([-1., 0., 0.]),
            },
        ]
        .into_iter()
        .for_each(|test| {
            let point = CartesianPoint::from(test.input);
            assert_eq!(
                point, test.output,
                "{}: got cartesian point = {:#?}, want {:#?}",
                test.name, point, test.output
            );
        });
    }

    #[test]
    fn unit_must_not_fail() {
        struct Test {
            name: &'static str,
            input: CartesianPoint,
            output: CartesianPoint,
        }

        vec![
            Test {
                name: "lenght of unit vector must be 1 at x axis",
                input: CartesianPoint::from([2., 0., 0.]),
                output: CartesianPoint::from([1., 0., 0.]),
            },
            Test {
                name: "lenght of unit vector must be 1 at y axis",
                input: CartesianPoint::from([0., 3., 0.]),
                output: CartesianPoint::from([0., 1., 0.]),
            },
            Test {
                name: "lenght of unit vector must be 1 at z axis",
                input: CartesianPoint::from([0., 0., -4.]),
                output: CartesianPoint::from([0., 0., -1.]),
            },
        ]
        .into_iter()
        .for_each(|test| {
            let unit = test.input.unit();
            assert_eq!(
                unit, test.output,
                "{}: got unit = {:?}, want {:?}",
                test.name, unit, test.output
            );
        })
    }

    #[test]
    fn rotation_matrix_must_not_fail() {
        const EPSILON: f64 = 0.0000000000000003;

        struct Test {
            name: &'static str,
            theta: f64,
            axis: CartesianPoint,
            input: CartesianPoint,
            output: CartesianPoint,
        }

        vec![
            Test {
                name: "full rotation on the x axis must not change the y point",
                theta: 2. * PI,
                axis: CartesianPoint::from([1., 0., 0.]),
                input: CartesianPoint::from([0., 1., 0.]),
                output: CartesianPoint::from([0., 1., 0.]),
            },
            Test {
                name: "half of a whole rotation on the x axis must change the y point",
                theta: PI,
                axis: CartesianPoint::from([1., 0., 0.]),
                input: CartesianPoint::from([0., 1., 0.]),
                output: CartesianPoint::from([0., -1., 0.]),
            },
            Test {
                name: "a quarter of a whole rotation on the x axis must change the y point",
                theta: FRAC_PI_2,
                axis: CartesianPoint::from([1., 0., 0.]),
                input: CartesianPoint::from([0., 1., 0.]),
                output: CartesianPoint::from([0., 0., 1.]),
            },
            Test {
                name: "full rotation on the z axis must not change the y point",
                theta: 2. * PI,
                axis: CartesianPoint::from([0., 0., 1.]),
                input: CartesianPoint::from([0., 1., 0.]),
                output: CartesianPoint::from([0., 1., 0.]),
            },
            Test {
                name: "half of a whole rotation on the z axis must change the y point",
                theta: PI,
                axis: CartesianPoint::from([0., 0., 1.]),
                input: CartesianPoint::from([0., 1., 0.]),
                output: CartesianPoint::from([0., -1., 0.]),
            },
            Test {
                name: "a quarter of a whole rotation on the z axis must change the y point",
                theta: FRAC_PI_2,
                axis: CartesianPoint::from([0., 0., 1.]),
                input: CartesianPoint::from([0., 1., 0.]),
                output: CartesianPoint::from([-1., 0., 0.]),
            },
            Test {
                name: "rotate over itself must not change the point",
                theta: FRAC_PI_2,
                axis: CartesianPoint::from([0., 1., 0.]),
                input: CartesianPoint::from([0., 1., 0.]),
                output: CartesianPoint::from([0., 1., 0.]),
            },
        ]
        .into_iter()
        .for_each(|test| {
            let rotated = RotationMatrix::default()
                .with_axis(test.axis)
                .with_theta(test.theta)
                .rotate(&test.input);

            assert!(
                approx_eq(rotated, test.output, EPSILON),
                "{}: got rotated = {:?}, want ±ε = {:?}",
                test.name,
                rotated,
                test.output
            );
        });
    }
}
