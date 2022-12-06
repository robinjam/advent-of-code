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

#[derive(Debug, PartialEq)]
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

fn result_from_char(c: char) -> Result {
    match c {
        'X' => Result::Lose,
        'Y' => Result::Draw,
        'Z' => Result::Win,
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

fn parse_move_and_result(line: String) -> (Move, Result) {
    let nth_char = |n| line.chars().nth(n).unwrap();
    (move_from_char(nth_char(0)), result_from_char(nth_char(2)))
}

#[test]
fn test_parse_move_and_result() {
    assert_eq!((Move::Rock, Result::Draw), parse_move_and_result("A Y".into()));
    assert_eq!((Move::Paper, Result::Lose), parse_move_and_result("B X".into()));
    assert_eq!((Move::Scissors, Result::Win), parse_move_and_result("C Z".into()));
}

fn choose_strategy(their_move: Move, desired_result: Result) -> Round {
    let my_move = match (&their_move, &desired_result) {
        (Move::Rock, Result::Win) => Move::Paper,
        (Move::Paper, Result::Win) => Move::Scissors,
        (Move::Scissors, Result::Win) => Move::Rock,

        (Move::Rock, Result::Draw) => Move::Rock,
        (Move::Paper, Result::Draw) => Move::Paper,
        (Move::Scissors, Result::Draw) => Move::Scissors,

        (Move::Rock, Result::Lose) => Move::Scissors,
        (Move::Paper, Result::Lose) => Move::Rock,
        (Move::Scissors, Result::Lose) => Move::Paper,
    };

    Round{ their_move, my_move }
}

#[test]
fn test_choose_strategy() {
    assert_eq!(Round{ their_move: Move::Rock, my_move: Move::Rock }, choose_strategy(Move::Rock, Result::Draw));
    assert_eq!(Round{ their_move: Move::Paper, my_move: Move::Rock }, choose_strategy(Move::Paper, Result::Lose));
    assert_eq!(Round{ their_move: Move::Scissors, my_move: Move::Rock }, choose_strategy(Move::Scissors, Result::Win));
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
    let read_lines = || BufReader::new(File::open("data/02.txt").unwrap()).lines().map(|line| line.unwrap());

    let part_1_score: i32 = read_lines().map(parse_round).map(score).sum();
    let part_2_score: i32 = read_lines().map(parse_move_and_result).map(|(their_move, desired_result)| choose_strategy(their_move, desired_result)).map(score).sum();

    println!("Day 02");
    println!("  Part 1: {}", part_1_score);
    println!("  Part 2: {}", part_2_score);
}
