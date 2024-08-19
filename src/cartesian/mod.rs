use std::{
    f64::consts::{FRAC_PI_2, PI},
    ops::{Add, Div, Mul, Sub},
};

use nalgebra::Vector3;

use crate::GeographicPoint;

mod rotation;
pub use rotation::*;

mod shape;
pub use shape::*;

mod translation;
pub use translation::*;

/// An arbitrary point in space using the cartesian system of coordinates.
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

impl Mul<f64> for CartesianPoint {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::from(self.0.mul(rhs))
    }
}

impl Div<f64> for CartesianPoint {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::from(self.0.div(rhs))
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

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, PI};

    use crate::{CartesianPoint, GeographicPoint, Latitude, Longitude};

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
}
