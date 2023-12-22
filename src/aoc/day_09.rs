enum Direction {
    Left,
    Right,
}

fn find_next_value(seq: &Vec<i64>, direction: Direction) -> i64 {
    let mut diffs: Vec<i64> = seq.clone();
    let mut outer_most_vals: Vec<i64> = Vec::new();

    while !diffs.iter().all(|&a| a == 0) {
        match direction {
            Direction::Left => { outer_most_vals.push(*diffs.first().unwrap()) }
            Direction::Right => { outer_most_vals.push(*diffs.last().unwrap()) }
        };
        diffs = diffs.windows(2)
            .map(|numbers| {
                numbers[1] - numbers[0]
            })
            .collect();
    }

    let accumulator = match direction {
        Direction::Left => { |a, b| b - a }
        Direction::Right => { |a, b| a + b }
    };
    outer_most_vals.iter().rev().fold(0, accumulator)
}

pub fn solve_part_1(input: &str) -> i64 {
    let history_list: Vec<Vec<i64>> = build_history(input);
    history_list.iter().map(|history| find_next_value(history, Direction::Right)).sum()
}

pub fn solve_part_2(input: &str) -> i64 {
    let history_list: Vec<Vec<i64>> = build_history(input);
    history_list.iter().map(|history| find_next_value(history, Direction::Left)).sum()
}

fn build_history(input: &str) -> Vec<Vec<i64>> {
    input.lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|splits| splits.iter().map(|&part| part.parse::<i64>().unwrap()).collect::<Vec<i64>>())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_next_value_right() {
        assert_eq!(find_next_value(&vec![0, 3, 6, 9, 12, 15], Direction::Right), 18);
        assert_eq!(find_next_value(&vec![1, 3, 6, 10, 15, 21], Direction::Right), 28);
        assert_eq!(find_next_value(&vec![10, 13, 16, 21, 30, 45], Direction::Right), 68);
    }

    #[test]
    fn test_find_next_value_left() {
        assert_eq!(find_next_value(&vec![10, 13, 16, 21, 30, 45], Direction::Left), 5);
    }
}