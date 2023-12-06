pub fn day01(input: &str) -> (String, String) {
    (part1(input), part2(input))
}

fn part1(input: &str) -> String {
    solve(input, |line| line.chars().filter_map(|c| c.to_digit(10)))
}

fn part2(input: &str) -> String {
    const DIGITS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    solve(input, |line| {
        (0..line.len())
            .map(|i| line.split_at(i).1)
            .filter_map(|substring| {
                if let Some(digit) = substring.chars().next().unwrap().to_digit(10) {
                    Some(digit)
                } else if let Some(i) = DIGITS.iter().position(|s| substring.starts_with(s)) {
                    Some(i as u32 + 1)
                } else {
                    None
                }
            })
    })
}

fn solve<'a, T, U>(input: &'a str, digits_from_line: T) -> String
where
    T: Fn(&'a str) -> U,
    U: DoubleEndedIterator<Item = u32> + Clone,
{
    input
        .lines()
        .filter_map(|line| {
            let mut digits = digits_from_line(line);
            Some(10 * digits.clone().next()? + digits.next_back()?)
        })
        .fold(0, |a, b| a + b)
        .to_string()
}

#[test]
fn test() {
    assert_eq!(part1(include_str!("../examples/01_1.txt")), "142");
    assert_eq!(part2(include_str!("../examples/01_2.txt")), "281");
}
