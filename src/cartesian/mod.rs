use std::{
    f64::consts::{FRAC_PI_2, PI},
    ops::Neg,
};

use nalgebra::{iter::MatrixIter, ArrayStorage, Const, Vector3};
use transform::Transform;

use crate::geographic::Geographic;

pub mod shape;
pub mod transform;

/// An arbitrary point in space using the cartesian system of coordinates.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Cartesian(Vector3<f64>);

impl<T> From<T> for Cartesian
where
    T: Into<Vector3<f64>>,
{
    fn from(value: T) -> Self {
        Cartesian(value.into())
    }
}

impl<'a> IntoIterator for &'a Cartesian {
    type Item = &'a f64;

    type IntoIter = MatrixIter<'a, f64, Const<3>, Const<1>, ArrayStorage<f64, 3, 1>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Neg for Cartesian {
    type Output = Cartesian;

    fn neg(self) -> Self::Output {
        Self::from(-self.0)
    }
}

impl From<Geographic> for Cartesian {
    fn from(point: Geographic) -> Self {
        let radial_distance = match point.altitude.into() {
            altitude if altitude == 0. => 1.,
            altitude => altitude,
        };

        let theta = FRAC_PI_2 - f64::from(point.latitude);
        let phi = point.longitude.into();

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

impl Cartesian {
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

    /// Returns the [Cartesian] representing the unitary vector of self.
    pub fn unit(&self) -> Self {
        self.0.normalize().into()
    }

    /// Returns the distance between self and the given point.
    pub fn distance(&self, rhs: &Cartesian) -> f64 {
        self.0.metric_distance(&rhs.0)
    }

    /// Performs the cartesian product between self and the given point.
    pub fn cross(&self, other: &Cartesian) -> Self {
        self.0.cross(&other.0).into()
    }

    /// Returns the distance of the point relative to the origin of coordinates.
    pub fn magnitude(&self) -> f64 {
        self.0.magnitude()
    }

    /// Performs the given transformation over self.
    pub fn transform<T: Transform>(self, transformation: T) -> Self {
        transformation.transform(self)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, PI};

    use crate::{
        cartesian::Cartesian,
        geographic::{Geographic, Latitude, Longitude},
    };

    #[test]
    fn cartesian_from_geographic_must_not_fail() {
        struct Test {
            name: &'static str,
            input: Geographic,
            output: Cartesian,
        }

        vec![
            Test {
                name: "north point",
                input: Geographic::default().with_latitude(Latitude::from(FRAC_PI_2)),
                output: Cartesian::from([0., 0., 1.]),
            },
            Test {
                name: "south point",
                input: Geographic::default().with_latitude(Latitude::from(-FRAC_PI_2)),
                output: Cartesian::from([0., 0., -1.]),
            },
            Test {
                name: "east point",
                input: Geographic::default().with_longitude(Longitude::from(FRAC_PI_2)),
                output: Cartesian::from([0., 1., 0.]),
            },
            Test {
                name: "weast point",
                input: Geographic::default().with_longitude(Longitude::from(-FRAC_PI_2)),
                output: Cartesian::from([0., -1., 0.]),
            },
            Test {
                name: "front point",
                input: Geographic::default(),
                output: Cartesian::from([1., 0., 0.]),
            },
            Test {
                name: "back point as negative bound",
                input: Geographic::default().with_longitude(Longitude::from(-PI)),
                output: Cartesian::from([-1., 0., 0.]),
            },
            Test {
                name: "back point as positive bound",
                input: Geographic::default().with_longitude(Longitude::from(PI)),
                output: Cartesian::from([-1., 0., 0.]),
            },
        ]
        .into_iter()
        .for_each(|test| {
            let from = Cartesian::from(test.input);
            let from = from;
            let point = from;
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
            input: Cartesian,
            output: Cartesian,
        }

        vec![
            Test {
                name: "lenght of unit vector must be 1 at x axis",
                input: Cartesian::from([2., 0., 0.]),
                output: Cartesian::from([1., 0., 0.]),
            },
            Test {
                name: "lenght of unit vector must be 1 at y axis",
                input: Cartesian::from([0., 3., 0.]),
                output: Cartesian::from([0., 1., 0.]),
            },
            Test {
                name: "lenght of unit vector must be 1 at z axis",
                input: Cartesian::from([0., 0., -4.]),
                output: Cartesian::from([0., 0., -1.]),
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
