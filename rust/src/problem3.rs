use crate::common::primes_below;

pub fn run() {
    let n: i64 = 600851475143;
    let limit = (n as f64).sqrt().floor() as i64;
    let primes = primes_below(limit);
    let mut largest = 0;
    for f in primes {
        if n % f == 0 {
            largest = f;
        }
    }
    println!("largest factor of {}: {}", n, largest);
}
