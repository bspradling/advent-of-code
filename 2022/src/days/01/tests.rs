use crate::days::one::{part_one, part_two};
use anyhow::Result;

#[test]
pub fn part_one_solution() -> Result<()> {
    let input = include_str!("resources/input.txt")
        .split("\n\n")
        .map(|x| {
            x.lines()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let result = part_one(input)?;

    assert_eq!(result, 69795);
    Ok(())
}

#[test]
pub fn part_two_solution() -> Result<()> {
    let input = include_str!("resources/input.txt")
        .split("\n\n")
        .map(|x| {
            x.lines()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let result = part_two(input)?;

    assert_eq!(result, 208437);
    Ok(())
}
