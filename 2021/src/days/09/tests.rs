use crate::days::nine::{part_one, part_two};
use anyhow::Result;

#[test]
pub fn part_one_solution() -> Result<()> {
    let input = include_str!("resources/input.txt")
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let low_points = part_one(&input)?;
    let solution: u32 = low_points.iter().map(|x| x.value).sum::<u32>() + low_points.len() as u32;
    assert_eq!(solution, 550);
    Ok(())
}

#[test]
pub fn part_two_solution() -> Result<()> {
    let input = include_str!("resources/input.txt")
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut result = part_two(&input)?;
    result.sort();
    let actual = result.iter().rev().take(3).product::<i32>();
    assert_eq!(actual, 1100682);
    Ok(())
}

#[test]
pub fn part_one_test_case() -> Result<()> {
    let input = include_str!("resources/test_case.txt")
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let low_points = part_one(&input)?;

    let actual: u32 = low_points.iter().map(|x| x.value).sum::<u32>() + low_points.len() as u32;
    assert_eq!(actual, 15);
    Ok(())
}

#[test]
pub fn part_two_test_case() -> Result<()> {
    let input = include_str!("resources/test_case.txt")
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut result = part_two(&input)?;
    result.sort();
    let actual = result.iter().rev().take(3).product::<i32>();
    assert_eq!(actual, 1134);
    Ok(())
}
