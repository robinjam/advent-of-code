use itertools::Itertools;

pub fn day11(input: &str) -> (String, String) {
    (part1(input), part2(input))
}

fn part1(input: &str) -> String {
    solve(input, 2)
}

fn part2(input: &str) -> String {
    solve(input, 1000000)
}

fn solve(input: &str, expansion_factor: usize) -> String {
    let universe = Universe::from(input);
    universe
        .galaxies
        .iter()
        .tuple_combinations()
        .map(|(a, b)| universe.distance_with_expansion(a, b, expansion_factor))
        .sum::<usize>()
        .to_string()
}

#[derive(Default)]
struct Universe {
    galaxies: Vec<Galaxy>,
}

impl Universe {
    fn distance_with_expansion(&self, a: &Galaxy, b: &Galaxy, expansion_factor: usize) -> usize {
        let empty_columns = (usize::min(a.x, b.x)..=usize::max(a.x, b.x))
            .filter(|x| !self.galaxies.iter().any(|g| g.x == *x))
            .count();
        let empty_rows = (usize::min(a.y, b.y)..=usize::max(a.y, b.y))
            .filter(|y| !self.galaxies.iter().any(|g| g.y == *y))
            .count();
        Galaxy::distance(&a, &b) + (expansion_factor - 1) * (empty_columns + empty_rows)
    }
}

impl From<&str> for Universe {
    fn from(value: &str) -> Self {
        let mut galaxies = vec![];
        for (y, line) in value.trim_end().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    galaxies.push(Galaxy { x, y });
                }
            }
        }
        Universe { galaxies }
    }
}

struct Galaxy {
    x: usize,
    y: usize,
}

impl Galaxy {
    fn distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[test]
fn test_galaxy_distance() {
    assert_eq!(
        Galaxy::distance(&Galaxy { x: 1, y: 5 }, &Galaxy { x: 5, y: 0 }),
        9
    );
}

#[test]
fn test_distance_with_expansion() {
    let universe = Universe::from(include_str!("../examples/11.txt"));
    assert_eq!(
        universe.distance_with_expansion(&universe.galaxies[4], &universe.galaxies[8], 2),
        9
    );
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("../examples/11.txt"), 2), "374");
    assert_eq!(solve(include_str!("../examples/11.txt"), 10), "1030");
    assert_eq!(solve(include_str!("../examples/11.txt"), 100), "8410");
}
