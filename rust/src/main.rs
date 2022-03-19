use std::env::args;

fn main() {
    let config = euler::Config::new(args());
    euler::run(config)
}
