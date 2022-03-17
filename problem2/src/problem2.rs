fn main() {
    let mut sum = 0;
    let mut a_n2;
    let mut a_n1 = 1;
    let mut a_n = 2;
    while a_n <= 4_000_000 {
        if a_n % 2 == 0 {
            sum += a_n
        }
        a_n2 = a_n1;
        a_n1 = a_n;
        a_n = a_n1 + a_n2
    }
    println!("sum {}", sum);
}
