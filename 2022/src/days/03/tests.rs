use crate::days::three::{part_one, part_two};
use anyhow::Result;

#[test]
pub fn part_one_solution() -> Result<()> {
    let input = include_str!("resources/input.txt")
        .lines()
        .map(|rucksack| {
            let middle = rucksack.len() / 2;
            (&rucksack[0..middle], &rucksack[middle..])
        })
        .collect::<Vec<(&str, &str)>>();

    let result = part_one(input)?;

    assert_eq!(result, 7581);
    Ok(())
}

#[test]
pub fn part_two_solution() -> Result<()> {
    let input = include_str!("resources/input.txt")
        .lines()
        .collect::<Vec<&str>>();

    let result = part_two(input)?;

    assert_eq!(result, 2525);
    Ok(())
}

#[test]
pub fn splitting() -> Result<()> {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    let result = input
        .lines()
        .map(|rucksack| {
            let middle = rucksack.len() / 2;
            (&rucksack[0..middle], &rucksack[middle..])
        })
        .collect::<Vec<(&str, &str)>>();

    assert_eq!(
        result,
        [
            ("vJrwpWtwJgWr", "hcsFMMfFFhFp"),
            ("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"),
            ("PmmdzqPrV", "vPwwTWBwg"),
            ("wMqvLMZHhHMvwLH", "jbvcjnnSBnvTQFn"),
            ("ttgJtRGJ", "QctTZtZT"),
            ("CrZsJsPPZsGz", "wwsLwLmpwMDw")
        ]
    );
    Ok(())
}

#[test]
pub fn ascii_values() -> Result<()> {
    assert_eq!('a' as u32 - 96, 1);
    assert_eq!('z' as u32 - 96, 26);
    assert_eq!('A' as u32 - 38, 27);
    assert_eq!('Z' as u32 - 38, 52);
    Ok(())
}
