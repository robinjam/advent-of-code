use std::{fs::read_to_string, collections::HashSet};

fn priority(c: &char) -> i32 {
    match c {
        'a'..='z' => *c as i32 - 96,
        'A'..='Z' => *c as i32 - 38,
        _ => panic!()
    }
}

fn intersect(a: HashSet<char>, b: HashSet<char>) -> HashSet<char> {
    a.intersection(&b).cloned().collect()
}

fn common_chars(strings: &[&str]) -> HashSet<char> {
    strings.
        iter().
        map(|line| line.chars()).
        map(HashSet::from_iter).
        reduce(intersect).
        unwrap()
}

fn priority_of_common_chars(strings: &[&str]) -> i32 {
    common_chars(strings).
        iter().
        map(priority).
        sum()
}

pub fn run() -> (String, String) {
    let buf = read_to_string("data/03.txt").unwrap();
    let lines: Vec<_> = buf.lines().collect();

    let part1: i32 = lines.
        iter().
        map(|line| {
            let (a, b) = line.split_at(line.len() / 2);
            priority_of_common_chars(&[a, b])
        }).
        sum();
    
    let part2: i32 = lines.
        chunks(3).
        map(priority_of_common_chars).
        sum();

    (part1.to_string(), part2.to_string())
}

#[test]
fn test_priority() {
    assert_eq!(priority(&'a'), 1);
    assert_eq!(priority(&'z'), 26);
    assert_eq!(priority(&'A'), 27);
    assert_eq!(priority(&'Z'), 52);
}
