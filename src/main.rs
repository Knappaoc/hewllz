#![feature(iter_intersperse, never_type, specialization)]
use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

use clap::Parser;

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
    let args = Args::parse();
    println!("Running AOC 2022, days {:?}", args.days);
    for day in args.days {
        if let Err(_) = run_day(day) {
            match ERR_STRAT {
                ErrorStrategy::Panic => panic!("Invalid day {day}"),
                ErrorStrategy::Shout => {
                    eprintln!("Skipping invalid day {day}");
                    return;
                }
                ErrorStrategy::Whisper => {
                    println!("Skipping invalid day {day}");
                    return;
                }
                ErrorStrategy::Silent => return,
            }
        }
    }
}

#[derive(Parser)]
struct Args {
    days: Vec<u8>,
}

fn run_day(day: u8) -> Result<(), u8> {
    let file_path = format!("resources/day{day:02}.txt");
    let input = File::open(file_path).expect("Failed to open file to read");
    let solution = get_solution_for(day)?;
    solution.run(input);
    Ok(())
}

fn solution_from<A, B>(f: &'static fn(A) -> B) -> &'static dyn Solution
where
    fn(A) -> B: Solution,
{
    f as &dyn Solution
}

fn get_solution_for(day: u8) -> Result<&'static dyn Solution, u8> {
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
        _ => Err(day),
    }
}

impl<T: Display, E: Display> Solution for fn(File) -> Result<T, E> {
    default fn run(&self, file: File) {
        match self(file) {
            Ok(v) => println!("solution: {v}"),
            Err(e) => eprintln!("error: {e}"),
        }
    }
}
impl<E: Display> Solution for fn(File) -> Result<String, E> {
    fn run(&self, file: File) {
        match self(file) {
            Ok(v) => println!("solution: {v}"),
            Err(e) => eprintln!("error: {e}"),
        }
    }
}
impl<T: Display> Solution for fn(File) -> Option<T> {
    fn run(&self, file: File) {
        match self(file) {
            Some(v) => println!("solution: {v}"),
            None => eprintln!("error"),
        }
    }
}
impl Solution for fn(File) -> String {
    fn run(&self, file: File) {
        println!("{}", self(file))
    }
}
impl Solution for fn(File) {
    fn run(&self, file: File) {
        self(file)
    }
}
impl Solution for fn() {
    fn run(&self, _: File) {
        unimplemented!("this solution hasn't been implemented yet")
    }
}

trait Solution {
    fn run(&self, input: File);
}

const ERR_STRAT: ErrorStrategy = ErrorStrategy::Panic;

#[allow(dead_code)]
enum ErrorStrategy {
    Panic,
    Shout,
    Whisper,
    Silent,
}
