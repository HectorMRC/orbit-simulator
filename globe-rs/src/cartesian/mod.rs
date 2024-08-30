use std::{
    f64::consts::{FRAC_PI_2, PI},
    ops::{Add, AddAssign, Neg},
};

use nalgebra::{iter::MatrixIter, ArrayStorage, Const, Vector3};
use transform::Transform;

use crate::geographic;

pub mod shape;
pub mod transform;

/// Coordinates according to the cartesian system of coordinates.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Coords(Vector3<f64>);

impl<T> From<T> for Coords
where
    T: Into<Vector3<f64>>,
{
    fn from(value: T) -> Self {
        Coords(value.into())
    }
}

impl<'a> IntoIterator for &'a Coords {
    type Item = &'a f64;

    type IntoIter = MatrixIter<'a, f64, Const<3>, Const<1>, ArrayStorage<f64, 3, 1>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Neg for Coords {
    type Output = Coords;

    fn neg(self) -> Self::Output {
        Self::from(-self.0)
    }
}

impl Add for Coords {
    type Output = Coords;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for Coords {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl From<geographic::Coords> for Coords {
    fn from(point: geographic::Coords) -> Self {
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

impl Coords {
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
    pub fn distance(&self, rhs: &Coords) -> f64 {
        self.0.metric_distance(&rhs.0)
    }

    /// Performs the cartesian product between self and the given point.
    pub fn cross(&self, other: &Coords) -> Self {
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
        cartesian::Coords,
        geographic::{self, Latitude, Longitude},
    };

    #[test]
    fn cartesian_from_geographic_must_not_fail() {
        struct Test {
            name: &'static str,
            input: geographic::Coords,
            output: Coords,
        }

        vec![
            Test {
                name: "north point",
                input: geographic::Coords::default().with_latitude(Latitude::from(FRAC_PI_2)),
                output: Coords::from([0., 0., 1.]),
            },
            Test {
                name: "south point",
                input: geographic::Coords::default().with_latitude(Latitude::from(-FRAC_PI_2)),
                output: Coords::from([0., 0., -1.]),
            },
            Test {
                name: "east point",
                input: geographic::Coords::default().with_longitude(Longitude::from(FRAC_PI_2)),
                output: Coords::from([0., 1., 0.]),
            },
            Test {
                name: "weast point",
                input: geographic::Coords::default().with_longitude(Longitude::from(-FRAC_PI_2)),
                output: Coords::from([0., -1., 0.]),
            },
            Test {
                name: "front point",
                input: geographic::Coords::default(),
                output: Coords::from([1., 0., 0.]),
            },
            Test {
                name: "back point as negative bound",
                input: geographic::Coords::default().with_longitude(Longitude::from(-PI)),
                output: Coords::from([-1., 0., 0.]),
            },
            Test {
                name: "back point as positive bound",
                input: geographic::Coords::default().with_longitude(Longitude::from(PI)),
                output: Coords::from([-1., 0., 0.]),
            },
        ]
        .into_iter()
        .for_each(|test| {
            let from = Coords::from(test.input);
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
            input: Coords,
            output: Coords,
        }

        vec![
            Test {
                name: "lenght of unit vector must be 1 at x axis",
                input: Coords::from([2., 0., 0.]),
                output: Coords::from([1., 0., 0.]),
            },
            Test {
                name: "lenght of unit vector must be 1 at y axis",
                input: Coords::from([0., 3., 0.]),
                output: Coords::from([0., 1., 0.]),
            },
            Test {
                name: "lenght of unit vector must be 1 at z axis",
                input: Coords::from([0., 0., -4.]),
                output: Coords::from([0., 0., -1.]),
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
