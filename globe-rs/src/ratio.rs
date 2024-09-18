use serde::{Deserialize, Serialize};

use crate::PositiveFloat;

/// A value that must be in the range of [[0, 1]].
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Ratio(PositiveFloat);

impl From<f64> for Ratio {
    fn from(value: f64) -> Self {
        if value > 1. {
            Self(PositiveFloat::from(1.))
        } else if value < 0. {
            Self(PositiveFloat::from(0.))
        } else {
            Self(value.into())
        }
    }
}

impl Ratio {
    /// Returns the ratio as a [f64].
    pub fn as_f64(&self) -> f64 {
        self.0 .0
    }
}
