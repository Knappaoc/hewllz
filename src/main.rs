#![feature(iter_intersperse, never_type, specialization)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

fn main() {
    app::main()
}

mod app {
    use clap::Parser;
    const ERR_STRAT: ErrorStrategy = ErrorStrategy::Panic;

    #[allow(dead_code)]
    enum ErrorStrategy {
        Panic,
        Shout,
        Whisper,
        Silent,
    }

    #[derive(Parser)]
    struct Args {
        days: Vec<u8>,
    }
    pub fn main() {
        let args = Args::parse();
        println!("Running AOC 2022, days {:?}", args.days);
        for day in args.days {
            if let Err(e) = super::adapter::run_day(day) {
                match ERR_STRAT {
                    ErrorStrategy::Panic => panic!("Error on day {day}: {e}"),
                    ErrorStrategy::Shout => {
                        eprintln!("Error on day {day}: {e}");
                        return;
                    }
                    ErrorStrategy::Whisper => {
                        println!("Error on day {day}: {e}");
                        return;
                    }
                    ErrorStrategy::Silent => return,
                }
            }
        }
    }
}
/*
mod adapter02 {
    use super::*;
    use std::{
        fmt::Display,
        fs::File,
        io::{BufRead, BufReader},
    };

    use anyhow::Context;

    fn get_solution_for(day: u8) -> Result<Solution, u8> {
        match day {
            1 => Ok(Solution::File(&(day01::main as fn(_) -> _))),
            2 => Ok(Solution::Line(&(day02::run as fn(_) -> _))),
            _ => Err(day),
        }
    }

    trait LineProcessor {
        fn run(&self, input: &mut dyn Iterator<Item = (usize, String)>);
    }
    trait FileProcessor {
        fn run(&self, input: File);
    }
    enum Solution {
        File(&'static dyn FileProcessor),
        Line(&'static dyn LineProcessor),
    }
    impl Solution {}

    impl FileProcessor for Solution {
        fn run(&self, input: File) {
            match *self {
                Solution::File(f) => f.run(input),
                Solution::Line(f) => run_lines(f, input).unwrap(),
            }
        }
    }

    fn run_lines(f: &dyn LineProcessor, input: File) -> anyhow::Result<()> {
        let lines = BufReader::new(input).lines().enumerate().map(
            |(i, r)| -> anyhow::Result<(usize, String)> {
                Ok((i, r.context(format!("failed to read line {i} of input"))?))
            },
        );
        itertools::process_results(lines, |mut it| f.run(&mut it))
    }
    impl<E> LineProcessor for fn(&mut dyn Iterator<Item = (usize, String)>) -> Result<String, E> {
        fn run(&self, input: &mut dyn Iterator<Item = (usize, String)>) {
            self(input);
        }
    }
    // impl LineProcessor for fn(&mut dyn Iterator<Item = (usize, String)>) {
    //     fn run(&self, input: &mut dyn Iterator<Item = (usize, String)>) {
    //         self(input)
    //     }
    // }

    impl<T: Display, E: Display> FileProcessor for fn(File) -> Result<T, E> {
        default fn run(&self, file: File) {
            match self(file) {
                Ok(v) => println!("solution: {v}"),
                Err(e) => eprintln!("error: {e}"),
            }
        }
    }
    impl<E: Display> FileProcessor for fn(File) -> Result<String, E> {
        fn run(&self, file: File) {
            match self(file) {
                Ok(v) => println!("solution: {v}"),
                Err(e) => eprintln!("error: {e}"),
            }
        }
    }
    impl<T: Display> FileProcessor for fn(File) -> Option<T> {
        fn run(&self, file: File) {
            match self(file) {
                Some(v) => println!("solution: {v}"),
                None => eprintln!("error"),
            }
        }
    }
    impl FileProcessor for fn(File) -> String {
        fn run(&self, file: File) {
            println!("{}", self(file))
        }
    }
    impl FileProcessor for fn(File) {
        fn run(&self, file: File) {
            self(file)
        }
    }
    impl FileProcessor for fn() {
        fn run(&self, _: File) {
            unimplemented!("this solution hasn't been implemented yet")
        }
    }
}
*/

mod adapter {
    use anyhow::{anyhow, Context};

    use super::*;
    use std::{fmt::Display, fs::File};

    pub fn run_day(day: u8) -> anyhow::Result<()> {
        let file_path = format!("resources/day{day:02}.txt");
        let input = File::open(&file_path)
            .with_context(|| format!("Failed to open file '{file_path}' to read"))?;
        let solution = get_solution_for(day)?;
        println!("running day {day}");
        solution.run(input);
        Ok(())
    }

    fn solution_from<A, B>(f: &'static fn(A) -> B) -> &'static dyn Solution
    where
        fn(A) -> B: Solution,
    {
        f as &dyn Solution
    }

    fn get_solution_for(day: u8) -> anyhow::Result<&'static dyn Solution> {
        match day {
            1 => Ok(solution_from(&(day01::main as _))),
            2 => Ok(solution_from(&(day02::main as _))),
            3 => Ok(solution_from(&(day03::main as _))),
            4 => Ok(solution_from(&(day04::main as _))),
            5 => Ok(solution_from(&(day05::main as _))),
            6 => Ok(solution_from(&(day06::main as _))),
            7 => Ok(solution_from(&(day07::main as _))),
            8 => Ok(solution_from(&(day08::main as _))),
            9 => Ok(solution_from(&(day09::main as _))),
            10 => Ok(solution_from(&(day10::main as _))),
            11 => Ok(solution_from(&(day11::main as _))),
            12 => Ok(solution_from(&(day12::main as _))),
            13 => Ok(solution_from(&(day13::main as _))),
            14 => Ok(solution_from(&(day14::main as _))),
            15 => Ok(solution_from(&(day15::main as _))),
            16 => Ok(solution_from(&(day16::main as _))),
            17 => Ok(solution_from(&(day17::main as _))),
            18 => Ok(solution_from(&(day18::main as _))),
            19 => Ok(solution_from(&(day19::main as _))),
            20 => Ok(solution_from(&(day20::main as _))),
            21 => Ok(solution_from(&(day21::main as _))),
            22 => Ok(solution_from(&(day22::main as _))),
            23 => Ok(solution_from(&(day23::main as _))),
            24 => Ok(solution_from(&(day24::main as _))),
            25 => Ok(solution_from(&(day25::main as _))),
            _ => Err(anyhow!("Invalid day: {day}")),
        }
    }

    impl<E: Display> Solution for fn(File) -> Result<(), E> {
        default fn run(&self, file: File) {
            if let Err(e) = self(file) {
                eprintln!("error: {e}")
            }
        }
    }
    impl Solution for fn(File) {
        fn run(&self, file: File) {
            self(file)
        }
    }

    trait Solution {
        fn run(&self, input: File);
    }
}
