#![feature(iter_intersperse)]
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
    let days_to_print: String = args
        .days
        .iter()
        .map(|d| format!("{:02}", d))
        .intersperse(", ".into())
        .collect();
    println!("Running AOC 2022, days {}", days_to_print);
    for day in args.days {
        match run_day(day) {
            Ok(day_main) => {
                println!("Running day {day}");
                day_main()
            }
            Err(_) => match ERR_STRAT {
                ErrorStrategy::Panic => panic!("Invalid day {day}"),
                ErrorStrategy::Shout => eprintln!("Skipping invalid day {day}"),
                ErrorStrategy::Whisper => println!("Skipping invalid day {day}"),
                ErrorStrategy::Silent => (),
            },
        }
    }
}

#[derive(Parser)]
struct Args {
    days: Vec<u8>,
}

fn run_day(day: u8) -> Result<fn(), u8> {
    match day {
        1 => Ok(day01::main),
        2 => Ok(day02::main),
        3 => Ok(day03::main),
        4 => Ok(day04::main),
        5 => Ok(day05::main),
        6 => Ok(day06::main),
        7 => Ok(day07::main),
        8 => Ok(day08::main),
        9 => Ok(day09::main),
        10 => Ok(day10::main),
        11 => Ok(day11::main),
        12 => Ok(day12::main),
        13 => Ok(day13::main),
        14 => Ok(day14::main),
        15 => Ok(day15::main),
        16 => Ok(day16::main),
        17 => Ok(day17::main),
        18 => Ok(day18::main),
        19 => Ok(day19::main),
        20 => Ok(day20::main),
        21 => Ok(day21::main),
        22 => Ok(day22::main),
        23 => Ok(day23::main),
        24 => Ok(day24::main),
        25 => Ok(day25::main),
        _ => Err(day),
    }
}

const ERR_STRAT: ErrorStrategy = ErrorStrategy::Panic;
enum ErrorStrategy {
    Panic,
    Shout,
    Whisper,
    Silent,
}
