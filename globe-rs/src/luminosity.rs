use std::{
    fmt::Debug,
    ops::{Div, Mul},
};

use serde::{Deserialize, Serialize};

use crate::PositiveFloat;

/// The intensity at which an arbitrary object brights.
#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Luminosity(PositiveFloat);

impl Mul<f64> for Luminosity {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self((self.0 .0 * rhs).into())
    }
}

impl Div<f64> for Luminosity {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self((self.0 .0 / rhs).into())
    }
}

impl Div for Luminosity {
    type Output = Luminosity;

    fn div(self, rhs: Self) -> Self::Output {
        Self((self.0 .0 / rhs.0 .0).into())
    }
}

impl Debug for Luminosity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Luminosity")
            .field(&format!("{} watts", self.0))
            .finish()
    }
}

impl Luminosity {
    const LUMENS_PER_WATT: f64 = 93.;
    pub const SUN: Self = Self(PositiveFloat(3.828e26));
    pub const ZERO: Self = Self(PositiveFloat::ZERO);

    /// Returns a new luminosity representing the given watts.
    pub fn watts(watts: f64) -> Self {
        Self(watts.into())
    }

    /// Returns a [f64] representing the luminosity in watts.
    pub fn as_watts(&self) -> f64 {
        self.0 .0
    }

    /// Returns a [f64] representing the luminosity in lumens.
    pub fn as_lm(&self) -> f64 {
        Self::LUMENS_PER_WATT * self.as_watts()
    }
}
