use crate::days::five::{part_one, OceanFloor};
use anyhow::Result;

#[test]
pub fn part_one_test_case() -> Result<()> {
    let input = include_str!("resources/test_case.txt");

    let _actual = part_one(OceanFloor::parse(input)?)?;

    // assert_eq!(actual, 5);

    Ok(())
}

#[test]
pub fn part_two_test_case() -> Result<()> {
    Ok(())
}
