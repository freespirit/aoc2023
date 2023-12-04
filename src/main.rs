use clap::Parser;
use aoc2023::day_01::{solve_part_1, solve_part_2};

/// Solve a daily puzzle from Advent of Code 2023.
#[derive(Parser, Debug)]
struct Args {
    /// File with input text for the day.
    input_file: String,
}

fn main() {
    let args = Args::parse();
    println!("{}", solve_part_2(args.input_file.as_str()));
}
