use std::fs::read_to_string;

pub fn run() {
    let mut sums: Vec<i32> =
        read_to_string("data/01.txt").
        unwrap().
        trim_end().
        split("\n\n").
        map(|block| {
            block.
                lines().
                map(|line| line.parse::<i32>().unwrap()).
                sum()
        }).
        collect();

    sums.sort_unstable_by(|a, b| b.cmp(a));

    println!("Day 01");
    println!("  Part 1: {}", sums[0]);
    println!("  Part 2: {}", sums[0..3].iter().sum::<i32>());
}
