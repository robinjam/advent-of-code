use std::fs::read_to_string;

use anyhow::{anyhow, Result};

pub fn run() -> Result<(String, String)> {
    let numbers: Vec<_> = read_to_string("data/25.txt")?
        .lines()
        .map(snafu_to_dec)
        .collect::<Result<_>>()?;

    let part1 = numbers.iter().sum();

    Ok((dec_to_snafu(part1), "N/A".into()))
}

fn snafu_to_dec(snafu: &str) -> Result<i64> {
    let mut result = 0;
    for (i, c) in snafu.chars().rev().enumerate() {
        result += 5_i64.pow(i as u32)
            * match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => return Err(anyhow!("Invalid SNAFU character: {}", c)),
            }
    }
    Ok(result)
}

fn dec_to_snafu(mut dec: i64) -> String {
    let mut result = "".to_owned();
    while dec > 0 {
        result.insert(
            0,
            match dec % 5 {
                0 => '0',
                1 => '1',
                2 => '2',
                3 => '=',
                4 => '-',
                _ => unreachable!(),
            },
        );
        dec = (dec + 2) / 5;
    }
    result
}

#[test]
fn test_snafu_to_dec() {
    assert_eq!(1, snafu_to_dec("1").unwrap());
    assert_eq!(2, snafu_to_dec("2").unwrap());
    assert_eq!(3, snafu_to_dec("1=").unwrap());
    assert_eq!(4, snafu_to_dec("1-").unwrap());
    assert_eq!(5, snafu_to_dec("10").unwrap());
    assert_eq!(6, snafu_to_dec("11").unwrap());
    assert_eq!(7, snafu_to_dec("12").unwrap());
    assert_eq!(8, snafu_to_dec("2=").unwrap());
    assert_eq!(9, snafu_to_dec("2-").unwrap());
    assert_eq!(10, snafu_to_dec("20").unwrap());
    assert_eq!(15, snafu_to_dec("1=0").unwrap());
    assert_eq!(20, snafu_to_dec("1-0").unwrap());
    assert_eq!(2022, snafu_to_dec("1=11-2").unwrap());
    assert_eq!(12345, snafu_to_dec("1-0---0").unwrap());
    assert_eq!(314159265, snafu_to_dec("1121-1110-1=0").unwrap());
}

#[test]
fn test_dec_to_snafu() {
    assert_eq!(dec_to_snafu(1), "1");
    assert_eq!(dec_to_snafu(2), "2");
    assert_eq!(dec_to_snafu(3), "1=");
    assert_eq!(dec_to_snafu(4), "1-");
    assert_eq!(dec_to_snafu(5), "10");
    assert_eq!(dec_to_snafu(6), "11");
    assert_eq!(dec_to_snafu(7), "12");
    assert_eq!(dec_to_snafu(8), "2=");
    assert_eq!(dec_to_snafu(9), "2-");
    assert_eq!(dec_to_snafu(10), "20");
    assert_eq!(dec_to_snafu(15), "1=0");
    assert_eq!(dec_to_snafu(20), "1-0");
    assert_eq!(dec_to_snafu(2022), "1=11-2");
    assert_eq!(dec_to_snafu(12345), "1-0---0");
    assert_eq!(dec_to_snafu(314159265), "1121-1110-1=0");
}
