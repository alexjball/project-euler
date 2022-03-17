use std::{collections::HashSet, time::Instant};

fn main() {
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

fn primes_below(limit: i64) -> Vec<i64> {
    let mut is_prime: Vec<bool> = vec![true; limit as usize];
    let mut primes: Vec<i64> = Vec::new();

    for n in 2..is_prime.len() {
        if is_prime[n] {
            primes.push(n as i64);
            let mut f = n + n;
            while f < is_prime.len() {
                is_prime[f] = false;
                f += n;
            }
        }
    }

    return primes;
}
