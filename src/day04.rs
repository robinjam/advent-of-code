use std::{fs::read_to_string, ops::{RangeInclusive}};

fn range_from_str(s: &str) -> RangeInclusive<i32> {
    let mut parts = s.split("-").map(|part| part.parse::<i32>().unwrap());
    let a = parts.next().unwrap();
    let b = parts.next().unwrap();

    a..=b
}

fn contains(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    b.start() >= a.start() && b.end() <= a.end()
}

fn overlaps(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    a.start() <= b.end() && a.end() >= b.start()
}

pub fn run() -> (i32, i32) {
    let buf = read_to_string("data/04.txt").unwrap();

    let foo = buf.
        lines().
        map(|line| {
            let mut parts = line.split(",");
            (
                range_from_str(parts.next().unwrap()),
                range_from_str(parts.next().unwrap())
            )
        });

    let part1 = foo.clone().
        filter(|(a, b)| {
            contains(&a, &b) || contains(&b, &a)
        }).
        count() as i32;
    
    let part2 = foo.filter(|(a, b)| overlaps(a, b)).count() as i32;

    (part1, part2)
}
