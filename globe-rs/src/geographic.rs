use std::f64::consts::{FRAC_PI_2, PI};

use crate::{cartesian, PositiveFloat};

/// Represents the horizontal axis in a geographic system of coordinates.
///
/// ## Definition
/// Since the longitude of a point on a sphere is the angle east (positive) or west (negative) in
/// reference of the maridian zero, the longitude value must be in the range __[-π, +π)__. Any
/// other value will be computed in order to set its equivalent inside the range.
///
/// ### Overflow
/// Both boundaries of the longitude range are consecutive, which means that overflowing one is the
/// same as continuing from the other in the same direction.
///
/// ## Example
/// ```
/// use globe_rs::geographic::Longitude;
/// use std::f64::consts::PI;
///
/// assert_eq!(
///     Longitude::from(PI + 1_f64),
///     Longitude::from(-PI + 1_f64)
/// );
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Longitude(f64);

impl From<f64> for Longitude {
    fn from(value: f64) -> Self {
        Self(
            (-PI..=PI)
                .contains(&value)
                .then_some(value)
                .unwrap_or_else(|| {
                    // Both boundaries of the range are consecutive, which means that
                    // overflowing one is the same as continuing from the other in the
                    // same direction.
                    (value + PI).rem_euclid(2_f64 * PI) - PI
                }),
        )
    }
}

impl From<Longitude> for f64 {
    fn from(value: Longitude) -> Self {
        value.0
    }
}

impl From<cartesian::Coords> for Longitude {
    /// Computes the [Longitude] of the given [Cartesian] as specified by the [Spherical
    /// coordinate system](https://en.wikipedia.org/wiki/Spherical_coordinate_system).
    fn from(point: cartesian::Coords) -> Self {
        match (point.x(), point.y()) {
            (x, y) if x > 0. => (y / x).atan(),
            (x, y) if x < 0. && y >= 0. => (y / x).atan() + PI,
            (x, y) if x < 0. && y < 0. => (y / x).atan() - PI,
            (x, y) if x == 0. && y > 0. => FRAC_PI_2,
            (x, y) if x == 0. && y < 0. => -FRAC_PI_2,
            (x, y) if x == 0. && y == 0. => 0., // fallback value

            _ => 0., // fallback value
        }
        .into()
    }
}

impl Longitude {
    /// Returns the [f64] representation of tha longitude in the range of __[-1.0, 1.0)__,
    /// resulting from dividing self with `π`.
    pub fn normal(&self) -> f64 {
        self.0 / PI
    }
}

/// Represents the vertical axis in a geographic system of coordinates.
///
/// ## Definition
/// Since the latitude of a point on a sphere is the angle between the equatorial plane and the
/// straight line that goes through that point and the center of the sphere, the latitude value
/// must be in the range __\[-π/2, +π/2\]__. Any other value must be computed in order to set
/// its equivalent inside the range.
///
/// ### Overflow
/// Overflowing any of both boundaries of the latitude range behaves like moving away from that
/// boundary and getting closer to the oposite one.
///
/// ## Example
/// ```
/// use globe_rs::geographic::Latitude;
/// use std::f64::consts::PI;
///
/// let overflowing_latitude = Latitude::from(-5. * PI / 4.);
/// let equivalent_latitude = Latitude::from(PI / 4.);
///
/// // due precision error both values may not be exactly the same  
/// let abs_error = 0.0000000000000002;
///
/// assert!(
///     (f64::from(equivalent_latitude) - f64::from(overflowing_latitude)).abs() <= abs_error,
///     "the overflowing latitude should be as the equivalent latitude ± e"
/// );
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Latitude(f64);

impl From<f64> for Latitude {
    fn from(value: f64) -> Self {
        Self(if (-FRAC_PI_2..=FRAC_PI_2).contains(&value) {
            value
        } else {
            value.sin().asin()
        })
    }
}

impl From<Latitude> for f64 {
    fn from(value: Latitude) -> Self {
        value.0
    }
}

impl From<cartesian::Coords> for Latitude {
    /// Computes the [Latitude] of the given [Cartesian] as specified by the [Spherical
    /// coordinate system](https://en.wikipedia.org/wiki/Spherical_coordinate_system).
    fn from(point: cartesian::Coords) -> Self {
        let theta = match (point.x(), point.y(), point.z()) {
            (x, y, z) if z > 0. => f64::atan(f64::sqrt(x.powi(2) + y.powi(2)) / z),
            (x, y, z) if z < 0. => PI + f64::atan(f64::sqrt(x.powi(2) + y.powi(2)) / z),
            (x, y, z) if z == 0. && x * y != 0. => FRAC_PI_2,
            // (x, y, z) if x == y && y == z => FRAC_PI_2, // fallback value
            _ => FRAC_PI_2, // fallback value
        };

        (FRAC_PI_2 - theta).into()
    }
}

impl Latitude {
    /// Returns the [f64] representation of tha latitude in the range of __[-1.0, 1.0)__, resulting
    /// from dividing self with `π/2`.
    pub fn normal(&self) -> f64 {
        self.0 / FRAC_PI_2
    }
}

/// Represents the radius in a geographic system of coordinates.
///
/// ## Definition
/// Since the altitude of a point on a sphere is the distance between that point and the center of
/// the sphere, the altitude value must be positive. The absolute of any other value must be
/// computed in order to get a proper radius notation.
///
/// ## Example
/// ```
/// use globe_rs::geographic::Altitude;
///
/// assert_eq!(
///     Altitude::from(-1.56),
///     Altitude::from(1.56)
/// );
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Altitude(PositiveFloat);

impl From<f64> for Altitude {
    fn from(value: f64) -> Self {
        Self(value.into())
    }
}

impl From<Altitude> for f64 {
    fn from(value: Altitude) -> Self {
        value.0 .0
    }
}

impl From<cartesian::Coords> for Altitude {
    /// Computes the [Altitude] of the given [Cartesian] as specified by the [Spherical
    /// coordinate system](https://en.wikipedia.org/wiki/Spherical_coordinate_system).
    fn from(point: cartesian::Coords) -> Self {
        f64::sqrt(point.x().powi(2) + point.y().powi(2) + point.z().powi(2)).into()
    }
}

/// Coordinates accodring to the geographical system of coordinates.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Coords {
    pub longitude: Longitude,
    pub latitude: Latitude,
    pub altitude: Altitude,
}

impl From<cartesian::Coords> for Coords {
    fn from(point: cartesian::Coords) -> Self {
        Self::default()
            .with_longitude(point.into())
            .with_latitude(point.into())
            .with_altitude(point.into())
    }
}

impl Coords {
    pub fn with_longitude(mut self, longitude: Longitude) -> Self {
        self.longitude = longitude;
        self
    }

    pub fn with_latitude(mut self, latitude: Latitude) -> Self {
        self.latitude = latitude;
        self
    }

    pub fn with_altitude(mut self, altitude: Altitude) -> Self {
        self.altitude = altitude;
        self
    }

    /// Computes the [great-circle distance](https://en.wikipedia.org/wiki/Great-circle_distance)
    /// from self to the given point (in radiants).
    pub fn distance(&self, other: &Coords) -> f64 {
        let prod_latitude_sin = f64::from(self.latitude).sin() * f64::from(other.latitude).sin();
        let prod_latitude_cos = f64::from(self.latitude).cos() * f64::from(other.latitude).cos();
        let longitude_diff = (f64::from(self.longitude) - f64::from(other.longitude)).abs();

        (prod_latitude_sin + prod_latitude_cos * longitude_diff.cos()).acos()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, PI};

    use crate::{
        cartesian,
        geographic::{Altitude, Coords, Latitude, Longitude},
        tests::approx_eq,
    };

    #[test]
    fn longitude_must_not_exceed_boundaries() {
        struct Test {
            name: &'static str,
            input: f64,
            output: f64,
        }

        vec![
            Test {
                name: "positive longitude value must not change",
                input: 1.,
                output: 1.,
            },
            Test {
                name: "negative longitude value must not change",
                input: -3.,
                output: -3.,
            },
            Test {
                name: "positive overflowing longitude must change",
                input: PI + 1.,
                output: -PI + 1.,
            },
            Test {
                name: "negative overflowing longitude must change",
                input: -PI - 1.,
                output: PI - 1.,
            },
        ]
        .into_iter()
        .for_each(|test| {
            let longitude: f64 = Longitude::from(test.input).into();

            assert_eq!(
                longitude, test.output,
                "{}: got longitude = {}, want {}",
                test.name, longitude, test.output
            );
        });
    }

    #[test]
    fn normal_longitude_must_not_exceed_boundaries() {
        struct Test {
            name: &'static str,
            input: f64,
            output: f64,
        }

        vec![
            Test {
                name: "zero longitude must be equals to zero",
                input: 0.,
                output: 0.,
            },
            Test {
                name: "positive longitude boundary must equals to positive one",
                input: PI,
                output: 1.,
            },
            Test {
                name: "arbitrary positive longitude must be positive",
                input: FRAC_PI_2,
                output: 0.5,
            },
            Test {
                name: "negative longitude boundary must equals to negative one",
                input: -PI,
                output: -1.,
            },
            Test {
                name: "arbitrary negative longitude must be negative",
                input: -FRAC_PI_2,
                output: -0.5,
            },
        ]
        .into_iter()
        .for_each(|test| {
            let normal = Longitude::from(test.input).normal();

            assert_eq!(
                normal, test.output,
                "{}: got normal = {}, want {}",
                test.name, normal, test.output
            );
        });
    }

    #[test]
    fn latitude_must_not_exceed_boundaries() {
        const ABS_ERROR: f64 = 0.0000000000000003;

        struct Test {
            name: &'static str,
            input: f64,
            output: f64,
        }

        vec![
            Test {
                name: "positive latitude value must not change",
                input: 1.,
                output: 1.,
            },
            Test {
                name: "negative latitude value must not change",
                input: -1.,
                output: -1.,
            },
            Test {
                name: "positive overflowing latitude must change",
                input: 7. * PI / 4.,
                output: -PI / 4.,
            },
            Test {
                name: "negative overflowing latidude must change",
                input: -7. * PI / 4.,
                output: PI / 4.,
            },
        ]
        .into_iter()
        .for_each(|test| {
            let latitude: f64 = Latitude::from(test.input).into();

            assert!(
                approx_eq(latitude, test.output, ABS_ERROR),
                "{}: got latitude = {}, want {}",
                test.name,
                latitude,
                test.output
            );
        });
    }

    #[test]
    fn normal_latitude_must_not_exceed_boundaries() {
        struct Test {
            name: &'static str,
            input: f64,
            output: f64,
        }

        vec![
            Test {
                name: "zero latitude must be equals to zero",
                input: 0.,
                output: 0.,
            },
            Test {
                name: "positive latitude boundary must equals to one",
                input: FRAC_PI_2,
                output: 1.,
            },
            Test {
                name: "arbitrary positive latitude must be positive",
                input: FRAC_PI_2 / 2.,
                output: 0.5,
            },
            Test {
                name: "negative latitude boundary must equals to negative one",
                input: -FRAC_PI_2,
                output: -1.,
            },
            Test {
                name: "arbitrary negative latitude must be negative",
                input: -FRAC_PI_2 / 2.,
                output: -0.5,
            },
        ]
        .into_iter()
        .for_each(|test| {
            let normal = Latitude::from(test.input).normal();

            assert_eq!(
                normal, test.output,
                "{}: got normal = {}, want {}",
                test.name, normal, test.output
            );
        });
    }

    #[test]
    fn geographic_from_cartesian_must_not_fail() {
        struct Test {
            name: &'static str,
            input: cartesian::Coords,
            output: Coords,
        }

        vec![
            Test {
                name: "north point",
                input: cartesian::Coords::from([0., 0., 1.]),
                output: Coords::default()
                    .with_latitude(Latitude::from(FRAC_PI_2))
                    .with_altitude(Altitude::from(1.)),
            },
            Test {
                name: "south point",
                input: cartesian::Coords::from([0., 0., -1.]),
                output: Coords::default()
                    .with_latitude(Latitude::from(-FRAC_PI_2))
                    .with_altitude(Altitude::from(1.)),
            },
            Test {
                name: "east point",
                input: cartesian::Coords::from([0., 1., 0.]),
                output: Coords::default()
                    .with_longitude(Longitude::from(FRAC_PI_2))
                    .with_altitude(Altitude::from(1.)),
            },
            Test {
                name: "weast point",
                input: cartesian::Coords::from([0., -1., 0.]),
                output: Coords::default()
                    .with_longitude(Longitude::from(-FRAC_PI_2))
                    .with_altitude(Altitude::from(1.)),
            },
            Test {
                name: "front point",
                input: cartesian::Coords::from([1., 0., 0.]),
                output: Coords::default().with_altitude(Altitude::from(1.)),
            },
            Test {
                name: "back point",
                input: cartesian::Coords::from([-1., 0., 0.]),
                output: Coords::default()
                    .with_longitude(Longitude::from(PI))
                    .with_altitude(Altitude::from(1.)),
            },
        ]
        .into_iter()
        .for_each(|test| {
            let point = Coords::from(test.input);

            assert_eq!(
                point.longitude,
                test.output.longitude,
                "{}: got longitude = {}, want {}",
                test.name,
                f64::from(point.longitude),
                f64::from(test.output.longitude),
            );

            assert_eq!(
                point.latitude,
                test.output.latitude,
                "{}: got latitude = {}, want {}",
                test.name,
                f64::from(point.latitude),
                f64::from(test.output.latitude),
            );

            assert_eq!(
                point.altitude,
                test.output.altitude,
                "{}: got altitude = {}, want {}",
                test.name,
                f64::from(point.altitude),
                f64::from(test.output.altitude),
            );
        });
    }

    #[test]
    fn distance_must_not_fail() {
        struct Test<'a> {
            name: &'a str,
            from: Coords,
            to: Coords,
            distance: f64,
        }

        vec![
            Test {
                name: "Same point must be zero",
                from: Coords::default(),
                to: Coords::default(),
                distance: 0.,
            },
            Test {
                name: "Oposite points in the horizontal",
                from: Coords::default(),
                to: Coords::default().with_longitude(Longitude::from(-PI)),
                distance: PI,
            },
            Test {
                name: "Oposite points in the vertical",
                from: Coords::default().with_latitude(Latitude::from(FRAC_PI_2)),
                to: Coords::default().with_latitude(Latitude::from(-FRAC_PI_2)),
                distance: PI,
            },
        ]
        .into_iter()
        .for_each(|test| {
            let distance = test.from.distance(&test.to);

            assert_eq!(
                distance, test.distance,
                "{}: distance {} ± e == {}",
                test.name, distance, test.distance,
            )
        });
    }
}
