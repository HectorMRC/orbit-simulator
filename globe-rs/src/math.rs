/// Returns the greatest common divisor of a and b.
pub fn integer_gcd(mut a: i64, mut b: i64) -> i64 {
    if a < b {
        (a, b) = (b, a);
    }

    while b != 0 {
        (a, b) = (b, a % b)
    }

    a
}

/// Returns the least common multiple of a and b.
pub fn integer_lcd(a: i64, b: i64) -> i64 {
    (a * b).abs() / integer_gcd(a, b)
}

/// Returns the numerator and denominator, fraction of which results in the given decimal value.
pub fn decimal_to_fraction(decimal: f64) -> (i64, i64) {
    let mut denominator = 1.;
    while (decimal * denominator).fract() != 0. {
        denominator *= 10.;
    }

    let numerator = (decimal * denominator) as i64;
    let denominator = denominator as i64;

    let gcd = integer_gcd(numerator, denominator as i64);
    (numerator / gcd, denominator / gcd)
}

/// Returns the least common multiple of two decimal values.
pub fn decimal_lcd(a: f64, b: f64) -> f64 {
    let (an, ad) = decimal_to_fraction(a);
    let (bn, bd) = decimal_to_fraction(b);

    integer_lcd(an, bn) as f64 / integer_gcd(ad, bd) as f64
}

#[cfg(test)]
mod tests {
    use crate::math::decimal_to_fraction;

    #[test]
    fn decimal_to_fraction_should_not_fail() {
        struct Test {
            input: f64,
            output: (i64, i64),
        }

        vec![
            Test {
                input: 0.5,
                output: (1, 2),
            },
            Test {
                input: 0.444444444444444444444444444444444444444,
                output: (4, 9),
            },
            Test {
                input: 0.666666666666666666666666666666666666666,
                output: (2, 3),
            },
            Test {
                input: 1.75,
                output: (7, 4),
            },
        ]
        .into_iter()
        .for_each(|test| {
            assert_eq!(decimal_to_fraction(test.input), test.output);
        })
    }
}
