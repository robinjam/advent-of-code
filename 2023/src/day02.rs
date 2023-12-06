use std::str::FromStr;

pub fn day02(input: &str) -> (String, String) {
    (part1(input), part2(input))
}

fn part1(input: &str) -> String {
    games_from(input)
        .filter(|game| {
            game.rounds
                .iter()
                .all(|round| round.red <= 12 && round.green <= 13 && round.blue <= 14)
        })
        .map(|game| game.id)
        .sum::<u32>()
        .to_string()
}

fn part2(input: &str) -> String {
    games_from(input)
        .map(|game| {
            let min = game.rounds.iter().fold(Round::default(), |mut acc, next| {
                acc.red = u32::max(acc.red, next.red);
                acc.green = u32::max(acc.green, next.green);
                acc.blue = u32::max(acc.blue, next.blue);
                acc
            });
            min.red * min.green * min.blue
        })
        .sum::<u32>()
        .to_string()
}

fn games_from(input: &str) -> impl Iterator<Item = Game> + '_ {
    input.lines().filter_map(|line| Game::from_str(line).ok())
}

#[derive(Debug, PartialEq, Eq, Default)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rounds) = s.split_once(": ").ok_or(ParseGameError)?;
        let id = id
            .split_once(" ")
            .ok_or(ParseGameError)?
            .1
            .parse()
            .or(Err(ParseGameError))?;
        let rounds = rounds
            .split("; ")
            .map(Round::from_str)
            .collect::<Result<Vec<_>, _>>()
            .or(Err(ParseGameError))?;
        Ok(Game { id, rounds })
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseRoundError;

impl FromStr for Round {
    type Err = ParseRoundError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Round::default();
        for entry in s.split(", ") {
            let (count, colour) = entry.split_once(" ").ok_or(ParseRoundError)?;
            let count: u32 = count.parse().or(Err(ParseRoundError))?;
            match colour {
                "red" => result.red += count,
                "green" => result.green += count,
                "blue" => result.blue += count,
                _ => return Err(ParseRoundError),
            };
        }
        Ok(result)
    }
}

#[test]
fn test() {
    let input = include_str!("../examples/02.txt");
    assert_eq!(part1(input), "8");
    assert_eq!(part2(input), "2286");
}

#[test]
fn test_round_from_string() {
    assert_eq!(
        "3 blue, 4 red".parse(),
        Ok(Round {
            red: 4,
            blue: 3,
            green: 0
        })
    );
}

#[test]
fn test_game_from_string() {
    assert_eq!(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".parse(),
        Ok(Game {
            id: 1,
            rounds: vec![
                Round {
                    red: 4,
                    green: 0,
                    blue: 3
                },
                Round {
                    red: 1,
                    green: 2,
                    blue: 6
                },
                Round {
                    red: 0,
                    green: 2,
                    blue: 0
                }
            ],
        })
    );
}
