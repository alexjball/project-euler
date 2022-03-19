use crate::common::primes_below;
use std::{collections::HashSet, time::Instant};

pub fn run() {
    let now = Instant::now();
    let limit = 1_000_000;
    let primes: HashSet<i64> = primes_below(limit).into_iter().collect();
    let mut count = 0;
    for n in 2..limit {
        let circular = is_circular_prime(n, &primes);
        if circular {
            count += 1;
        }
    }
    println!(
        "{:?} number of circular primes below {}: {}",
        now.elapsed(),
        limit,
        count
    );
}

fn is_circular_prime(n: i64, primes: &HashSet<i64>) -> bool {
    let mut s = n.to_string();
    let mut r = n;
    let n_chars = s.len();
    for _ in 0..n_chars {
        if !primes.contains(&r) {
            return false;
        }

        s = rotate(s);
        r = s.parse::<i64>().unwrap();
    }
    return true;
}

fn rotate(s: String) -> String {
    let mut c = s.chars();
    let first = c.next();
    let mut r = c.collect::<String>();
    r.push(first.unwrap());
    r
}
