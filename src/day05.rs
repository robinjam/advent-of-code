use std::fs::read_to_string;
use regex::Regex;

enum Mode {
    CrateMover9000,
    CrateMover9001
}

type Stacks = [Vec<char>; 9];

#[derive(Clone, Debug, PartialEq)]
struct Instruction {
    count: usize,
    src_index: usize,
    dst_index: usize,
}

fn parse(buf: &str) -> (Stacks, Vec<Instruction>) {
    let mut blocks = buf.split(" 1   2   3   4   5   6   7   8   9 \n\n");
    (
        parse_stacks(blocks.next().unwrap()),
        parse_instructions(blocks.next().unwrap())
    )
}

fn parse_stacks(stacks: &str) -> Stacks {
    let mut result = Stacks::default();
    for line in stacks.lines().rev() {
        for (column, value) in line.chars().skip(1).step_by(4).enumerate() {
            if value.is_alphabetic() {
                result[column].push(value);
            }
        }
    }
    result
}

fn parse_instructions(instructions: &str) -> Vec<Instruction> {
    Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").
        unwrap().
        captures_iter(instructions).
        map(|matches| {
            Instruction{
                count: matches[1].parse().unwrap(),
                src_index: matches[2].parse::<usize>().unwrap() - 1,
                dst_index: matches[3].parse::<usize>().unwrap() - 1
            }
        }).
        collect()
}

fn apply_instructions(mode: Mode, mut stacks: Stacks, instructions: Vec<Instruction>) -> String {
    for instruction in instructions {
        let src_stack = &mut stacks[instruction.src_index];
        let mut removed_values = src_stack.split_off(src_stack.len() - instruction.count);
        match mode {
            Mode::CrateMover9000 => removed_values.reverse(),
            Mode::CrateMover9001 => ()
        }
        stacks[instruction.dst_index].append(&mut removed_values);
    }
    String::from_iter(stacks.map(|stack| stack.last().unwrap().clone()))
}

pub fn run() -> (String, String) {
    let (stacks, instructions) = parse(&read_to_string("data/05.txt").unwrap());
    let part1 = apply_instructions(Mode::CrateMover9000, stacks.clone(), instructions.clone());
    let part2 = apply_instructions(Mode::CrateMover9001, stacks, instructions);
    (part1, part2)
}

#[test]
fn test_parse_instructions() {
    assert_eq!(
        Vec::from([
            Instruction{ count: 2, src_index: 7, dst_index: 0 },
            Instruction{ count: 4, src_index: 8, dst_index: 7 }
        ]),
        parse_instructions("move 2 from 8 to 1\nmove 4 from 9 to 8")
    );
}

#[test]
fn test_parse_stacks() {
    assert_eq!(
        [
            vec!['N', 'B'],
            vec!['B'],
            vec!['Q'],
            vec!['R'],
            vec!['V'],
            vec!['F'],
            vec!['D', 'N'],
            vec!['F'],
            vec!['M', 'H'],
        ],
        parse_stacks("[B]                     [N]     [H]\n[N] [B] [Q] [R] [V] [F] [D] [F] [M]")
    );
}
