/// Find the lowest time needed to beat the max distance.
fn find_lowest_winning_time(race_time: i64, max_distance: i64) -> i64 {
    let mut start = 0;
    let mut end = race_time / 2;
    while start < end {
        let time = start + (end - start) / 2;
        let distance = time * (race_time - time);
        if distance > max_distance {
            end = time;
        } else {
            start = time + 1;
        }
    }

    start
}


fn ways_to_win(time_ms: i64, max_distance: i64) -> i64 {
    let lowest_needed_time = find_lowest_winning_time(time_ms, max_distance);
    let ways = (time_ms - lowest_needed_time) - lowest_needed_time;
    ways + 1
}

pub fn solve_part_1(input: &str) -> i64 {
    let rows = input.lines().collect::<Vec<&str>>();
    let times = rows[0].strip_prefix("Time:").unwrap();
    let distances = rows[1].strip_prefix("Distance:").unwrap();

    let times: Vec<i64> = times.split_whitespace().filter_map(|n| n.parse::<i64>().ok())
        .collect();
    let distances: Vec<i64> = distances.split_whitespace().filter_map(|n| n.parse::<i64>().ok())
        .collect();

    times.iter().zip(distances.iter())
        .map(|(&t, &d)| ways_to_win(t, d))
        .product()
}

pub fn solve_part_2(input: &str) -> i64 {
    let rows = input.lines().collect::<Vec<&str>>();
    let times = rows[0].strip_prefix("Time:").unwrap();
    let distances = rows[1].strip_prefix("Distance:").unwrap();

    let time = times.split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<i64>().unwrap();
    let distance = distances.split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse::<i64>().unwrap();

    ways_to_win(time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ways_to_win() {
        assert_eq!(ways_to_win(7, 9), 4);
        assert_eq!(ways_to_win(15, 40), 8);
        assert_eq!(ways_to_win(30, 200), 9);

        assert_eq!(ways_to_win(71530, 940200), 71503);
    }

    #[test]
    fn test_find_lowest_winning_time() {
        assert_eq!(find_lowest_winning_time(7, 9), 2);
        assert_eq!(find_lowest_winning_time(15, 40), 4);
        assert_eq!(find_lowest_winning_time(30, 200), 11);

        assert_eq!(find_lowest_winning_time(71530, 940200), 14);
    }
}