use std::{collections::HashMap, fs::read_to_string, str::FromStr};

use anyhow::{anyhow, Context, Error, Result};

pub fn run() -> Result<(String, String)> {
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();
    for line in read_to_string("data/21.txt")?.lines() {
        let (name, job) = line
            .split_once(": ")
            .context(format!("Invalid format: {}", line))?;
        monkeys.insert(name.into(), job.parse()?);
    }

    let part1 = monkeys
        .get("root")
        .context("Can't find monkey named 'root'")?
        .evaluate(&monkeys)?;

    Ok((part1.to_string(), "TODO".into()))
}

enum Opcode {
    Add,
    Sub,
    Mul,
    Div,
}

enum Monkey {
    Constant(i64),
    Operation(Opcode, String, String),
}

impl Monkey {
    fn evaluate(&self, monkeys: &HashMap<String, Monkey>) -> Result<i64> {
        match self {
            Monkey::Constant(n) => Ok(*n),
            Monkey::Operation(op, a, b) => {
                let a = monkeys
                    .get(a)
                    .context(format!("Can't find monkey {}", a))?
                    .evaluate(monkeys)?;
                let b = monkeys
                    .get(b)
                    .context(format!("Can't find monkey {}", b))?
                    .evaluate(monkeys)?;
                match op {
                    Opcode::Add => Ok(a + b),
                    Opcode::Sub => Ok(a - b),
                    Opcode::Mul => Ok(a * b),
                    Opcode::Div => Ok(a / b),
                }
            }
        }
    }
}

impl FromStr for Monkey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Ok(n) = s.parse::<i64>() {
            Ok(Monkey::Constant(n))
        } else {
            let (lhs, rhs) = s.split_once(" ").context(format!("Invalid format: {}", s))?;
            let (opcode, rhs) = rhs.split_once(" ").context(format!("Invalid format: {}", s))?;
            let opcode = match opcode {
                "+" => Opcode::Add,
                "-" => Opcode::Sub,
                "*" => Opcode::Mul,
                "/" => Opcode::Div,
                _ => return Err(anyhow!("Invalid opcode: {}", opcode)),
            };
            Ok(Monkey::Operation(opcode, lhs.into(), rhs.into()))
        }
    }
}
