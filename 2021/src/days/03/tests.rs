use crate::days::three::{part_one, part_two};
use anyhow::Result;

#[test]
pub fn part_one_solution() -> Result<()> {
    let report = include_str!("resources/input.txt")
        .lines()
        .collect::<Vec<&str>>();
    let actual = part_one(report)?;

    assert_eq!(actual, 749376);
    Ok(())
}

#[test]
pub fn part_two_solution() -> Result<()> {
    let report = include_str!("resources/input.txt")
        .lines()
        .collect::<Vec<&str>>();
    let actual = part_two(report)?;

    assert_eq!(actual, 2372923);
    Ok(())
}

#[test]
pub fn part_one_test_case() -> Result<()> {
    let input = vec![
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    assert_eq!(part_one(input)?, 198);
    Ok(())
}

#[test]
pub fn part_two_test_case() -> Result<()> {
    let input = vec![
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    assert_eq!(part_two(input)?, 230);
    Ok(())
}
