use std::fs;

pub fn solve_part_1(input_file: &str) -> i32 {
    return match fs::read_to_string(input_file) {
        Ok(schematic) => { compute_scores(schematic.as_str()).iter().sum() }
        Err(_) => { 0 }
    };
}

pub fn solve_part_2(input_file: &str) -> i32 {
    return match fs::read_to_string(input_file) {
        Ok(schematic) => {
            fill_copies(schematic.as_str()).len() as i32
        }
        Err(_) => { 0 }
    };
}


/// Compute an i32 score for each card in the table.
fn compute_scores(cards_table: &str) -> Vec<i32> {
    cards_table.lines()
        .map(winning_numbers_in_card)
        .map(|numbers|numbers.len() as i32)
        .map(|n| if n > 0 {
            2i32.pow((n - 1) as u32)
        } else {
            0
        })
        .collect()
}

fn winning_numbers_in_card(card: &str) -> Vec<i32> {
    let (card_id, numbers) = card.split_once(':').unwrap();
    let (winning_numbers, card_numbers) = numbers.split_once('|').unwrap();

    let winning_numbers = winning_numbers
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let card_numbers = card_numbers
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    card_numbers.into_iter()
        .filter(|n| winning_numbers.contains(n))
        .into_iter().collect::<Vec<i32>>()
}

fn fill_copies(cards_table: &str) -> Vec<usize> {
    let mut indices: Vec<usize> = (0usize..cards_table.lines().count()).collect();

    let num_winning_in_card: Vec<usize> = cards_table.lines()
        .map(winning_numbers_in_card)
        .map(|numbers| numbers.len() as usize)
        .collect();

    for (card_id, numbers) in num_winning_in_card.into_iter().enumerate() {
        for j in card_id +1..=(card_id + numbers) {
            let duplicates = indices.iter().filter(|&k| k == &card_id).count();
            (0..duplicates).for_each(|_k| indices.push(j));
        }
    }

    indices.sort();
    indices
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CARDS_TABLE: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_compute_scores() {
        assert_eq!(compute_scores(TEST_CARDS_TABLE), vec![8, 2, 2, 1, 0, 0]);
    }

    #[test]
    fn test_fill_copies() {
        assert_eq!(fill_copies(TEST_CARDS_TABLE).iter().count(), 30);
    }
}