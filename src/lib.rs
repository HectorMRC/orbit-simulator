use std::ops::{Add, Sub};

mod cartesian;
pub use cartesian::*;

mod geographic;
pub use geographic::*;

mod radiant;
pub use radiant::*;

/// Returns true if, and only if, v2 is in the range of v1 ± e. Otherwise returns false.
#[inline(always)]
pub fn approx_eq<T, E>(v1: T, v2: T, abs_error: E) -> bool
where
    T: Copy + PartialOrd + Add<E, Output = T> + Sub<E, Output = T>,
    E: Copy,
{
    v1 + abs_error >= v2 && v1 - abs_error <= v2
}
