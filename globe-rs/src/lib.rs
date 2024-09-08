pub mod cartesian;

pub mod geographic;

mod orbit;
pub use orbit::*;

mod system;
pub use system::*;

mod distance;
pub use distance::*;

mod frequency;
pub use frequency::*;

mod mass;
pub use mass::*;

mod radiant;
pub use radiant::*;

mod velocity;
pub use velocity::*;

mod luminosity;
pub use luminosity::*;

/// A [f64] that is always positive.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
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
