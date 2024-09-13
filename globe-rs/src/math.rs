/// Returns the greatest common divisor of a and b.
pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    if a < b {
        (a, b) = (b, a);
    }

    while b != 0 {
        (a, b) = (b, a % b)
    }

    a
}

/// Returns the least common multiple of a and b.
pub fn lcd(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

pub fn common_base(decimals: &mut [f64]) -> f64 {
    let mut denominator = 1.;
    while decimals.iter().any(|d| d.fract() != 0.) {
        decimals.iter_mut().for_each(|d| *d *= 10.);
        denominator *= 10.;
    }

    denominator
}

#[cfg(test)]
mod tests {
    use crate::math::common_base;

    #[test]
    fn common_base_should_not_fail() {
        struct Test {
            input: Vec<f64>,
            output: f64,
        }

        vec![Test {
            input: vec![0.5, -0.05, 1.00045],
            output: 100_000.,
        }]
        .into_iter()
        .for_each(|mut test| {
            let got = common_base(&mut test.input);
            assert_eq!(got, test.output);
        })
    }
}
