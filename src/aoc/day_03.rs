use std::fs;

/// A top-left - bottom-right position in a schematic.
#[derive(Debug, PartialEq)]
struct Position {
    x: usize, // column
    y: usize, // row
}

fn find_part_numbers(schematic: &str) -> Vec<i32> {
    let mut valid_part_numbers: Vec<i32> = Vec::new();
    let symbol_positions = find_symbol_positions(schematic);

    let num_lines = schematic.lines().count();
    for (i, line) in schematic.lines().enumerate() {
        let seqs = seq_digit_positions(line);

        seqs.iter().for_each(|seq| {
            let candidate_positions = find_candidate_positions(num_lines, i, &line, seq);
            if candidate_positions.iter().any(|pos| symbol_positions.contains(pos)) {
                let number = line[seq[0]..=seq[seq.len() - 1]].parse::<i32>().unwrap();
                valid_part_numbers.push(number);
            }
        });
    }

    valid_part_numbers
}

fn find_candidate_positions(num_lines: usize, line_index: usize, line: &&str, positions_seq: &Vec<usize>) -> Vec<Position> {
    let mut candidate_positions: Vec<Position> = Vec::new();
    if line_index > 0 {
        // the row above
        positions_seq.iter().for_each(|pos| {
            candidate_positions.push(Position { x: *pos, y: line_index - 1 });
        });
    }
    if line_index < num_lines - 1 {
        // the row below
        positions_seq.iter().for_each(|pos| {
            candidate_positions.push(Position { x: *pos, y: line_index + 1 });
        });
    }
    let left_most_position = positions_seq.iter().min().unwrap();
    if left_most_position > &0 {
        // the column to the left
        candidate_positions.push(Position { x: left_most_position - 1, y: line_index });
        if line_index > 0 {
            // top-left corner
            candidate_positions.push(Position { x: left_most_position - 1, y: line_index - 1 });
        }
        if line_index < num_lines - 1 {
            // bottom-left corner
            candidate_positions.push(Position { x: left_most_position - 1, y: line_index + 1 });
        }
    }

    let right_most_position = positions_seq.iter().max().unwrap();
    if right_most_position < &(line.len() - 1) {
        // the column to the right
        candidate_positions.push(Position { x: right_most_position + 1, y: line_index });
        if line_index > 0 {
            // top-right corner
            candidate_positions.push(Position { x: right_most_position + 1, y: line_index - 1 });
        }
        if line_index < num_lines - 1 {
            // bottom-right corner
            candidate_positions.push(Position { x: right_most_position + 1, y: line_index + 1 });
        }
    }

    candidate_positions
}

/// Find all sequential positions with digits in a line.
fn seq_digit_positions(line: &str) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    let mut seq: Vec<usize> = Vec::new();
    for (i, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            seq.push(i);
        } else {
            if !seq.is_empty() {
                result.push(seq);
            }
            seq = Vec::new();
        }
    }

    if !seq.is_empty() {
        result.push(seq);
    }

    result
}

fn find_symbol_positions(schematic: &str) -> Vec<Position> {
    let mut result = Vec::new();

    for (y, line) in schematic.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if !c.is_digit(10) && c != '.' {
                result.push(Position { x, y });
            }
        }
    }

    result
}

pub fn solve_part_1(input_file: &str) -> i32 {
    return match fs::read_to_string(input_file) {
        Ok(schematic) => {
            find_part_numbers(schematic.as_str()).iter().sum()
        }
        Err(_) => {
            0
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SCHEMATIC: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_seq_digit_positions() {
        assert_eq!(seq_digit_positions("467..114.."), vec![vec![0, 1, 2], vec![5, 6, 7]]);
        assert_eq!(seq_digit_positions("...*......"), Vec::<Vec<usize>>::new());
        assert_eq!(seq_digit_positions("..35..633."), vec![vec![2, 3], vec![6, 7, 8]]);
    }

    #[test]
    fn test_symbol_positions() {
        assert_eq!(find_symbol_positions(TEST_SCHEMATIC), vec![
            Position { x: 3, y: 1 },
            Position { x: 6, y: 3 },
            Position { x: 3, y: 4 },
            Position { x: 5, y: 5 },
            Position { x: 3, y: 8 },
            Position { x: 5, y: 8 },
        ]);
    }

    #[test]
    fn find_part_numbers_example() {
        assert_eq!(find_part_numbers(TEST_SCHEMATIC), vec![467, 35, 633, 617, 592, 755, 664, 598]);

    }
    #[test]
    fn test_day3_example() {
        assert_eq!(find_part_numbers(TEST_SCHEMATIC).iter().sum::<i32>(), 4361);
    }

    #[test]
    fn row_1() {
        let rows= "\
....546......*....454...120..683.............923.....@...*...865.574......276........56...../57.659..*................-...-...512...........
............329...*.................................606.599...................*927..*.................674..*........723..974................
................378..911........987.....606......................899.73....489......848.....................664...............388......589..";
        assert_eq!(find_part_numbers(rows), vec![454, 56, 57, 329, 606, 599, 927, 674, 723, 974, 378, 489, 848, 664]);
    }

    #[test]
    fn test_line_4() {
        let rows = "\
............832*105..@........$..................*.........797.....535..932.........*....152...........123.........678.540...........-...6..
....&..948..........................271....-....228..79.26.........................733...=...715............27.586........*.......883...*...";
        assert_eq!(find_part_numbers(rows), vec![832, 105, 152, 540, 6, 228, 733, 883]);
    }
}