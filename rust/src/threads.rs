use std::{thread, time::Duration};

pub fn run() {
    let n_cores = thread::available_parallelism()
        .map(|s| s.get())
        .unwrap_or(1);

    println!("available parallelism = {}", n_cores);

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
