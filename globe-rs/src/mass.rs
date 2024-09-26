use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::PositiveFloat;

/// The mass of an arbitrary object, which is always a positive number.
#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Mass(PositiveFloat);

impl Debug for Mass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Mass")
            .field(&format!("{} kg", self.0))
            .finish()
    }
}

impl Mass {
    /// Returns a new mass of kg kilograms.
    pub fn kg(kg: f64) -> Self {
        Self((kg).into())
    }

    /// Returns a [f64] representing the mass in kilograms.
    pub fn as_kg(&self) -> f64 {
        self.0 .0
    }
}
