use crate::days::seven::{part_one, part_two};
use anyhow::Result;

#[test]
pub fn part_one_solution() -> Result<()> {
    let crab_locations: Vec<i32> = include_str!("resources/input.txt")
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let solution = part_one(crab_locations)?;

    assert_eq!(solution, 343441);
    Ok(())
}

#[test]
pub fn part_two_solution() -> Result<()> {
    let crab_locations: Vec<i32> = include_str!("resources/input.txt")
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let solution = part_two(crab_locations)?;

    assert_eq!(solution, 98925151);
    Ok(())
}

#[test]
pub fn part_one_test_case() -> Result<()> {
    let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    let actual = part_one(input)?;

    assert_eq!(actual, 37);
    Ok(())
}

#[test]
pub fn part_two_test_case() -> Result<()> {
    let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    let actual = part_two(input)?;

    assert_eq!(actual, 168);
    Ok(())
}
