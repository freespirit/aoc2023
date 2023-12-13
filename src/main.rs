use std::fs;
use clap::Parser;
use aoc2023::{day_03, day_04, day_05};

/// Solve a daily puzzle from Advent of Code 2023.
#[derive(Parser, Debug)]
struct Args {
    /// File with input text for the day.
    input_file: String,
}

fn main() {
    let args = Args::parse();
    match fs::read_to_string(args.input_file.as_str()) {
        Ok(schematic) => {
            println!("Part 1: {}", day_05::solve_part_1(schematic.as_str()));
            println!("Part 2: {}", day_05::solve_part_2(schematic.as_str()));
        }
        Err(_) => { println!("Error reading input file."); }
    };
}
