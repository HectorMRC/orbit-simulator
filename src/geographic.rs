use crate::CartesianPoint;
use std::f64::consts::{FRAC_PI_2, PI};
use std::ops::{Add, Mul, Sub};
use wasm_bindgen::prelude::wasm_bindgen;

/// Represents a point using the geographic system of coordinates.
#[wasm_bindgen]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct GeographicPoint {
    longitude: f64,
    latitude: f64,
    altitude: f64,
}

impl From<CartesianPoint> for GeographicPoint {
    fn from(value: CartesianPoint) -> Self {
        GeographicPoint::from_cartesian(&value)
    }
}

#[wasm_bindgen]
impl GeographicPoint {
    pub fn new(longitude: f64, latitude: f64, altitude: f64) -> Self {
        Self {
            longitude,
            latitude,
            altitude,
        }
    }

    /// Returns the equivalent [`GeographicPoint`] of the given [`CartesianPoint`]
    pub fn from_cartesian(point: &CartesianPoint) -> Self {
        // see: https://en.wikipedia.org/wiki/Spherical_coordinate_system
        let radius = f64::sqrt(point.x().powi(2) + point.y().powi(2) + point.z().powi(2));

        let theta = match (point.x(), point.y(), point.z()) {
            (x, y, z) if z > 0. => f64::atan(f64::sqrt(x.powi(2) + y.powi(2)) / z),
            (x, y, z) if z < 0. => PI + f64::atan(f64::sqrt(x.powi(2) + y.powi(2)) / z),
            (x, y, z) if z == 0. && x * y != 0. => FRAC_PI_2,
            (x, y, z) if x == y && y == z => FRAC_PI_2, // fallback value

            _ => FRAC_PI_2, // fallback value
        };

        let phi = match (point.x(), point.y()) {
            (x, y) if x > 0. => f64::atan(y / x),
            (x, y) if x < 0. && y >= 0. => f64::atan(y / x) + PI,
            (x, y) if x < 0. && y < 0. => f64::atan(y / x) - PI,
            (x, y) if x == 0. && y > 0. => FRAC_PI_2,
            (x, y) if x == 0. && y < 0. => -FRAC_PI_2,
            (x, y) if x == 0. && y == 0. => 0., // fallback value

            _ => 0., // fallback value
        };

        Self::default()
            .with_longitude(phi)
            .with_latitude(FRAC_PI_2 - theta)
            .with_altitude(radius)
    }

    /// Calls set_longitude on self and returns it.
    pub fn with_longitude(mut self, value: f64) -> Self {
        self.set_longitude(value);
        self
    }

    /// Calls set_latitude on self and returns it.
    pub fn with_latitude(mut self, value: f64) -> Self {
        self.set_latitude(value);
        self
    }

    /// Calls set_altitude on self and returns it.
    pub fn with_altitude(mut self, value: f64) -> Self {
        self.set_altitude(value);
        self
    }

    /// Sets the given longitude (in radiants) to the point.
    ///
    /// ## Definition
    /// Since the longitude of a point on a sphere is the angle east (positive) or
    /// west (negative) in reference of the maridian zero, the longitude value must
    /// be in the range __[-π, +π)__. Any other value will be recomputed in order
    /// to set its equivalent inside the range.
    ///
    /// ### Longitude adjustment
    /// Both boundaries of the longitude range are consecutive, which means that
    /// overflowing one is the same as continuing from the other in the same
    /// direction.
    ///
    /// ## Example
    /// ```
    /// use globe_rs::GeographicPoint;
    /// use std::f64::consts::PI;
    /// use float_cmp::approx_eq;
    ///
    /// let mut point = GeographicPoint::default();
    /// point.set_longitude(PI + 1_f64);
    ///
    /// assert!(approx_eq!(f64, point.longitude(), -PI + 1_f64, ulps = 2));
    /// ```
    pub fn set_longitude(&mut self, value: f64) {
        self.longitude = (-PI..=PI)
            .contains(&value)
            .then_some(value)
            .unwrap_or_else(|| {
                // Both boundaries of the range are consecutive, which means that
                // overflowing one is the same as continuing from the other in the
                // same direction.
                value.add(PI).rem_euclid(2_f64.mul(PI)).sub(PI)
            })
    }

    /// Sets the given latitude (in radiants) to the point.
    ///
    /// ## Definition
    /// Since the latitude of a point on a sphere is the angle between the
    /// equatorial plane and the straight line that passes through that point and
    /// through the center of the sphere, the latitude value must be in the range
    /// __\[-π/2, +π/2\]__. Any other value will be recomputed in order to set its
    /// equivalent inside the range. Notice that this action may recompute the
    /// longitude as well.
    ///
    /// ### Latitude adjustment
    /// Overflowing any of both boundaries of the latitude range behaves like
    /// moving away from that point and getting closer to the oposite one.
    ///
    /// ### Longitude adjustment
    /// Geometrically speaking, meridians are half of a circle going from the north
    /// pole to the south one. The position of each meridian in the perimeter of
    /// the sphere (horizontal axis) is set by the longitude itself. However, this
    /// value may change when the latitude overflows its normalized range. This
    /// happen since exceeding any of its established limits means moving from one
    /// to the other half of the circle on which the meridian is drawn. And
    /// therefore, the longitude gets increased by exactly `π` radiants.
    ///
    /// Of course, this mutation on the longitude only applies when the overflow of
    /// the latitude is not enough to complete a full lap. If it is, the longitude
    /// does not change at all.
    ///
    /// ## Example
    /// ```
    /// use globe_rs::GeographicPoint;
    /// use std::f64::consts::PI;
    /// use float_cmp::approx_eq;
    ///
    /// let mut point = GeographicPoint::default();
    /// point.set_latitude(-5. * PI / 4.);
    ///
    /// assert!(approx_eq!(f64, point.latitude(), PI / 4., ulps = 2));
    /// assert!(approx_eq!(f64, point.longitude(), PI, ulps = 2));
    /// ```
    pub fn set_latitude(&mut self, value: f64) {
        self.latitude = (-FRAC_PI_2..=FRAC_PI_2)
            .contains(&value)
            .then_some(value)
            .unwrap_or_else(|| {
                // The derivative of sin(x) is cos(x), and so, cos(x) determines if
                // the sign of the longitude of the point must change.
                if value.cos().signum() != self.latitude.cos().signum() {
                    // Increasing the longitude of the point by π radiants (180º)
                    // ensures the sign is changed while maintaining it in the same
                    // pair of complementary meridians.
                    self.set_longitude(self.longitude.add(PI));
                }

                value.sin().asin()
            });
    }

    /// Sets the given altitude to the point.
    pub fn set_altitude(&mut self, value: f64) {
        self.altitude = value;
    }

    /// Returns the longitude (in radiants) of the point.
    pub fn longitude(&self) -> f64 {
        self.longitude
    }

    /// Returns the latitude (in radiants) of the point.
    pub fn latitude(&self) -> f64 {
        self.latitude
    }

    /// Returns the altitude (in radiants) of the point.
    pub fn altitude(&self) -> f64 {
        self.altitude
    }

    /// Returns the result of dividing `π` to the longitude of the point, resulting
    /// in a value in the range __[-1.0, 1.0)__
    pub fn long_ratio(&self) -> f64 {
        self.longitude / PI
    }

    /// Returns the result of dividing `π/2` to the latitude of the point, resulting
    /// in a value in the range __\[-1.0, 1.0\]__
    pub fn lat_ratio(&self) -> f64 {
        self.latitude / FRAC_PI_2
    }

    /// Computes the [great-circle distance](https://en.wikipedia.org/wiki/Great-circle_distance) from self to the given point (in radiants).
    pub fn distance(&self, other: &GeographicPoint) -> f64 {
        (self.latitude().sin() * other.latitude().sin()
            + self.latitude().cos()
                * other.latitude().cos()
                * (self.longitude() - other.longitude()).abs().cos())
        .acos()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    const ULPS: i64 = 2;

    #[test]
    fn longitude_must_not_exceed_boundaries() {
        struct TestCase {
            name: &'static str,
            input: f64,
            longitude: f64,
        }

        vec![
            TestCase {
                name: "positive longitude value must not change",
                input: 1.,
                longitude: 1.,
            },
            TestCase {
                name: "negative longitude value must not change",
                input: -3.,
                longitude: -3.,
            },
            TestCase {
                name: "positive overflowing longitude must change",
                input: PI + 1.,
                longitude: -PI + 1.,
            },
            TestCase {
                name: "negative overflowing longitude must change",
                input: -PI - 1.,
                longitude: PI - 1.,
            },
        ]
        .into_iter()
        .for_each(|test_case| {
            let point = GeographicPoint::default().with_longitude(test_case.input);
            assert!(
                approx_eq!(f64, point.longitude, test_case.longitude, ulps = ULPS),
                "{}: {} ±ε = {}",
                test_case.name,
                point.longitude,
                test_case.longitude
            );
        });
    }

    #[test]
    fn latitude_must_not_exceed_boundaries() {
        struct TestCase {
            name: &'static str,
            input: f64,
            latitude: f64,
        }

        vec![
            TestCase {
                name: "positive latitude value must not change",
                input: 1.,
                latitude: 1.,
            },
            TestCase {
                name: "negative latitude value must not change",
                input: -1.,
                latitude: -1.,
            },
            TestCase {
                name: "positive overflowing latitude must change",
                input: 7. * PI / 4.,
                latitude: -PI / 4.,
            },
            TestCase {
                name: "negative overflowing latidude must change",
                input: -7. * PI / 4.,
                latitude: PI / 4.,
            },
        ]
        .into_iter()
        .for_each(|test_case| {
            let point = GeographicPoint::default().with_latitude(test_case.input);
            assert!(
                approx_eq!(f64, point.latitude, test_case.latitude, ulps = ULPS),
                "{}: {} ±ε = {}",
                test_case.name,
                point.latitude,
                test_case.latitude
            );
        });
    }

    #[test]
    fn longitude_may_change_based_on_latitude() {
        struct TestCase {
            name: &'static str,
            input: f64,
            longitude: f64,
        }

        vec![
            TestCase {
                name: "positive latitude value must not change longitude",
                input: 1.,
                longitude: 0.,
            },
            TestCase {
                name: "negative latitude value must not change longitude",
                input: -1.,
                longitude: 0.,
            },
            TestCase {
                name: "positive overflowing latitude must not change longitude",
                input: 7. * PI / 4.,
                longitude: 0.,
            },
            TestCase {
                name: "negative overflowing latidude must not change longitude",
                input: -7. * PI / 4.,
                longitude: 0.,
            },
            TestCase {
                name: "positive overflowing latitude must change longitude",
                input: 5. * PI / 4.,
                longitude: PI,
            },
            TestCase {
                name: "negative overflowing latidude must change longitude",
                input: -PI,
                longitude: PI,
            },
        ]
        .into_iter()
        .for_each(|test_case| {
            let point = GeographicPoint::default().with_latitude(test_case.input);
            assert!(
                approx_eq!(f64, point.longitude, test_case.longitude, ulps = ULPS),
                "{}: {} ±ε = {}",
                test_case.name,
                point.longitude,
                test_case.longitude
            );
        });
    }

    #[test]
    fn long_ratio_must_not_exceed_boundaries() {
        struct TestCase {
            name: &'static str,
            longitude: f64,
            ratio: f64,
        }

        vec![
            TestCase {
                name: "zero longitude must be equals to zero",
                longitude: 0.,
                ratio: 0.,
            },
            TestCase {
                name: "positive longitude boundary must be positive",
                longitude: PI,
                ratio: 1.,
            },
            TestCase {
                name: "arbitrary positive longitude must be positive",
                longitude: FRAC_PI_2,
                ratio: 0.5,
            },
            TestCase {
                name: "negative longitude boundary must equals to negative one",
                longitude: -PI,
                ratio: -1.,
            },
            TestCase {
                name: "arbitrary negative longitude must be negative",
                longitude: -FRAC_PI_2,
                ratio: -0.5,
            },
        ]
        .into_iter()
        .for_each(|test_case| {
            let point = GeographicPoint::default().with_longitude(test_case.longitude);
            assert!(
                approx_eq!(f64, point.long_ratio(), test_case.ratio, ulps = ULPS),
                "{}: {} ±ε = {}",
                test_case.name,
                point.long_ratio(),
                test_case.ratio
            );
        });
    }

    #[test]
    fn lat_ratio_must_not_exceed_boundaries() {
        struct TestCase {
            name: &'static str,
            latitude: f64,
            ratio: f64,
        }

        vec![
            TestCase {
                name: "zero latitude must be equals to zero",
                latitude: 0.,
                ratio: 0.,
            },
            TestCase {
                name: "positive latitude boundary must equals to one",
                latitude: FRAC_PI_2,
                ratio: 1.,
            },
            TestCase {
                name: "arbitrary positive latitude must be positive",
                latitude: FRAC_PI_2 / 2.,
                ratio: 0.5,
            },
            TestCase {
                name: "negative latitude boundary must equals to negative one",
                latitude: -FRAC_PI_2,
                ratio: -1.,
            },
            TestCase {
                name: "arbitrary negative latitude must be negative",
                latitude: -FRAC_PI_2 / 2.,
                ratio: -0.5,
            },
        ]
        .into_iter()
        .for_each(|test_case| {
            let point = GeographicPoint::default().with_latitude(test_case.latitude);
            assert!(
                approx_eq!(f64, point.lat_ratio(), test_case.ratio, ulps = ULPS),
                "{}: {} ±ε = {}",
                test_case.name,
                point.lat_ratio(),
                test_case.ratio
            );
        });
    }

    #[test]
    fn basic_operations_must_be_consistent() {
        let mut point = GeographicPoint::default()
            .with_longitude(-FRAC_PI_2)
            .with_latitude(FRAC_PI_2 / 2.);

        point.set_latitude(point.latitude().add(PI));

        assert!(
            approx_eq!(f64, point.longitude(), FRAC_PI_2, ulps = ULPS),
            "longitude must switch to positive: {} ±ε = {}",
            point.longitude(),
            FRAC_PI_2
        );

        assert!(
            approx_eq!(f64, point.latitude(), -FRAC_PI_2 / 2., ulps = ULPS),
            "latitude must switch to negative: {} ±ε = {}",
            point.latitude(),
            -FRAC_PI_2 / 2.
        );
    }

    #[test]
    fn geographic_from_cartesian_must_not_fail() {
        struct TestCase {
            name: &'static str,
            geographic: GeographicPoint,
            cartesian: CartesianPoint,
        }

        vec![
            TestCase {
                name: "north point",
                geographic: GeographicPoint::default()
                    .with_latitude(FRAC_PI_2)
                    .with_altitude(1.),
                cartesian: CartesianPoint::new(0., 0., 1.),
            },
            TestCase {
                name: "south point",
                geographic: GeographicPoint::default()
                    .with_latitude(-FRAC_PI_2)
                    .with_altitude(1.),
                cartesian: CartesianPoint::new(0., 0., -1.),
            },
            TestCase {
                name: "east point",
                geographic: GeographicPoint::default()
                    .with_longitude(FRAC_PI_2)
                    .with_altitude(1.),
                cartesian: CartesianPoint::new(0., 1., 0.),
            },
            TestCase {
                name: "weast point",
                geographic: GeographicPoint::default()
                    .with_longitude(-FRAC_PI_2)
                    .with_altitude(1.),
                cartesian: CartesianPoint::new(0., -1., 0.),
            },
            TestCase {
                name: "front point",
                geographic: GeographicPoint::default().with_altitude(1.),
                cartesian: CartesianPoint::new(1., 0., 0.),
            },
            TestCase {
                name: "back point",
                geographic: GeographicPoint::default()
                    .with_longitude(PI)
                    .with_altitude(1.),
                cartesian: CartesianPoint::new(-1., 0., 0.),
            },
        ]
        .into_iter()
        .for_each(|test_case| {
            let point = GeographicPoint::from(test_case.cartesian);
            assert!(
                approx_eq!(
                    f64,
                    point.longitude(),
                    test_case.geographic.longitude(),
                    ulps = ULPS
                ),
                "{}: longitude {:#?} ±ε == {:#?}",
                test_case.name,
                point.longitude(),
                test_case.geographic.longitude(),
            );

            assert!(
                approx_eq!(
                    f64,
                    point.latitude(),
                    test_case.geographic.latitude(),
                    ulps = ULPS
                ),
                "{}: latitude {:#?} ±ε == {:#?}",
                test_case.name,
                point.latitude(),
                test_case.geographic.latitude(),
            );

            assert!(
                approx_eq!(
                    f64,
                    point.altitude(),
                    test_case.geographic.altitude(),
                    ulps = ULPS
                ),
                "{}: altitude {:#?} ±ε == {:#?}",
                test_case.name,
                point.altitude(),
                test_case.geographic.altitude(),
            );
        });
    }

    #[test]
    fn distance_must_not_fail() {
        struct TestCase<'a> {
            name: &'a str,
            from: GeographicPoint,
            to: GeographicPoint,
            distance: f64,
        }

        vec![
            TestCase {
                name: "Same point must be zero",
                from: GeographicPoint::default(),
                to: GeographicPoint::default(),
                distance: 0.,
            },
            TestCase {
                name: "Oposite points in the horizontal",
                from: GeographicPoint::default(),
                to: GeographicPoint::default().with_longitude(-PI),
                distance: PI,
            },
            TestCase {
                name: "Oposite points in the vertical",
                from: GeographicPoint::default().with_latitude(FRAC_PI_2),
                to: GeographicPoint::default().with_latitude(-FRAC_PI_2),
                distance: PI,
            },
        ]
        .into_iter()
        .for_each(|test_case| {
            let got = test_case.from.distance(&test_case.to);

            assert!(
                approx_eq!(f64, got, test_case.distance, ulps = ULPS),
                "{}: distance {:#?} ±ε == {:#?}",
                test_case.name,
                got,
                test_case.distance,
            )
        });
    }
}
