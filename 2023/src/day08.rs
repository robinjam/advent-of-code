use std::{collections::HashMap, ops::Index, str::FromStr};

pub fn day08(input: &str) -> (String, String) {
    (part1(input), part2(input))
}

fn part1(input: &str) -> String {
    let (instructions, nodes) = parse(input);

    let mut current_node = "AAA";
    for (i, instruction) in instructions.enumerate() {
        if current_node == "ZZZ" {
            return i.to_string();
        }
        let (lhs, rhs) = nodes[&current_node];
        match instruction {
            Instruction::Left => current_node = lhs,
            Instruction::Right => current_node = rhs,
        }
    }

    unreachable!()
}

fn part2(input: &str) -> String {
    "".into()
}

fn parse(
    input: &str,
) -> (
    impl Iterator<Item = Instruction> + '_,
    HashMap<&str, (&str, &str)>,
) {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().map(Instruction::from).cycle();
    let nodes = lines
        .skip(1)
        .map(|line| {
            let name = &line[0..3];
            let lhs = &line[7..10];
            let rhs = &line[12..15];
            (name, (lhs, rhs))
        })
        .collect();
    (instructions, nodes)
}

enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(c: char) -> Self {
        match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!(),
        }
    }
}

#[test]
fn test() {
    assert_eq!(part1(include_str!("../examples/08_1.txt")), "2");
    assert_eq!(part1(include_str!("../examples/08_2.txt")), "6");
}
