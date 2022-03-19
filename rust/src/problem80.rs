use std::time::Instant;

use bigdecimal::BigDecimal;

pub fn run() {
    let start = Instant::now();
    let sum = (1..=100)
        .flat_map(|n| {
            BigDecimal::from(n).sqrt().and_then(|s| {
                if s.is_integer() {
                    None
                } else {
                    Some(s.as_bigint_and_exponent().0)
                }
            })
        })
        .map(|digits| {
            digits.to_string()[..100]
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("{:?} {}", start.elapsed(), sum);
}
