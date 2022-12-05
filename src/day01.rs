use std::{fs::File, io::{BufReader, BufRead}};

pub fn run() {
    let file = File::open("data/01.txt").unwrap();
    let lines = BufReader::new(file).lines().map(|line| line.unwrap());
    
    let mut sums = vec![0];
    for line in lines {
        if line == "" {
            sums.push(0);
        }
        else {
            let prev_value = sums.pop().unwrap();
            sums.push(prev_value + line.parse::<i32>().unwrap());
        }
    }
    sums.sort();
    sums.reverse();

    println!("Day 01");
    println!("  Part 1: {}", sums.first().unwrap());
    println!("  Part 2: {}", sums.iter().take(3).sum::<i32>());
}
