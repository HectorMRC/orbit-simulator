use std::ops::Add;

use serde::{Deserialize, Serialize};

use crate::PositiveFloat;

/// The distance between two points in space, which is always a positive number.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Distance(PositiveFloat);

impl Add for Distance {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Distance {
    pub const NONE: Self = Self(PositiveFloat::ZERO);

    /// Returns a new distance of km kilometers.
    pub fn km(km: f64) -> Self {
        Self((km).into())
    }

    /// Returns a [f64] representing the distance in meters.
    pub fn as_meters(&self) -> f64 {
        f64::from(self.0) * 1000.
    }

    /// Returns a [f64] representing the distance in kilometers.
    pub fn as_km(&self) -> f64 {
        self.0.into()
    }
}
