use std::{
    f64::consts::PI,
    ops::{Add, Div, Mul},
};

use serde::{Deserialize, Serialize};

use crate::{Frequency, PositiveFloat};

/// The [radiant](https://en.wikipedia.org/wiki/Radian) unit, which is always a positive number
/// within the range of [0, 2π].
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Radiant(PositiveFloat);

impl From<f64> for Radiant {
    fn from(value: f64) -> Self {
        if (0. ..=Self::TWO_PI.as_f64()).contains(&value) {
            return Self(value.into());
        }

        let mut modulus = value % Self::TWO_PI.as_f64();
        if value.is_sign_negative() {
            modulus = (modulus + Self::TWO_PI.as_f64()) % Self::TWO_PI.as_f64();
        }

        Self(modulus.into())
    }
}

impl From<Frequency> for Radiant {
    /// The radiants per seconds the frequency represents.
    fn from(value: Frequency) -> Self {
        (value.as_hz() * Self::TWO_PI.as_f64()).into()
    }
}

impl Add for Radiant {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        (self.0 .0 + rhs.0 .0).into()
    }
}

impl Mul<f64> for Radiant {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        (self.0 .0 * rhs).into()
    }
}

impl Div<f64> for Radiant {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        (self.0 .0 / rhs).into()
    }
}

impl Radiant {
    pub const TWO_PI: Self = Self(PositiveFloat(2. * PI));

    /// Returns true if, and only if, self is exactly 2π, which implies a rotation of 360 degrees.
    pub fn is_full(&self) -> bool {
        self.0 == Self::TWO_PI.0
    }

    /// Returns the amount of radiants as a [f64].
    pub fn as_f64(&self) -> f64 {
        self.0 .0
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, PI};

    use crate::Radiant;

    #[test]
    fn radiant_must_not_exceed_boundaries() {
        struct Test {
            name: &'static str,
            input: f64,
            output: f64,
        }

        vec![
            Test {
                name: "radiant within range must not change",
                input: PI,
                output: PI,
            },
            Test {
                name: "2π radiants must not equals zero",
                input: Radiant::TWO_PI.as_f64(),
                output: Radiant::TWO_PI.as_f64(),
            },
            Test {
                name: "negative radiant must change",
                input: -FRAC_PI_2,
                output: Radiant::TWO_PI.as_f64() - FRAC_PI_2,
            },
            Test {
                name: "overflowing radiant must change",
                input: Radiant::TWO_PI.as_f64() + FRAC_PI_2,
                output: FRAC_PI_2,
            },
        ]
        .into_iter()
        .for_each(|test| {
            let radiant = Radiant::from(test.input).as_f64();

            assert_eq!(
                radiant, test.output,
                "{}: got radiant = {}, want {}",
                test.name, radiant, test.output
            );
        });
    }
}
