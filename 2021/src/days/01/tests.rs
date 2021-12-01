use anyhow::Result;
use crate::days::one::{part_one, part_two};

#[test]
pub fn part_one_solution() -> Result<()> {
    let input = include_str!("resources/input.txt").lines().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let result = part_one(&input)?;

    assert_eq!(result, 1374);
    Ok(())
}

#[test]
pub fn part_two_solution() -> Result<()> {
    let input = include_str!("resources/input.txt").lines().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let result = part_two(&input)?;

    println!("{:?}", result);
    Ok(())
}

#[test]
pub fn small_test_case() -> Result<()> {
    let input = vec![199,200,208,210,200,207,240,269,260,263];
    let actual = part_two(&input)?;

    assert_eq!(actual, 5);
    Ok(())
}
