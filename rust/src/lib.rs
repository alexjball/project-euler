use std::{collections::HashMap, env::Args};

mod common;

mod problem2;
mod problem3;
mod problem35;
mod problem4;
mod problem62;
mod problem68;

fn problems() -> HashMap<i32, fn()> {
    HashMap::from([
        (2, problem2::run as fn()),
        (3, problem3::run),
        (4, problem4::run),
        (35, problem35::run),
        (62, problem62::run),
        (68, problem68::run),
    ])
}

pub struct Config {
    problem: i32,
}

impl Config {
    pub fn new(mut args: Args) -> Config {
        args.next();
        let problem = args
            .next()
            .unwrap_or_else(|| panic!("Please specify the problem number"))
            .parse()
            .unwrap_or_else(|_| panic!("Invalid number"));

        Config { problem }
    }
}

pub fn run(config: Config) {
    problems()
        .get(&config.problem)
        .unwrap_or_else(|| panic!("Unimplemented problem"))()
}
