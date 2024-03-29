use std::{collections::HashMap, env::Args, time::Instant};

mod common;

mod problem107;
mod problem160;
mod problem2;
mod problem3;
mod problem35;
mod problem4;
mod problem6;
mod problem62;
mod problem68;
mod problem80;
mod scratch;
mod threads;

fn problems() -> HashMap<&'static str, fn()> {
    HashMap::from([
        ("2", problem2::run as fn()),
        ("3", problem3::run),
        ("4", problem4::run),
        ("6", problem6::run),
        ("35", problem35::run),
        ("62", problem62::run),
        ("68", problem68::run),
        ("80", problem80::run),
        ("107", problem107::run),
        ("160", problem160::run),
        ("scratch", scratch::run),
        ("threads", threads::run),
    ])
}

pub struct Config {
    problem: String,
}

impl Config {
    pub fn new(mut args: Args) -> Config {
        args.next();
        let problem = args
            .next()
            .unwrap_or_else(|| panic!("Please specify the problem number"));
        Config { problem }
    }
}

pub fn run(config: Config) {
    let problems = problems();
    let run = problems
        .get(config.problem.as_str())
        .unwrap_or_else(|| panic!("Unimplemented problem"));

    let start = Instant::now();
    run();
    let end = Instant::now();

    println!("\n{:?} elapsed", end.duration_since(start))
}
