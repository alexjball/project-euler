use std::{str::FromStr, time::Instant};

use bigdecimal::BigDecimal;

pub fn run() {
    let start = Instant::now();
    let dec = BigDecimal::from_str("5").unwrap().with_prec(200);
    let sqrt = dec.sqrt().unwrap().with_prec(200);
    println!("{:?} {} {}", start.elapsed(), dec, sqrt);
}
