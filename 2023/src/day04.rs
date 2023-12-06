use std::{collections::HashSet, str::FromStr};

pub fn day04(input: &str) -> (String, String) {
    (part1(input), part2(input))
}

fn part1(input: &str) -> String {
    cards_from(input)
        .map(|card| card.points())
        .sum::<u32>()
        .to_string()
}

fn part2(input: &str) -> String {
    let cards = cards_from(input);
    let mut card_counts = vec![1; cards.clone().count()];
    for (i, card) in cards.enumerate() {
        for j in (i + 1)..(i + 1 + card.winning_numbers()) {
            card_counts[j] += card_counts[i];
        }
    }
    card_counts.iter().sum::<u32>().to_string()
}

fn cards_from(input: &str) -> impl Iterator<Item = Card> + Clone + '_ {
    input.lines().filter_map(|line| Card::from_str(line).ok())
}

#[derive(Debug, PartialEq, Eq)]
struct Card {
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

impl Card {
    fn winning_numbers(&self) -> usize {
        self.my_numbers.intersection(&self.winning_numbers).count()
    }

    fn points(&self) -> u32 {
        let num_winners = self.winning_numbers() as u32;
        if num_winners > 0 {
            u32::pow(2, num_winners - 1)
        } else {
            0
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCardError;

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split_once(": ").ok_or(ParseCardError)?.1;
        let (winning_numbers, my_numbers) = s.split_once(" | ").ok_or(ParseCardError)?;
        let winning_numbers = winning_numbers
            .split(" ")
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();
        let my_numbers = my_numbers
            .split(" ")
            .filter_map(|s| s.parse::<u32>().ok())
            .collect();
        Ok(Card {
            winning_numbers,
            my_numbers,
        })
    }
}

#[test]
fn test_card_from_str() {
    assert_eq!(
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".parse(),
        Ok(Card {
            winning_numbers: [41, 48, 83, 86, 17].into(),
            my_numbers: [83, 86, 6, 31, 17, 9, 48, 53].into(),
        })
    );
}

#[test]
fn test_card_points() {
    assert_eq!(
        Card {
            winning_numbers: [13, 32, 20, 16, 61].into(),
            my_numbers: [61, 30, 68, 82, 17, 32, 24, 19].into(),
        }
        .points(),
        2
    );
    assert_eq!(
        Card {
            winning_numbers: [41, 92, 73, 84, 69].into(),
            my_numbers: [59, 84, 76, 51, 58, 5, 54, 83].into(),
        }
        .points(),
        1
    );
    assert_eq!(
        Card {
            winning_numbers: [87, 83, 26, 28, 32].into(),
            my_numbers: [88, 30, 70, 12, 93, 22, 82, 36].into(),
        }
        .points(),
        0
    );
}

#[test]
fn test() {
    let input = include_str!("../examples/04.txt");
    assert_eq!(part1(input), "13");
    assert_eq!(part2(input), "30");
}
