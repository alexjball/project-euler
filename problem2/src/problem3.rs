fn main() {
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
