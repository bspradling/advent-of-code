use crate::days::one::{part_one, part_two};

use anyhow::Result;

#[test]
pub fn part_one_solution() -> Result<()> {
    let input = include_str!("resources/input.txt");

    let numbers: Vec<i64> = input
        .lines()
        .map(|line| {
            line.parse::<i64>()
                .expect("Could not convert string to i32!")
        })
        .collect();

    assert_eq!(part_one(2020, &numbers)?, 1007104);
    Ok(())
}

#[test]
pub fn part_two_solution() -> Result<()> {
    let input = include_str!("resources/input.txt");

    let numbers: Vec<i64> = input
        .lines()
        .map(|line| {
            line.parse::<i64>()
                .expect("Could not convert string to i32!")
        })
        .collect();

    assert_eq!(part_two(2020, numbers)?, 18847752);
    Ok(())
}
