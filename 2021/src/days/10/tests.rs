use crate::days::ten::{part_one, part_two};
use anyhow::Result;

#[test]
pub fn part_one_solution() -> Result<()> {
    let input = include_str!("resources/input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let actual = part_one(input)?;

    assert_eq!(actual, 370407);
    Ok(())
}

#[test]
pub fn part_two_solution() -> Result<()> {
    let input = include_str!("resources/input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let actual = part_two(input)?;

    assert_eq!(actual, 3249889609);
    Ok(())
}

#[test]
pub fn part_one_test_case() -> Result<()> {
    let input = include_str!("resources/test-case.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let actual = part_one(input)?;

    assert_eq!(actual, 26397);
    Ok(())
}

#[test]
pub fn part_two_test_case() -> Result<()> {
    let input = include_str!("resources/test-case.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let actual = part_two(input)?;

    assert_eq!(actual, 288957);
    Ok(())
}

#[test]
pub fn test_single_line() -> Result<()> {
    let input = "[({(<(())[]>[[{[]{<()<>>";
    let _result = part_one(vec![input.chars().collect()])?;
    Ok(())
}
