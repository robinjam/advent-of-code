use std::fs::read_to_string;

pub fn run() -> (i32, i32) {
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

    (
        sums[0],
        sums[0..3].iter().sum()
    )
}
