pub fn run() {
    let n: u64 = 1_000_000_000;
    println!("f({}) = {}", n, f(n))
}

fn f(n: u64) -> u64 {
    let mut sum: u64 = 0;
    for i in 1..n {
        sum = (sum + i) % 1_000_000
    }
    sum
}
