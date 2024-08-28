use crate::PositiveFloat;

/// The distance between two points in space, which is always a positive number.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Distance(PositiveFloat);

impl Distance {
    /// Returns a new distance of km kilometers.
    pub fn km(km: f64) -> Self {
        Self((km).into())
    }

    /// Returns an [f64] representing the distance in meters.
    pub fn as_meters(&self) -> f64 {
        f64::from(self.0) * 1000.
    }

    /// Returns an [f64] representing the distance in kilometers.
    pub fn as_km(&self) -> f64 {
        self.0.into()
    }
}
