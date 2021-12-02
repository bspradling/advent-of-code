use crate::days::two::{part_one, part_two, Command};
use anyhow::Result;
use std::str::FromStr;

#[test]
pub fn part_one_solution() -> Result<()> {
    let input = include_str!("resources/input.txt")
        .lines()
        .map(|x| Command::from_str(x).unwrap())
        .collect::<Vec<Command>>();

    let result = part_one(&input)?;

    println!("{:?}", result);
    assert_eq!(result.0 * result.1, 1670340);
    Ok(())
}

#[test]
pub fn part_two_solution() -> Result<()> {
    let input = include_str!("resources/input.txt")
        .lines()
        .map(|x| Command::from_str(x).unwrap())
        .collect::<Vec<Command>>();
    let result = part_two(&input)?;

    println!("{:?}", result);
    assert_eq!(result.horizontal * result.depth, 1954293920);
    Ok(())
}

#[test]
pub fn part_one_test_case() -> Result<()> {
    let input = vec![
        Command::FORWARD(5),
        Command::DOWN(5),
        Command::FORWARD(8),
        Command::UP(3),
        Command::DOWN(8),
        Command::FORWARD(2),
    ];
    let actual = part_one(&input)?;

    println!("{:?}", &actual);
    assert_eq!(actual.0 * actual.1, 150);
    Ok(())
}

#[test]
pub fn part_two_test_case() -> Result<()> {
    let input = vec![
        Command::FORWARD(5),
        Command::DOWN(5),
        Command::FORWARD(8),
        Command::UP(3),
        Command::DOWN(8),
        Command::FORWARD(2),
    ];
    let actual = part_two(&input)?;

    println!("{:?}", &actual);
    assert_eq!(actual.horizontal * actual.depth, 900);
    Ok(())
}

#[test]
pub fn test_command_parsing() -> Result<()> {
    let input = vec!["up 5", "down 3", "forward 10"];
    let expected = vec![Command::UP(5), Command::DOWN(3), Command::FORWARD(10)];
    let actual = input
        .into_iter()
        .map(|x| Command::from_str(x).unwrap())
        .collect::<Vec<Command>>();

    assert_eq!(actual, expected);
    Ok(())
}
