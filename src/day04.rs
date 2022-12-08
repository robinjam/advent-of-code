use std::{fs::read_to_string, ops::{RangeInclusive}};

fn pair_from_iter<I, T>(mut iter: I) -> (T, T) where I: Iterator<Item = T> {
    (iter.next().unwrap(), iter.next().unwrap())
}

fn range_from_str(s: &str) -> RangeInclusive<i32> {
    let (a, b) = pair_from_iter(
        s.split("-").map(|part| part.parse::<i32>().unwrap())
    );
    a..=b
}

fn contains(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    b.start() >= a.start() && b.end() <= a.end()
}

fn overlaps(a: &RangeInclusive<i32>, b: &RangeInclusive<i32>) -> bool {
    a.start() <= b.end() && a.end() >= b.start()
}

pub fn run() -> (String, String) {
    let buf = read_to_string("data/04.txt").unwrap();

    let ranges = buf.
        lines().
        map(|line| pair_from_iter(line.split(",").map(range_from_str)));

    let part1 = ranges.clone().
        filter(|(a, b)| contains(&a, &b) || contains(&b, &a)).
        count();

    let part2 = ranges.
        filter(|(a, b)| overlaps(a, b))
        .count();

    (part1.to_string(), part2.to_string())
}
