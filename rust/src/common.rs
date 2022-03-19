pub fn primes_below(limit: i64) -> Vec<i64> {
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
