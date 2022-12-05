use std::{fs::File, io::{BufReader, BufRead}};

#[derive(Debug, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
struct Round {
    their_move: Move,
    my_move: Move,
}

enum Result {
    Lose,
    Draw,
    Win,
}

fn move_from_char(c: char) -> Move {
    match c {
        'A' => Move::Rock,
        'B' => Move::Paper,
        'C' => Move::Scissors,
        'X' => Move::Rock,
        'Y' => Move::Paper,
        'Z' => Move::Scissors,
        _ => panic!()
    }
}

fn parse_round(line: String) -> Round {
    let move_from_nth_char = |n| move_from_char(line.chars().nth(n).unwrap());
    Round{ their_move: move_from_nth_char(0), my_move: move_from_nth_char(2) }
}

#[test]
fn test_parse_round() {
    assert_eq!(Round{ their_move: Move::Rock, my_move: Move::Paper }, parse_round("A Y".into()));
    assert_eq!(Round{ their_move: Move::Paper, my_move: Move::Rock }, parse_round("B X".into()));
    assert_eq!(Round{ their_move: Move::Scissors, my_move: Move::Scissors }, parse_round("C Z".into()));
}

fn result(round: Round) -> Result {
    match (round.my_move, round.their_move) {
        (Move::Rock, Move::Scissors) => Result::Win,
        (Move::Paper, Move::Rock) => Result::Win,
        (Move::Scissors, Move::Paper) => Result::Win,

        (Move::Rock, Move::Rock) => Result::Draw,
        (Move::Paper, Move::Paper) => Result::Draw,
        (Move::Scissors, Move::Scissors) => Result::Draw,

        (Move::Rock, Move::Paper) => Result::Lose,
        (Move::Paper, Move::Scissors) => Result::Lose,
        (Move::Scissors, Move::Rock) => Result::Lose,
    }
}

fn score(round: Round) -> i32 {
    let shape_score = match round.my_move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3
    };

    let outcome_score = match result(round) {
        Result::Lose => 0,
        Result::Draw => 3,
        Result::Win => 6
    };

    shape_score + outcome_score
}

#[test]
fn test_score() {
    assert_eq!(8, score(Round{ their_move: Move::Rock, my_move: Move::Paper }));
    assert_eq!(1, score(Round{ their_move: Move::Paper, my_move: Move::Rock }));
    assert_eq!(6, score(Round{ their_move: Move::Scissors, my_move: Move::Scissors }));
}

pub fn run() {
    let file = File::open("data/02.txt").unwrap();
    let lines = BufReader::new(file).lines().map(|line| line.unwrap());
    let score: i32 = lines.map(parse_round).map(score).sum();

    println!("Day 02");
    println!("  Part 1: {}", score);
}
