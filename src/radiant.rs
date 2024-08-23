use std::f64::consts::PI;

pub const TWO_PI: f64 = 2. * PI;

/// The [radiant](https://en.wikipedia.org/wiki/Radian) unit, which is always a positive number
/// within the range of [0, 2π].
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Radiant(f64);

impl From<f64> for Radiant {
    fn from(value: f64) -> Self {
        if (0. ..=TWO_PI).contains(&value) {
            return Self(value);
        }

        let mut modulus = value % TWO_PI;
        if value.is_sign_negative() {
            modulus = (modulus + TWO_PI) % TWO_PI;
        }

        Self(modulus)
    }
}

impl From<Radiant> for f64 {
    fn from(value: Radiant) -> Self {
        value.0
    }
}

impl Radiant {
    pub const MAX: Self = Self(TWO_PI);
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, PI};

    use crate::{Radiant, TWO_PI};

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
                name: "negative radiant must change",
                input: -FRAC_PI_2,
                output: TWO_PI - FRAC_PI_2,
            },
            Test {
                name: "overflowing radiant must change",
                input: TWO_PI + FRAC_PI_2,
                output: FRAC_PI_2,
            },
        ]
        .into_iter()
        .for_each(|test| {
            let radiant: f64 = Radiant::from(test.input).into();

            assert_eq!(
                radiant, test.output,
                "{}: got radiant = {}, want {}",
                test.name, radiant, test.output
            );
        });
    }
}
