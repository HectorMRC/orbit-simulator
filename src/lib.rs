mod cartesian;
use std::ops::{Add, Sub};

pub use cartesian::*;

mod geographic;
pub use geographic::*;

/// Returns true if, and only if, v2 is in the range of v1 ± ε. Otherwise returns false.
#[inline(always)]
pub fn approx_eq<T, E>(v1: T, v2: T, epsilon: E) -> bool
where
    T: Copy + PartialOrd + Add<E, Output = T> + Sub<E, Output = T>,
    E: Copy,
{
    v1 + epsilon >= v2 && v1 - epsilon <= v2
}
