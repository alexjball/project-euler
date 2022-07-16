pub fn run() {
    let sum_of_squares: i64 = (1..=100).map(|x| x * x).sum();
    let square_of_sum: i64 = (1..=100).sum::<i64>().pow(2);
    let diff = square_of_sum - sum_of_squares;
    println!("diff {}", diff)
}
