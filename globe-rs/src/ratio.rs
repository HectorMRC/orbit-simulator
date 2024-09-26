use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::PositiveFloat;

/// A value that must be in the range of [[0, 1]].
#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
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

impl Debug for Ratio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Ratio")
            .field(&format!("{}%", self.0 .0 * 100.))
            .finish()
    }
}

impl Ratio {
    /// Returns the ratio as a [f64].
    pub fn as_f64(&self) -> f64 {
        self.0 .0
    }
}
