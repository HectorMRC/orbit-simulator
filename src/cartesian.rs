use std::f64::consts::{FRAC_PI_2, PI};

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

    /// Returns the normal version of the [CartesianPoint], which length is exactly 1.
    pub fn normal(&self) -> Self {
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

    /// Returns the [CartesianPoint] resulting from rotating self theta radians around the edge
    /// passing by the origin and the given axis point.
    pub fn rotate(&self, axis: Self, theta: f64) -> Self {
        let mut rotated_point = *self;

        if rotated_point.0.normalize() == axis.0.normalize() {
            // the point belongs to the axis line, so the rotation takes no effect
            return rotated_point;
        }

        let d = (axis.y().powi(2) + axis.z().powi(2)).sqrt();
        let cd = axis.z() * d;

        let rz = Matrix3::new(
            theta.cos(),
            theta.sin(),
            0.,
            -theta.sin(),
            theta.cos(),
            0.,
            0.,
            0.,
            1.,
        );

        let ry = Matrix3::new(d, 0., -axis.x(), 0., 1., 0., axis.x(), 0., d);
        let ry_inv = Matrix3::new(d, 0., axis.x(), 0., 1., 0., -axis.x(), 0., d);

        if d == 0. {
            // the rotation axis is already perpendicular to the xy plane.
            rotated_point.0 = ry_inv * rz * ry * rotated_point.0;
            return rotated_point;
        }

        let c_div_d =
            Vector3::new(0., 0., axis.z()).dot(&Vector3::new(0., axis.y(), axis.z())) / cd;

        let b_div_d = Vector3::new(0., 0., axis.z())
            .cross(&Vector3::new(0., axis.y(), axis.z()))
            .norm()
            / cd;

        let rx = Matrix3::new(1., 0., 0., 0., c_div_d, -b_div_d, 0., b_div_d, c_div_d);
        let rx_inv = Matrix3::new(1., 0., 0., 0., c_div_d, b_div_d, 0., -b_div_d, c_div_d);

        rotated_point.0 = rx_inv * ry_inv * rz * ry * rx * rotated_point.0;
        rotated_point
    }
}

#[cfg(test)]
mod tests {
    use std::{
        f64::consts::{FRAC_PI_2, PI},
        ops::{Add, Sub},
    };

    use crate::{tests::approx_eq, CartesianPoint, GeographicPoint, Latitude, Longitude};

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
    fn rotate_must_not_fail() {
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
                output: CartesianPoint::from([0., 0., -1.]),
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
                output: CartesianPoint::from([1., 0., 0.]),
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
            let rotated = test.input.rotate(test.axis, test.theta);

            assert!(
                approx_eq(rotated, test.output, EPSILON),
                "{}: {:?} ±ε = {:?}",
                test.name,
                rotated,
                test.output
            );
        });
    }
}
