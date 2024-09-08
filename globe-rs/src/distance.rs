use std::ops::{Add, Div, Mul};

use serde::{Deserialize, Serialize};

use crate::PositiveFloat;

/// The distance between two points in space, which is always a positive number.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Distance(PositiveFloat);

impl Add for Distance {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self((self.0 .0 + rhs.0 .0).into())
    }
}

impl Mul<f64> for Distance {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self((self.0 .0 * rhs).into())
    }
}

impl Div<f64> for Distance {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self((self.0 .0 / rhs).into())
    }
}

impl Distance {
    pub const ZERO: Self = Self(PositiveFloat::ZERO);
    pub const ASTRONOMICAL_UNIT: Self = Self(PositiveFloat(149_597_870.7));

    /// Returns a new distance of km kilometers.
    pub fn km(km: f64) -> Self {
        Self((km).into())
    }

    /// Returns a [f64] representing the distance in meters.
    pub fn as_meters(&self) -> f64 {
        self.0 .0 * 1000.
    }

    /// Returns a [f64] representing the distance in kilometers.
    pub fn as_km(&self) -> f64 {
        self.0 .0
    }

    /// Returns the absolute difference between self and the given distance.
    pub fn diff(self, rhs: Self) -> Self {
        Self((self.0 .0 - rhs.0 .0).into())
    }
}
