use crate::days::four::{part_one, part_two, Interval};
use anyhow::Result;

#[test]
pub fn part_one_solution() -> Result<()> {
    let result = include_str!("resources/input.txt")
        .lines()
        .map(|l| {
            l.split(",")
                .map(|i| Interval::parse(i).unwrap())
                .collect::<Vec<Interval>>()
        })
        .collect::<Vec<Vec<Interval>>>();

    let answer = part_one(result)?;

    assert_eq!(answer, 588);
    Ok(())
}

#[test]
pub fn part_two_solution() -> Result<()> {
    let result = include_str!("resources/input.txt")
        .lines()
        .map(|l| {
            l.split(",")
                .map(|i| Interval::parse(i).unwrap())
                .collect::<Vec<Interval>>()
        })
        .collect::<Vec<Vec<Interval>>>();

    let answer = part_two(result)?;

    assert_eq!(answer, 911);
    Ok(())
}

#[test]
pub fn part_one_test_case() -> Result<()> {
    let result = include_str!("resources/test_case.txt")
        .lines()
        .map(|l| {
            l.split(",")
                .map(|i| Interval::parse(i).unwrap())
                .collect::<Vec<Interval>>()
        })
        .collect::<Vec<Vec<Interval>>>();

    let answer = part_one(result)?;

    assert_eq!(answer, 2);
    Ok(())
}

#[test]
pub fn part_two_test_case() -> Result<()> {
    let result = include_str!("resources/test_case.txt")
        .lines()
        .map(|l| {
            l.split(",")
                .map(|i| Interval::parse(i).unwrap())
                .collect::<Vec<Interval>>()
        })
        .collect::<Vec<Vec<Interval>>>();

    let answer = part_two(result)?;

    assert_eq!(answer, 4);
    Ok(())
}
