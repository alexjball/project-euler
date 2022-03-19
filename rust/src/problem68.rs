use itertools::Itertools;
use std::time::Instant;

pub fn run() {
    let x: Vec<u64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let x: Vec<u64> = vec![1, 2, 3, 4, 5, 6];

    let start = Instant::now();
    let perms = x.iter().permutations(x.len());
    let solutions = perms.filter_map(check_permutation);
    let max_16_digit = solutions
        .clone()
        .filter(|s| s < &10_u64.pow(16))
        .max()
        .unwrap();
    let end = Instant::now();

    println!(
        "{:?} solutions:{:#?}\nmax: {}",
        end.duration_since(start),
        solutions.collect_vec(),
        max_16_digit
    );
}

fn check_permutation(v: Vec<&u64>) -> Option<u64> {
    let lines = [
        [v[0], v[1], v[2]],
        [v[3], v[2], v[4]],
        [v[5], v[4], v[6]],
        [v[7], v[6], v[8]],
        [v[9], v[8], v[1]],
    ];

    // let lines = [[v[0], v[1], v[2]], [v[3], v[2], v[4]], [v[5], v[4], v[1]]];

    let correct_order = lines[0][0] == lines.iter().min_by_key(|l| l[0]).unwrap()[0];
    if !correct_order {
        return None;
    }

    let all_equal = lines
        .iter()
        .map(|line| line[0] + line[1] + line[2])
        .all_equal();
    if !all_equal {
        return None;
    }

    let mut magic: u64 = 0;
    for v in lines.iter().flatten() {
        magic = magic * (if **v == 10 { 100 } else { 10 }) + **v;
    }

    return Some(magic);
}
