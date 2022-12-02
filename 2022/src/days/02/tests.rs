use crate::days::two::RockPaperScissors;
use crate::days::two::{part_one, part_two};
use anyhow::Result;

#[test]
pub fn part_one_solution() -> Result<()> {
    let input = include_str!("resources/input.txt")
        .lines()
        .map(|x| RockPaperScissors::from(x))
        .collect::<Vec<RockPaperScissors>>();

    let result = part_one(input)?;

    assert_eq!(result, 10718);
    Ok(())
}

#[test]
pub fn part_two_solution() -> Result<()> {
    let input = include_str!("resources/input.txt")
        .lines()
        .map(|x| RockPaperScissors::from_outcome(x))
        .collect::<Vec<RockPaperScissors>>();

    let result = part_two(input)?;

    assert_eq!(result, 14652);
    Ok(())
}
