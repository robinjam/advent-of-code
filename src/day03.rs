use std::{fs::read_to_string, collections::HashSet};

fn priority(c: &char) -> i32 {
    match c {
        'a'..='z' => *c as i32 - 96,
        'A'..='Z' => *c as i32 - 38,
        _ => panic!()
    }
}

pub fn run() -> (i32, i32) {
    let part1: i32 = read_to_string("data/03.txt").
        unwrap().
        lines().
        map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            let a_chars: HashSet<char> = a.chars().collect();
            let b_chars: HashSet<char> = b.chars().collect();
            let common_chars = a_chars.intersection(&b_chars);
            common_chars.map(priority).sum::<i32>()
        }).
        sum();

    (part1, 0)
}

#[test]
fn test_priority() {
    assert_eq!(priority(&'a'), 1);
    assert_eq!(priority(&'z'), 26);
    assert_eq!(priority(&'A'), 27);
    assert_eq!(priority(&'Z'), 52);
}
