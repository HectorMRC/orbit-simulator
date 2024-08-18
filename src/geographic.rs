use std::f64::consts::{FRAC_PI_2, PI};

use crate::CartesianPoint;

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
/// use globe_rs::Longitude;
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

impl From<CartesianPoint> for Longitude {
    /// Computes the [Longitude] of the given [CartesianPoint] as specified by the [Spherical
    /// coordinate system](https://en.wikipedia.org/wiki/Spherical_coordinate_system).
    fn from(point: CartesianPoint) -> Self {
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
    /// Returns the longitude value as an f64.
    pub fn as_f64(&self) -> f64 {
        self.0
    }

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
/// use globe_rs::Latitude;
/// use std::f64::consts::PI;
///
/// let overflowing_latitude = Latitude::from(-5. * PI / 4.);
/// let equivalent_latitude = Latitude::from(PI / 4.);
///
/// // due precision error both values may not be exactly the same  
/// let epsilon = 0.0000000000000001;
///
/// assert!(
///     equivalent_latitude.as_f64() + epsilon >= overflowing_latitude.as_f64() &&
///     equivalent_latitude.as_f64() - epsilon <= overflowing_latitude.as_f64(),
///     "the overflowing latitude should be as the equivalent latitude ± ε"
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

impl From<CartesianPoint> for Latitude {
    /// Computes the [Latitude] of the given [CartesianPoint] as specified by the [Spherical
    /// coordinate system](https://en.wikipedia.org/wiki/Spherical_coordinate_system).
    fn from(point: CartesianPoint) -> Self {
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
    /// Returns the latitude value as an f64.
    pub fn as_f64(&self) -> f64 {
        self.0
    }

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
/// use globe_rs::Altitude;
///
/// assert_eq!(
///     Altitude::from(-1.56),
///     Altitude::from(1.56)
/// );
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Altitude(f64);

impl From<f64> for Altitude {
    fn from(value: f64) -> Self {
        Self(value.abs())
    }
}

impl From<CartesianPoint> for Altitude {
    /// Computes the [Altitude] of the given [CartesianPoint] as specified by the [Spherical
    /// coordinate system](https://en.wikipedia.org/wiki/Spherical_coordinate_system).
    fn from(point: CartesianPoint) -> Self {
        f64::sqrt(point.x().powi(2) + point.y().powi(2) + point.z().powi(2)).into()
    }
}

impl Altitude {
    /// Returns the altitude value as an f64.
    pub fn as_f64(&self) -> f64 {
        self.0
    }
}

/// Represents an arbitrary point in space using the geographic system of coordinates.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct GeographicPoint {
    pub longitude: Longitude,
    pub latitude: Latitude,
    pub altitude: Altitude,
}

impl From<CartesianPoint> for GeographicPoint {
    fn from(point: CartesianPoint) -> Self {
        Self::default()
            .with_longitude(point.into())
            .with_latitude(point.into())
            .with_altitude(point.into())
    }
}

impl GeographicPoint {
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
    pub fn distance(&self, other: &GeographicPoint) -> f64 {
        let prod_latitude_sin = self.latitude.as_f64().sin() * other.latitude.as_f64().sin();
        let prod_latitude_cos = self.latitude.as_f64().cos() * other.latitude.as_f64().cos();
        let longitude_diff = (self.longitude.as_f64() - other.longitude.as_f64()).abs();

        (prod_latitude_sin + prod_latitude_cos * longitude_diff.cos()).acos()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use float_cmp::approx_eq;

//     const ULPS: i64 = 2;

//     #[test]
//     fn longitude_must_not_exceed_boundaries() {
//         struct TestCase {
//             name: &'static str,
//             input: f64,
//             longitude: f64,
//         }

//         vec![
//             TestCase {
//                 name: "positive longitude value must not change",
//                 input: 1.,
//                 longitude: 1.,
//             },
//             TestCase {
//                 name: "negative longitude value must not change",
//                 input: -3.,
//                 longitude: -3.,
//             },
//             TestCase {
//                 name: "positive overflowing longitude must change",
//                 input: PI + 1.,
//                 longitude: -PI + 1.,
//             },
//             TestCase {
//                 name: "negative overflowing longitude must change",
//                 input: -PI - 1.,
//                 longitude: PI - 1.,
//             },
//         ]
//         .into_iter()
//         .for_each(|test_case| {
//             let point = GeographicPoint::default().with_longitude(test_case.input);
//             assert!(
//                 approx_eq!(f64, point.longitude, test_case.longitude, ulps = ULPS),
//                 "{}: {} ±ε = {}",
//                 test_case.name,
//                 point.longitude,
//                 test_case.longitude
//             );
//         });
//     }

//     #[test]
//     fn latitude_must_not_exceed_boundaries() {
//         struct TestCase {
//             name: &'static str,
//             input: f64,
//             latitude: f64,
//         }

//         vec![
//             TestCase {
//                 name: "positive latitude value must not change",
//                 input: 1.,
//                 latitude: 1.,
//             },
//             TestCase {
//                 name: "negative latitude value must not change",
//                 input: -1.,
//                 latitude: -1.,
//             },
//             TestCase {
//                 name: "positive overflowing latitude must change",
//                 input: 7. * PI / 4.,
//                 latitude: -PI / 4.,
//             },
//             TestCase {
//                 name: "negative overflowing latidude must change",
//                 input: -7. * PI / 4.,
//                 latitude: PI / 4.,
//             },
//         ]
//         .into_iter()
//         .for_each(|test_case| {
//             let point = GeographicPoint::default().with_latitude(test_case.input);
//             assert!(
//                 approx_eq!(f64, point.latitude, test_case.latitude, ulps = ULPS),
//                 "{}: {} ±ε = {}",
//                 test_case.name,
//                 point.latitude,
//                 test_case.latitude
//             );
//         });
//     }

//     #[test]
//     fn longitude_may_change_based_on_latitude() {
//         struct TestCase {
//             name: &'static str,
//             input: f64,
//             longitude: f64,
//         }

//         vec![
//             TestCase {
//                 name: "positive latitude value must not change longitude",
//                 input: 1.,
//                 longitude: 0.,
//             },
//             TestCase {
//                 name: "negative latitude value must not change longitude",
//                 input: -1.,
//                 longitude: 0.,
//             },
//             TestCase {
//                 name: "positive overflowing latitude must not change longitude",
//                 input: 7. * PI / 4.,
//                 longitude: 0.,
//             },
//             TestCase {
//                 name: "negative overflowing latidude must not change longitude",
//                 input: -7. * PI / 4.,
//                 longitude: 0.,
//             },
//             TestCase {
//                 name: "positive overflowing latitude must change longitude",
//                 input: 5. * PI / 4.,
//                 longitude: -PI,
//             },
//             TestCase {
//                 name: "negative overflowing latidude must change longitude",
//                 input: -PI,
//                 longitude: -PI,
//             },
//         ]
//         .into_iter()
//         .for_each(|test_case| {
//             let point = GeographicPoint::default().with_latitude(test_case.input);
//             assert!(
//                 approx_eq!(f64, point.longitude, test_case.longitude, ulps = ULPS),
//                 "{}: {} ±ε = {}",
//                 test_case.name,
//                 point.longitude,
//                 test_case.longitude
//             );
//         });
//     }

//     #[test]
//     fn long_ratio_must_not_exceed_boundaries() {
//         struct TestCase {
//             name: &'static str,
//             longitude: f64,
//             ratio: f64,
//         }

//         vec![
//             TestCase {
//                 name: "zero longitude must be equals to zero",
//                 longitude: 0.,
//                 ratio: 0.,
//             },
//             TestCase {
//                 name: "positive longitude boundary must equals to positive one",
//                 longitude: PI,
//                 ratio: 1.,
//             },
//             TestCase {
//                 name: "arbitrary positive longitude must be positive",
//                 longitude: FRAC_PI_2,
//                 ratio: 0.5,
//             },
//             TestCase {
//                 name: "negative longitude boundary must equals to negative one",
//                 longitude: -PI,
//                 ratio: -1.,
//             },
//             TestCase {
//                 name: "arbitrary negative longitude must be negative",
//                 longitude: -FRAC_PI_2,
//                 ratio: -0.5,
//             },
//         ]
//         .into_iter()
//         .for_each(|test_case| {
//             let point = GeographicPoint::default().with_longitude(test_case.longitude);
//             assert!(
//                 approx_eq!(f64, point.long_ratio(), test_case.ratio, ulps = ULPS),
//                 "{}: {} ±ε = {}",
//                 test_case.name,
//                 point.long_ratio(),
//                 test_case.ratio
//             );
//         });
//     }

//     #[test]
//     fn lat_ratio_must_not_exceed_boundaries() {
//         struct TestCase {
//             name: &'static str,
//             latitude: f64,
//             ratio: f64,
//         }

//         vec![
//             TestCase {
//                 name: "zero latitude must be equals to zero",
//                 latitude: 0.,
//                 ratio: 0.,
//             },
//             TestCase {
//                 name: "positive latitude boundary must equals to one",
//                 latitude: FRAC_PI_2,
//                 ratio: 1.,
//             },
//             TestCase {
//                 name: "arbitrary positive latitude must be positive",
//                 latitude: FRAC_PI_2 / 2.,
//                 ratio: 0.5,
//             },
//             TestCase {
//                 name: "negative latitude boundary must equals to negative one",
//                 latitude: -FRAC_PI_2,
//                 ratio: -1.,
//             },
//             TestCase {
//                 name: "arbitrary negative latitude must be negative",
//                 latitude: -FRAC_PI_2 / 2.,
//                 ratio: -0.5,
//             },
//         ]
//         .into_iter()
//         .for_each(|test_case| {
//             let point = GeographicPoint::default().with_latitude(test_case.latitude);
//             assert!(
//                 approx_eq!(f64, point.lat_ratio(), test_case.ratio, ulps = ULPS),
//                 "{}: {} ±ε = {}",
//                 test_case.name,
//                 point.lat_ratio(),
//                 test_case.ratio
//             );
//         });
//     }

//     #[test]
//     fn basic_operations_must_be_consistent() {
//         let mut point = GeographicPoint::default()
//             .with_longitude(-FRAC_PI_2)
//             .with_latitude(FRAC_PI_2 / 2.);

//         point.set_latitude(point.latitude().add(PI));

//         assert!(
//             approx_eq!(f64, point.longitude(), FRAC_PI_2, ulps = ULPS),
//             "longitude must switch to positive: {} ±ε = {}",
//             point.longitude(),
//             FRAC_PI_2
//         );

//         assert!(
//             approx_eq!(f64, point.latitude(), -FRAC_PI_2 / 2., ulps = ULPS),
//             "latitude must switch to negative: {} ±ε = {}",
//             point.latitude(),
//             -FRAC_PI_2 / 2.
//         );
//     }

//     #[test]
//     fn geographic_from_cartesian_must_not_fail() {
//         struct TestCase {
//             name: &'static str,
//             geographic: GeographicPoint,
//             cartesian: CartesianPoint,
//         }

//         vec![
//             TestCase {
//                 name: "north point",
//                 geographic: GeographicPoint::default()
//                     .with_latitude(FRAC_PI_2)
//                     .with_altitude(1.),
//                 cartesian: CartesianPoint::new(0., 0., 1.),
//             },
//             TestCase {
//                 name: "south point",
//                 geographic: GeographicPoint::default()
//                     .with_latitude(-FRAC_PI_2)
//                     .with_altitude(1.),
//                 cartesian: CartesianPoint::new(0., 0., -1.),
//             },
//             TestCase {
//                 name: "east point",
//                 geographic: GeographicPoint::default()
//                     .with_longitude(FRAC_PI_2)
//                     .with_altitude(1.),
//                 cartesian: CartesianPoint::new(0., 1., 0.),
//             },
//             TestCase {
//                 name: "weast point",
//                 geographic: GeographicPoint::default()
//                     .with_longitude(-FRAC_PI_2)
//                     .with_altitude(1.),
//                 cartesian: CartesianPoint::new(0., -1., 0.),
//             },
//             TestCase {
//                 name: "front point",
//                 geographic: GeographicPoint::default().with_altitude(1.),
//                 cartesian: CartesianPoint::new(1., 0., 0.),
//             },
//             TestCase {
//                 name: "back point",
//                 geographic: GeographicPoint::default()
//                     .with_longitude(PI)
//                     .with_altitude(1.),
//                 cartesian: CartesianPoint::new(-1., 0., 0.),
//             },
//         ]
//         .into_iter()
//         .for_each(|test_case| {
//             let point = GeographicPoint::from(test_case.cartesian);
//             assert!(
//                 approx_eq!(
//                     f64,
//                     point.longitude(),
//                     test_case.geographic.longitude(),
//                     ulps = ULPS
//                 ),
//                 "{}: longitude {:#?} ±ε == {:#?}",
//                 test_case.name,
//                 point.longitude(),
//                 test_case.geographic.longitude(),
//             );

//             assert!(
//                 approx_eq!(
//                     f64,
//                     point.latitude(),
//                     test_case.geographic.latitude(),
//                     ulps = ULPS
//                 ),
//                 "{}: latitude {:#?} ±ε == {:#?}",
//                 test_case.name,
//                 point.latitude(),
//                 test_case.geographic.latitude(),
//             );

//             assert!(
//                 approx_eq!(
//                     f64,
//                     point.altitude(),
//                     test_case.geographic.altitude(),
//                     ulps = ULPS
//                 ),
//                 "{}: altitude {:#?} ±ε == {:#?}",
//                 test_case.name,
//                 point.altitude(),
//                 test_case.geographic.altitude(),
//             );
//         });
//     }

//     #[test]
//     fn distance_must_not_fail() {
//         struct TestCase<'a> {
//             name: &'a str,
//             from: GeographicPoint,
//             to: GeographicPoint,
//             distance: f64,
//         }

//         vec![
//             TestCase {
//                 name: "Same point must be zero",
//                 from: GeographicPoint::default(),
//                 to: GeographicPoint::default(),
//                 distance: 0.,
//             },
//             TestCase {
//                 name: "Oposite points in the horizontal",
//                 from: GeographicPoint::default(),
//                 to: GeographicPoint::default().with_longitude(-PI),
//                 distance: PI,
//             },
//             TestCase {
//                 name: "Oposite points in the vertical",
//                 from: GeographicPoint::default().with_latitude(FRAC_PI_2),
//                 to: GeographicPoint::default().with_latitude(-FRAC_PI_2),
//                 distance: PI,
//             },
//         ]
//         .into_iter()
//         .for_each(|test_case| {
//             let got = test_case.from.distance(&test_case.to);

//             assert!(
//                 approx_eq!(f64, got, test_case.distance, ulps = ULPS),
//                 "{}: distance {:#?} ±ε == {:#?}",
//                 test_case.name,
//                 got,
//                 test_case.distance,
//             )
//         });
//     }
// }
