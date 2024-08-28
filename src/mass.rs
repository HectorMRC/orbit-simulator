use crate::PositiveFloat;

/// The mass of an arbitrary object, which is always a positive number.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Mass(PositiveFloat);

impl Mass {
    /// Returns a new mass of kg kilograms.
    pub fn kg(kg: f64) -> Self {
        Self((kg).into())
    }

    /// Returns an [f64] representing the mass in kilograms.
    pub fn as_kg(&self) -> f64 {
        self.0.into()
    }
}
