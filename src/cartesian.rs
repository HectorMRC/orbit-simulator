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

    /// Rotates self in theta radians around the edge passing by the origin and the given axis point.
    pub fn rotate(&mut self, axis: Self, theta: f64) {
        if self.0.normalize() == axis.0.normalize() {
            // the point belongs to the axis line, so the rotation takes no effect
            return;
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
            self.0 = ry_inv * rz * ry * self.0;
            return;
        }

        let c_div_d =
            Vector3::new(0., 0., axis.z()).dot(&Vector3::new(0., axis.y(), axis.z())) / cd;

        let b_div_d = Vector3::new(0., 0., axis.z())
            .cross(&Vector3::new(0., axis.y(), axis.z()))
            .norm()
            / cd;

        let rx = Matrix3::new(1., 0., 0., 0., c_div_d, -b_div_d, 0., b_div_d, c_div_d);
        let rx_inv = Matrix3::new(1., 0., 0., 0., c_div_d, b_div_d, 0., -b_div_d, c_div_d);

        self.0 = rx_inv * ry_inv * rz * ry * rx * self.0;
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use float_cmp::approx_eq;

//     const ULPS: i64 = 2;

//     #[test]
//     fn cartesian_from_geographic_must_not_fail() {
//         struct TestCase {
//             name: &'static str,
//             geographic: GeographicPoint,
//             cartesian: CartesianPoint,
//         }

//         vec![
//             TestCase {
//                 name: "north point",
//                 geographic: GeographicPoint::default().with_latitude(FRAC_PI_2),
//                 cartesian: CartesianPoint::new(0., 0., 1.),
//             },
//             TestCase {
//                 name: "south point",
//                 geographic: GeographicPoint::default().with_latitude(-FRAC_PI_2),
//                 cartesian: CartesianPoint::new(0., 0., -1.),
//             },
//             TestCase {
//                 name: "east point",
//                 geographic: GeographicPoint::default().with_longitude(FRAC_PI_2),
//                 cartesian: CartesianPoint::new(0., 1., 0.),
//             },
//             TestCase {
//                 name: "weast point",
//                 geographic: GeographicPoint::default().with_longitude(-FRAC_PI_2),
//                 cartesian: CartesianPoint::new(0., -1., 0.),
//             },
//             TestCase {
//                 name: "front point",
//                 geographic: GeographicPoint::default(),
//                 cartesian: CartesianPoint::new(1., 0., 0.),
//             },
//             TestCase {
//                 name: "back point as negative bound",
//                 geographic: GeographicPoint::default().with_longitude(-PI),
//                 cartesian: CartesianPoint::new(-1., 0., 0.),
//             },
//             TestCase {
//                 name: "back point as positive bound",
//                 geographic: GeographicPoint::default().with_longitude(PI),
//                 cartesian: CartesianPoint::new(-1., 0., 0.),
//             },
//         ]
//         .into_iter()
//         .for_each(|test_case| {
//             let point = CartesianPoint::from(test_case.geographic);
//             assert!(
//                 approx_eq!(
//                     &[f64],
//                     point.0.as_slice(),
//                     test_case.cartesian.0.as_slice(),
//                     ulps = ULPS
//                 ),
//                 "{}: {} ±ε = {}",
//                 test_case.name,
//                 point.0,
//                 test_case.cartesian.0
//             );
//         });
//     }

//     #[test]
//     fn rotate_must_not_fail() {
//         const EPSILON: f64 = 0.00000000000000024492935982947064;

//         struct TestCase {
//             name: &'static str,
//             theta: f64,
//             axis: CartesianPoint,
//             origin: CartesianPoint,
//             want: CartesianPoint,
//         }

//         vec![
//             TestCase {
//                 name: "full rotation on the x axis must not change the y point",
//                 theta: 2. * PI,
//                 axis: CartesianPoint::new(1., 0., 0.),
//                 origin: CartesianPoint::new(0., 1., 0.),
//                 want: CartesianPoint::new(0., 1., 0.),
//             },
//             TestCase {
//                 name: "half of a whole rotation on the x axis must change the y point",
//                 theta: PI,
//                 axis: CartesianPoint::new(1., 0., 0.),
//                 origin: CartesianPoint::new(0., 1., 0.),
//                 want: CartesianPoint::new(0., -1., 0.),
//             },
//             TestCase {
//                 name: "a quarter of a whole rotation on the x axis must change the y point",
//                 theta: FRAC_PI_2,
//                 axis: CartesianPoint::new(1., 0., 0.),
//                 origin: CartesianPoint::new(0., 1., 0.),
//                 want: CartesianPoint::new(0., 0., -1.),
//             },
//             TestCase {
//                 name: "full rotation on the z axis must not change the y point",
//                 theta: 2. * PI,
//                 axis: CartesianPoint::new(0., 0., 1.),
//                 origin: CartesianPoint::new(0., 1., 0.),
//                 want: CartesianPoint::new(0., 1., 0.),
//             },
//             TestCase {
//                 name: "half of a whole rotation on the z axis must change the y point",
//                 theta: PI,
//                 axis: CartesianPoint::new(0., 0., 1.),
//                 origin: CartesianPoint::new(0., 1., 0.),
//                 want: CartesianPoint::new(0., -1., 0.),
//             },
//             TestCase {
//                 name: "a quarter of a whole rotation on the z axis must change the y point",
//                 theta: FRAC_PI_2,
//                 axis: CartesianPoint::new(0., 0., 1.),
//                 origin: CartesianPoint::new(0., 1., 0.),
//                 want: CartesianPoint::new(1., 0., 0.),
//             },
//             TestCase {
//                 name: "rotate over itself must not change the point",
//                 theta: FRAC_PI_2,
//                 axis: CartesianPoint::new(0., 1., 0.),
//                 origin: CartesianPoint::new(0., 1., 0.),
//                 want: CartesianPoint::new(0., 1., 0.),
//             },
//         ]
//         .into_iter()
//         .for_each(|mut test_case| {
//             test_case.origin.rotate(test_case.axis, test_case.theta);

//             assert!(
//                 approx_eq!(
//                     &[f64],
//                     test_case.origin.0.as_slice(),
//                     test_case.want.0.as_slice(),
//                     epsilon = EPSILON,
//                     ulps = 17
//                 ),
//                 "{}: {} ±ε = {}",
//                 test_case.name,
//                 test_case.origin.0,
//                 test_case.want.0
//             );
//         });
//     }
// }
