use std::fmt::Display;

pub mod cartesian;
pub mod geographic;

mod orbit;
pub use orbit::*;

mod distance;
pub use distance::*;

mod luminosity;
pub use luminosity::*;

mod mass;
pub use mass::*;

mod radian;
pub use radian::*;

mod ratio;
pub use ratio::*;

mod velocity;
pub use velocity::*;

/// A [f64] that is always positive.
#[derive(Debug, Default, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
struct PositiveFloat(f64);

impl From<f64> for PositiveFloat {
    fn from(value: f64) -> Self {
        Self(value.abs())
    }
}

impl Eq for PositiveFloat {}

impl Ord for PositiveFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl PartialOrd for PositiveFloat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for PositiveFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PositiveFloat {
    pub const ZERO: Self = Self(0.);
}

#[cfg(test)]
mod tests {
    use std::ops::Sub;

    use num_traits::Signed;

    /// Returns true if, and only if, abs_error >= |v1 - v2|. Otherwise returns false.
    #[inline(always)]
    pub fn approx_eq<T, E>(v1: T, v2: T, abs_error: E) -> bool
    where
        T: Sub<Output = T> + Signed,
        E: PartialOrd<T>,
    {
        abs_error >= (v1 - v2).abs()
    }
}
