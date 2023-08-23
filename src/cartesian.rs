use crate::GeographicPoint;
use nalgebra::{Matrix3, Vector3};
use std::{
    f64::consts::{FRAC_PI_2, PI},
    ops::{Div, Index, IndexMut},
};
use wasm_bindgen::prelude::wasm_bindgen;

/// Represents a point using the Cartesian system of coordinates.
#[wasm_bindgen]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct CartesianPoint(Vector3<f64>);

impl From<GeographicPoint> for CartesianPoint {
    fn from(value: GeographicPoint) -> Self {
        CartesianPoint::from_geographic(&value)
    }
}

impl From<Vector3<f64>> for CartesianPoint {
    fn from(value: Vector3<f64>) -> Self {
        CartesianPoint(value)
    }
}

impl Index<usize> for CartesianPoint {
    type Output = f64;

    fn index(&self, index: usize) -> &<Self as Index<usize>>::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for CartesianPoint {
    fn index_mut(&mut self, index: usize) -> &mut <Self as Index<usize>>::Output {
        &mut self.0[index]
    }
}

#[wasm_bindgen]
impl CartesianPoint {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Vector3::new(x, y, z))
    }

    /// Returns the equivalent [`CartesianPoint`] of the given [`GeographicPoint`]
    pub fn from_geographic(point: &GeographicPoint) -> Self {
        let radial_distance = match point.altitude() {
            altitude if altitude == 0. => 1.,
            altitude => altitude,
        };

        let theta = FRAC_PI_2 - point.latitude();
        let phi = point.longitude();

        // improve sin & cos precision for exact numbers
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

        Self(Vector3::new(
            radial_distance * theta_sin * phi_cos,
            radial_distance * theta_sin * phi_sin,
            radial_distance * theta_cos,
        ))
    }

    #[inline(always)]
    pub fn x(&self) -> f64 {
        self[0]
    }

    #[inline(always)]
    pub fn set_x(&mut self, x: f64) {
        self[0] = x;
    }

    #[inline(always)]
    pub fn y(&self) -> f64 {
        self[1]
    }

    #[inline(always)]
    pub fn set_y(&mut self, y: f64) {
        self[1] = y;
    }

    #[inline(always)]
    pub fn z(&self) -> f64 {
        self[2]
    }

    #[inline(always)]
    pub fn set_z(&mut self, z: f64) {
        self[2] = z;
    }

    /// Returns the point resulting from rotating self in theta radians about the axis that is
    /// plotted from the origin of coordinates to the given axis point.
    pub fn rotate(&self, axis: Self, theta: f64) -> Self {
        if self.0.normalize() == axis.0.normalize() {
            // the point belongs to the axis line, so the rotation takes no effect
            return self.clone();
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
            return (ry_inv * rz * ry * self.0).into();
        }

        let c_div_d = Vector3::new(0., 0., axis.z())
            .dot(&Vector3::new(0., axis.y(), axis.z()))
            .div(cd);

        let b_div_d = Vector3::new(0., 0., axis.z())
            .cross(&Vector3::new(0., axis.y(), axis.z()))
            .norm()
            .div(cd);

        let rx = Matrix3::new(1., 0., 0., 0., c_div_d, -b_div_d, 0., b_div_d, c_div_d);
        let rx_inv = Matrix3::new(1., 0., 0., 0., c_div_d, b_div_d, 0., -b_div_d, c_div_d);

        (rx_inv * ry_inv * rz * ry * rx * self.0).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    const ULPS: i64 = 2;
    const EPSILON: f64 = 0.00000000000000024492935982947064;

    #[test]
    fn cartesian_from_geographic_must_not_fail() {
        struct TestCase {
            name: &'static str,
            geographic: GeographicPoint,
            cartesian: CartesianPoint,
        }

        vec![
            TestCase {
                name: "north point",
                geographic: GeographicPoint::default().with_latitude(FRAC_PI_2),
                cartesian: CartesianPoint::new(0., 0., 1.),
            },
            TestCase {
                name: "south point",
                geographic: GeographicPoint::default().with_latitude(-FRAC_PI_2),
                cartesian: CartesianPoint::new(0., 0., -1.),
            },
            TestCase {
                name: "east point",
                geographic: GeographicPoint::default().with_longitude(FRAC_PI_2),
                cartesian: CartesianPoint::new(0., 1., 0.),
            },
            TestCase {
                name: "weast point",
                geographic: GeographicPoint::default().with_longitude(-FRAC_PI_2),
                cartesian: CartesianPoint::new(0., -1., 0.),
            },
            TestCase {
                name: "front point",
                geographic: GeographicPoint::default(),
                cartesian: CartesianPoint::new(1., 0., 0.),
            },
            TestCase {
                name: "back point",
                geographic: GeographicPoint::default().with_longitude(-PI),
                cartesian: CartesianPoint::new(-1., 0., 0.),
            },
        ]
        .into_iter()
        .for_each(|test_case| {
            let point = CartesianPoint::from(test_case.geographic);
            assert!(
                approx_eq!(
                    &[f64],
                    point.0.as_slice(),
                    test_case.cartesian.0.as_slice(),
                    ulps = ULPS
                ),
                "{}: got = {}, want = {}",
                test_case.name,
                point.0,
                test_case.cartesian.0
            );
        });
    }

    #[test]
    fn rotate_must_not_fail() {
        struct TestCase {
            name: &'static str,
            theta: f64,
            axis: CartesianPoint,
            origin: CartesianPoint,
            want: CartesianPoint,
        }

        vec![
            TestCase {
                name: "full rotation on the x axis must not change the y point",
                theta: 2. * PI,
                axis: CartesianPoint::new(1., 0., 0.),
                origin: CartesianPoint::new(0., 1., 0.),
                want: CartesianPoint::new(0., 1., 0.),
            },
            TestCase {
                name: "half of a whole rotation on the x axis must change the y point",
                theta: PI,
                axis: CartesianPoint::new(1., 0., 0.),
                origin: CartesianPoint::new(0., 1., 0.),
                want: CartesianPoint::new(0., -1., 0.),
            },
            TestCase {
                name: "a quarter of a whole rotation on the x axis must change the y point",
                theta: FRAC_PI_2,
                axis: CartesianPoint::new(1., 0., 0.),
                origin: CartesianPoint::new(0., 1., 0.),
                want: CartesianPoint::new(0., 0., -1.),
            },
            TestCase {
                name: "full rotation on the z axis must not change the y point",
                theta: 2. * PI,
                axis: CartesianPoint::new(0., 0., 1.),
                origin: CartesianPoint::new(0., 1., 0.),
                want: CartesianPoint::new(0., 1., 0.),
            },
            TestCase {
                name: "half of a whole rotation on the z axis must change the y point",
                theta: PI,
                axis: CartesianPoint::new(0., 0., 1.),
                origin: CartesianPoint::new(0., 1., 0.),
                want: CartesianPoint::new(0., -1., 0.),
            },
            TestCase {
                name: "a quarter of a whole rotation on the z axis must change the y point",
                theta: FRAC_PI_2,
                axis: CartesianPoint::new(0., 0., 1.),
                origin: CartesianPoint::new(0., 1., 0.),
                want: CartesianPoint::new(1., 0., 0.),
            },
            TestCase {
                name: "rotate over itself must not change the point",
                theta: FRAC_PI_2,
                axis: CartesianPoint::new(0., 1., 0.),
                origin: CartesianPoint::new(0., 1., 0.),
                want: CartesianPoint::new(0., 1., 0.),
            },
        ]
        .into_iter()
        .for_each(|test_case| {
            let got: CartesianPoint = test_case.origin.rotate(test_case.axis, test_case.theta);
            assert!(
                approx_eq!(
                    &[f64],
                    got.0.as_slice(),
                    test_case.want.0.as_slice(),
                    epsilon = EPSILON,
                    ulps = 17
                ),
                "{}: got = {}, want = {}",
                test_case.name,
                got.0,
                test_case.want.0
            );
        });
    }
}
