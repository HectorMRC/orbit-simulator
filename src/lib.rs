use std::ops::Sub;

use num_traits::Signed;

pub mod cartesian;

pub mod geographic;

mod radiant;
pub use radiant::*;

/// Returns true if, and only if, abs_error >= |v1 - v2|. Otherwise returns false.
#[inline(always)]
pub fn approx_eq<T, E>(v1: T, v2: T, abs_error: E) -> bool
where
    T: Sub<Output = T> + Signed,
    E: PartialOrd<T>
{
    abs_error >= (v1 - v2).abs()
}
