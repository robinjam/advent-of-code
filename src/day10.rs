use std::{fs::read_to_string, ops::Generator, iter::from_generator};

use itertools::Itertools;

fn create_generator() -> impl Generator<Yield = i32, Return = ()> {
    || {
        let mut x = 1;
        yield x;

        let lines: Vec<String> = read_to_string("data/10.txt").
            unwrap().
            lines().
            map(|line| line.to_owned()).
            collect();

        for line in lines {
            match &line[0..4] {
                "noop" => {
                    yield x;
                },
                "addx" => {
                    yield x;
                    x += line[5..].parse::<i32>().unwrap();
                    yield x;
                },
                _ => panic!()
            }
        }
    }
}

fn part1() -> i32 {
    let values: Vec<i32> = from_generator(create_generator()).collect();
    (20..=220).step_by(40).map(|i| i * values[i as usize - 1]).sum()
}

fn part2() -> String {
    from_generator(create_generator()).
        enumerate().
        map(|(idx, x)| {
            if (x - 1..=x + 1).contains(&(idx as i32 % 40)) {
                '#'
            }
            else {
                '.'
            }
        }).
        chunks(40).into_iter().
        map(|chunk| chunk.into_iter().join("")).
        join("\n")
}

pub fn run() -> (String, String) {
    (part1().to_string(), "\n".to_owned() + &part2())
}
