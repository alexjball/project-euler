pub fn run() {
    let mut largest: (i32, i32, i32) = (0, 0, 0);
    for n1 in 100..1000 {
        for n2 in n1..1000 {
            let p = n1 * n2;
            if is_palindrome(p) && p > largest.2 {
                largest = (n1, n2, p);
            }
        }
    }
    println!("largest palindrome: {:?}", largest)
}

fn is_palindrome(n: i32) -> bool {
    let forward = n.to_string();
    let reverse: String = forward.chars().rev().collect();
    return forward == reverse;
}
