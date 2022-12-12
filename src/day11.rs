use std::{str::FromStr, fs::read_to_string, mem::swap};

use anyhow::{Result, Error, anyhow, Context};

pub fn run() -> Result<(String, String)> {
    let monkeys = parse_monkeys(&read_to_string("data/11.txt").unwrap()).unwrap();
    
    let common_multiple = monkeys.iter().map(|m| m.test_divisor).product();
    let part1 = play_game(monkeys.clone(), 20, true, common_multiple);
    let part2 = play_game(monkeys, 10000, false, common_multiple);
    Ok((part1.to_string(), part2.to_string()))
}

fn play_game(mut monkeys: Vec<Monkey>, rounds: usize, part1: bool, common_multiple: u64) -> usize {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let mut monkey = Monkey::default();
            swap(&mut monkey, &mut monkeys[i]);
            for item in monkey.items.drain(..) {
                monkey.num_inspections += 1;
                let new_value = monkey.operation.evaluate(item);
                let new_score = if part1 { new_value / 3 } else { new_value % common_multiple };
                let next_target = if new_score % monkey.test_divisor == 0 {
                    monkey.target_if_true
                } else {
                    monkey.target_if_false
                };
                monkeys[next_target].items.push(new_score)
            }
            swap(&mut monkey, &mut monkeys[i]);
        }
    }
    monkey_business(&mut monkeys)
}

fn parse_monkeys(buf: &str) -> Result<Vec<Monkey>> {
    buf.
        split("\n\n").
        map(|line| Ok(line.parse()?)).
        collect()
}

fn monkey_business(monkeys: &mut Vec<Monkey>) -> usize {
    monkeys.sort_unstable_by(|a, b| b.num_inspections.cmp(&a.num_inspections));
    monkeys[0].num_inspections * monkeys[1].num_inspections
}

#[derive(Clone, Debug, Default, PartialEq)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test_divisor: u64,
    target_if_true: usize,
    target_if_false: usize,
    num_inspections: usize,
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(buf: &str) -> Result<Monkey> {
        let mut lines = buf.lines();
        let mut read = |range| -> Result<&str> {
            let line = lines.next().context("input exhausted")?;
            Ok(line.get(range).context(format!("line too short: {}", line))?)
        };
        let _ = read(0..);
        Ok(Monkey{
            items: read(18..)?.
                split(", ").
                map(|n| Ok(n.parse()?)).
                collect::<Result<_>>()?,
            operation: read(19..)?.parse()?,
            test_divisor: read(21..)?.parse()?,
            target_if_true: read(29..)?.parse()?,
            target_if_false: read(30..)?.parse()?,
            num_inspections: 0,
        })
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct Operation {
    opcode: Opcode,
    values: (Value, Value),
}

impl Operation {
    fn evaluate(&self, old_value: u64) -> u64 {
        let [a, b] = [&self.values.0, &self.values.1].map(|value|
            match *value {
                Value::Old => old_value,
                Value::New(new_value) => new_value,
            }
        );
        match self.opcode {
            Opcode::Add => a + b,
            Opcode::Mul => a * b,
        }
    }
}

impl FromStr for Operation {
    type Err = Error;

    fn from_str(buf: &str) -> Result<Operation> {
        let [a, op, b]: [&str; 3] = buf.
            split(" ").
            collect::<Vec<&str>>().
            as_slice().
            try_into()?;

        Ok(Operation {
            opcode: op.parse()?,
            values: (a.parse()?, b.parse()?)
        })
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum Opcode {
    #[default] Add,
    Mul,
}

impl FromStr for Opcode {
    type Err = Error;

    fn from_str(buf: &str) -> Result<Opcode> {
        match buf {
            "+" => Ok(Opcode::Add),
            "*" => Ok(Opcode::Mul),
            op => Err(anyhow!("invalid operand {}", op))
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum Value {
    #[default] Old,
    New(u64),
}

impl FromStr for Value {
    type Err = Error;

    fn from_str(buf: &str) -> Result<Value> {
        match buf {
            "old" => Ok(Value::Old),
            n => Ok(Value::New(n.parse()?))
        }
    }
}
