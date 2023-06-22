//! Point definition and implementations.

use std::ops::{Add, Mul, Sub};

const PI: f64 = std::f64::consts::PI;
const FRAC_PI_2: f64 = std::f64::consts::FRAC_PI_2;

/// Represents a point in a three dimentional space using the geographic coordinate
/// system (in radiants).
#[derive(Debug, Default, Clone, Copy)]
pub struct GeographicPoint {
    longitude: f64,
    latitude: f64,
    altitude: f64,
}

impl GeographicPoint {
    /// Calls [`set_longitude`] on self and returns it.
    pub fn with_longitude(mut self, value: f64) -> Self {
        self.set_longitude(value);
        self
    }

    /// Calls [`set_latitude`] on self and returns it.
    pub fn with_latitude(mut self, value: f64) -> Self {
        self.set_latitude(value);
        self
    }

    /// Calls [`set_altitude`] on self and returns it.
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
    /// ## Example
    /// ```
    /// use geoda::point::GeographicPoint;
    /// use std::f64::consts::PI;
    /// use float_cmp::approx_eq;
    ///
    /// let mut point = GeographicPoint::default();
    /// point.set_longitude(PI + 1_f64);
    ///
    /// assert!(approx_eq!(f64, point.longitude(), -PI + 1_f64, ulps = 2));
    /// ```
    pub fn set_longitude(&mut self, value: f64) {
        self.longitude = (-PI..PI)
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
    /// ## Example
    /// ```
    /// use geoda::point::GeographicPoint;
    /// use std::f64::consts::PI;
    /// use float_cmp::approx_eq;
    ///
    /// let mut point = GeographicPoint::default();
    /// point.set_latitude(-5. * PI / 4.);
    ///
    /// assert!(approx_eq!(f64, point.latitude(), PI / 4., ulps = 2));
    /// assert!(approx_eq!(f64, point.longitude(), -PI, ulps = 2));
    /// ```
    pub fn set_latitude(&mut self, value: f64) {
        self.latitude = (-PI / 2.0..PI / 2.)
            .contains(&value)
            .then_some(value)
            .unwrap_or_else(|| {
                // The derivative of sin(x) is cos(x), and so, cos(x) determines the
                // sign of the longitude of the point.
                if value.cos().signum() != self.longitude.signum() {
                    // Increasing the longitude of the point by 180º (π) ensures the
                    // sign is changed while maintaining it in the same pair of
                    // complementary meridians.
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
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use float_cmp::approx_eq;

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
                approx_eq!(f64, point.longitude, test_case.longitude, ulps = 2),
                "{}: {} ±t == {}",
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
                approx_eq!(f64, point.latitude, test_case.latitude, ulps = 2),
                "{}: {} ±t == {}",
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
                longitude: -PI,
            },
            TestCase {
                name: "negative overflowing latidude must change longitude",
                input: -PI,
                longitude: -PI,
            },
        ]
        .into_iter()
        .for_each(|test_case| {
            let point = GeographicPoint::default().with_latitude(test_case.input);
            assert!(
                approx_eq!(f64, point.longitude, test_case.longitude, ulps = 2),
                "{}: {} ±t == {}",
                test_case.name,
                point.longitude,
                test_case.longitude
            );
        });
    }
}
