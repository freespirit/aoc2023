use std::fs;
use clap::Parser;
use aoc2023::{day_03, day_04, day_05, day_06, day_07, day_08, day_09};

/// Solve a daily puzzle from Advent of Code 2023.
#[derive(Parser, Debug)]
struct Args {
    /// File with input text for the day.
    input_file: String,
}

fn main() {
    let args = Args::parse();
    match fs::read_to_string(args.input_file) {
        Ok(schematic) => {
            println!("Part 1: {}", day_09::solve_part_1(schematic.as_str()));
            println!("Part 2: {}", day_09::solve_part_2(schematic.as_str()));
        }
        Err(_) => { println!("Error reading input file."); }
    };
}
