use std::{fs::read_to_string, collections::HashSet};

fn find_start_of_packet(buf: &[char], marker_length: usize) -> usize {
    buf.
        windows(marker_length).
        position(|window| {
            let mut uniqe_chars = HashSet::new();
            window.iter().all(|char| uniqe_chars.insert(char))
        }).
        unwrap() + marker_length
}

pub fn run() -> (String, String) {
    let chars: Vec<char> = read_to_string("data/06.txt").unwrap().chars().collect();
    let part1 = find_start_of_packet(&chars, 4);
    let part2 = find_start_of_packet(&chars, 14);

    (part1.to_string(), part2.to_string())
}
