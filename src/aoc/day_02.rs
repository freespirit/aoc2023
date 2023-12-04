use std::fs;

#[derive(Debug, PartialEq)]
struct Set {
    red: i32,
    green: i32,
    blue: i32,
}

struct Game {
    id: i32,
    sets: Vec<Set>,
}

struct Bag {
    red: i32,
    green: i32,
    blue: i32,
}

fn is_valid(game: &Game, bag: &Bag) -> bool {
    return game.sets.iter().all(|set|
        set.red <= bag.red
            && set.green <= bag.green
            && set.blue <= bag.blue
    );
}

/// Build a game from a line of text.
fn build_game(line: &str) -> Game {
    let mut game = Game {
        id: 0,
        sets: Vec::new(),
    };

    let mut parts = line.split(": ");
    game.id = parts.next().unwrap().strip_prefix("Game ").unwrap().parse::<i32>().unwrap();

    let sets = parts.next().unwrap().split("; ");
    for set_string in sets {
        let colors = set_string.split(", ");
        let mut set = Set {
            red: 0,
            green: 0,
            blue: 0,
        };
        for color in colors {
            let mut color_parts = color.split(' ');
            let count = color_parts.next().unwrap().parse::<i32>().unwrap();
            let color = color_parts.next().unwrap();
            match color {
                "red" => set.red = count,
                "green" => set.green = count,
                "blue" => set.blue = count,
                _ => panic!("Unknown color {}", color),
            }
        }
        game.sets.push(set);
    }

    game
}

pub fn solve_part_1(input_file: &str) -> i32 {
    let bag = Bag { red: 12, green: 13, blue: 14 };

    return match fs::read_to_string(input_file) {
        Ok(contents) => {
            contents.lines()
                .map(build_game)
                .filter(|game| is_valid(game, &bag))
                .map(|game| game.id)
                .sum()
        }
        Err(error) => {
            println!("Error: {}", error);
            0
        }
    };
}

/// Find the fewest number of cubes of each color that could have been in the bag to make the game
/// possible.
fn find_min_set(game: &Game) -> Set {
    let min_set = game.sets.iter().fold(Set { red: 0, green: 0, blue: 0 }, |min, set| {
        Set {
            red: if set.red > min.red { set.red } else { min.red },
            green: if set.green > min.green { set.green } else { min.green },
            blue: if set.blue > min.blue { set.blue } else { min.blue },
        }
    });
    min_set
}

fn compute_power(set: &Set) -> i32 {
    set.red * set.green * set.blue
}

pub fn solve_part_2(input_file: &str) -> i32 {
    return match fs::read_to_string(input_file) {
        Ok(contents) => {
            contents.lines()
                .map(build_game)
                .map(|game: Game| find_min_set(&game))
                .map(|set: Set| compute_power(&set))
                .sum()
        }
        Err(error) => {
            println!("Error: {}", error);
            0
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::day_02::{Bag, find_min_set, Game, is_valid, Set};

    const TEST_BAG: Bag = Bag { red: 12, green: 13, blue: 14 };

    #[test]
    fn is_valid_works() {
        let game_1 = Game {
            id: 1,
            sets: vec![
                Set { red: 4, green: 0, blue: 3 },
                Set { red: 1, green: 2, blue: 6 },
                Set { red: 0, green: 2, blue: 0 },
            ],
        };
        assert!(is_valid(&game_1, &TEST_BAG));

        let game_2 = Game {
            id: 2,
            sets: vec![
                Set { red: 0, green: 2, blue: 1 },
                Set { red: 1, green: 3, blue: 4 },
                Set { red: 0, green: 1, blue: 1 },
            ],
        };
        assert!(is_valid(&game_2, &TEST_BAG));

        let game_3 = Game {
            id: 3,
            sets: vec![
                Set { red: 20, green: 8, blue: 6 },
                Set { red: 4, green: 13, blue: 5 },
                Set { red: 1, green: 5, blue: 0 },
            ],
        };
        assert_eq!(is_valid(&game_3, &TEST_BAG), false);

        let game_4 = Game {
            id: 4,
            sets: vec![
                Set { red: 3, green: 1, blue: 6 },
                Set { red: 6, green: 3, blue: 0 },
                Set { red: 14, green: 3, blue: 15 },
            ],
        };
        assert_eq!(is_valid(&game_4, &TEST_BAG), false);

        let game_5 = Game {
            id: 5,
            sets: vec![
                Set { red: 6, green: 3, blue: 1 },
                Set { red: 1, green: 2, blue: 2 },
            ],
        };
        assert!(is_valid(&game_5, &TEST_BAG));
    }

    #[test]
    fn find_min_set_works() {
        let game_1 = Game {
            id: 1,
            sets: vec![
                Set { red: 4, green: 0, blue: 3 },
                Set { red: 1, green: 2, blue: 6 },
                Set { red: 0, green: 2, blue: 0 },
            ],
        };
        assert_eq!(find_min_set(&game_1), Set { red: 4, green: 2, blue: 6 });

        let game_2 = Game {
            id: 2,
            sets: vec![
                Set { red: 0, green: 2, blue: 1 },
                Set { red: 1, green: 3, blue: 4 },
                Set { red: 0, green: 1, blue: 1 },
            ],
        };
        assert_eq!(find_min_set(&game_2), Set { red: 1, green: 3, blue: 4 });

        let game_3 = Game {
            id: 3,
            sets: vec![
                Set { red: 20, green: 8, blue: 6 },
                Set { red: 4, green: 13, blue: 5 },
                Set { red: 1, green: 5, blue: 0 },
            ],
        };
        assert_eq!(find_min_set(&game_3), Set { red: 20, green: 13, blue: 6 });

        let game_4 = Game {
            id: 4,
            sets: vec![
                Set { red: 3, green: 1, blue: 6 },
                Set { red: 6, green: 3, blue: 0 },
                Set { red: 14, green: 3, blue: 15 },
            ],
        };
        assert_eq!(find_min_set(&game_4), Set { red: 14, green: 3, blue: 15 });

        let game_5 = Game {
            id: 5,
            sets: vec![
                Set { red: 6, green: 3, blue: 1 },
                Set { red: 1, green: 2, blue: 2 },
            ],
        };
        assert_eq!(find_min_set(&game_5), Set { red: 6, green: 3, blue: 2 });
    }
}