use std::{collections::HashMap, time::Instant};

pub fn run() {
    let now = Instant::now();
    let k = 5;
    let max_n = 20000;
    let result = find_first_with_k_permutations(k, max_n);
    println!(
        "{:?} k={}, max_n={}, result={}",
        now.elapsed(),
        k,
        max_n,
        result
    );
}

#[derive(Debug)]
struct PermGroup {
    min: u64,
    n: [u64; 10],
    count: u8,
}

#[derive(Debug)]
struct Candidate {
    n: u64,
    cutoff: u64,
}
// [Candidate { n: 5027, cutoff: 1000000000000 }, Candidate { n: 5196, cutoff: 1000000000000 }]
fn find_first_with_k_permutations(k: u8, max_n: u64) -> u64 {
    let mut cube_counts: HashMap<u64, PermGroup> = HashMap::new();
    let mut candidates: Vec<Candidate> = Vec::new();

    for n in 1..max_n {
        let cube = n * n * n;
        let hash = hash_digits(cube);

        if !candidates.is_empty() {
            // println!("{:?}", candidates);
            match candidates.iter().find(|c| cube > c.cutoff) {
                Some(c) => {
                    return c.n;
                }
                None => (),
            }
        }

        // println!("{} {}", n, hash);
        match cube_counts.get_mut(&hash) {
            Some(group) => {
                group.count += 1;
                group.n[group.count as usize] = n;
                if group.count == k {
                    let n = group.min;
                    let cube = (n * n * n) as f64;
                    println!("{:?}", group);
                    candidates.push(Candidate {
                        n,
                        cutoff: 10_u64.pow(cube.log10().floor() as u32 + 1),
                    });
                }
            }
            None => {
                cube_counts.insert(
                    hash,
                    PermGroup {
                        min: n,
                        count: 1,
                        n: [0, n, 0, 0, 0, 0, 0, 0, 0, 0],
                    },
                );
            }
        }
    }
    return 0;
}

fn hash_digits(mut n: u64) -> u64 {
    let mut digits = Vec::<u8>::new();
    while n > 0 {
        digits.push((n % 10) as u8);
        n /= 10;
    }
    digits.sort();
    let mut hash = 1_u64;
    for d in digits.iter().rev() {
        hash = hash * 10 + *d as u64;
    }
    return hash;
}
