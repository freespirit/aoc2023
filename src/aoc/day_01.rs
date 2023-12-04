use std::fs;

/// # Examples:
///
/// ```
/// assert_eq!(aoc2023::aoc::day_01::calibrate("1abc2"), 12);
/// ```
///
/// ```
/// assert_eq!(aoc2023::aoc::day_01::calibrate("pqr3stu8vwx"), 38);
/// ```
///
/// ```
/// assert_eq!(aoc2023::aoc::day_01::calibrate("a1b2c3d4e5f"), 15);
/// ```
///
/// ```
/// assert_eq!(aoc2023::aoc::day_01::calibrate("treb7uchet"), 77);
/// ```
pub fn calibrate(line: &str) -> i32 {
    let digits: Vec<i32> = line.chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    10 * digits.first().unwrap_or(&0)
        + digits.last().unwrap_or(&0)
}

/// Solve AOC2023 day 1, Part One.
/// https://adventofcode.com/2023/day/1
pub fn solve_part_1(input: &str) -> i32 {
    return match fs::read_to_string(input) {
        Ok(contents) => {
            contents.lines().map(|line| calibrate(line)).sum()
        }
        Err(error) => {
            println!("Error: {}", error);
            0
        }
    };
}

/// Calibrate with words in addition to digits.
///
/// # Examples:
/// ```
/// assert_eq!(aoc2023::aoc::day_01::calibrate_words("two1nine"), 29);
/// ```
///
/// ```
/// assert_eq!(aoc2023::aoc::day_01::calibrate_words("eightwothree"), 83);
/// ```
///
/// ```
/// assert_eq!(aoc2023::aoc::day_01::calibrate_words("abcone2threexyz"), 13);
/// ```
///
/// ```
/// assert_eq!(aoc2023::aoc::day_01::calibrate_words("xtwone3four"), 24);
/// ```
///
/// ```
/// assert_eq!(aoc2023::aoc::day_01::calibrate_words("4nineeightseven2"), 42);
/// ```
///
/// ```
/// assert_eq!(aoc2023::aoc::day_01::calibrate_words("zoneight234"), 14);
/// ```
///
/// ```
/// assert_eq!(aoc2023::aoc::day_01::calibrate_words("7pqrstsixteen"), 76);
/// ```
///
/// ```
/// assert_eq!(aoc2023::aoc::day_01::calibrate_words("262"), 22);
/// ```
pub fn calibrate_words(line: &str) -> i32 {
    let words = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let digits = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    // rewrite using matches to find all possible positions for a single digit/word
    let mut digit_positions: std::collections::HashMap<usize, i32> = std::collections::HashMap::new();
    for (index, word) in words.iter().enumerate() {
        line.match_indices(word)
            .for_each(|(position, _)| {
                digit_positions.insert(position, (index + 1) as i32);
            });
    }
    for digit in digits.iter() {
        line.match_indices(&digit.to_string())
            .for_each(|(position, _)| {
                digit_positions.insert(position, *digit);
            });
    }


    // find the lowest and highest digit positions
    let left_most = digit_positions.keys()
        .min()
        .map(|k| digit_positions.get(k).unwrap_or(&0));
    let right_most = digit_positions.keys()
        .max()
        .map(|k| digit_positions.get(k).unwrap_or(&0));


    let result = 10 * left_most.unwrap_or(&0) + right_most.unwrap_or(&0);
    // println!("{} -> {}", line, result);
    return result;
}

/// Solve AOC2023 day 1, Part Two.
/// https://adventofcode.com/2023/day/1#part2
pub fn solve_part_2(input: &str) -> i32 {
    return match fs::read_to_string(input) {
        Ok(contents) => {
            contents.lines().map(|line| calibrate_words(line)).sum()
        }
        Err(error) => {
            println!("Error: {}", error);
            0
        }
    };
}