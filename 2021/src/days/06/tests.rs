use anyhow::Result;
use crate::days::six::{part_one, part_two};

#[test]
pub fn part_one_solution() -> Result<()> {
    let input = include_str!("resources/input.txt");
    let day_0 = input.split(",").map(|x| x.parse::<u64>().unwrap()).collect();
    let laternfishes = part_one(day_0, 80)?;

    assert_eq!(laternfishes.len(), 354564);

    Ok(())
}


#[test]
pub fn part_two_solution() -> Result<()> {
    let input = include_str!("resources/input.txt");
    let day_0 = input.split(",").map(|x| x.parse::<u64>().unwrap()).collect();
    let laternfishes = part_two(day_0, 256)?;

    assert_eq!(laternfishes.iter().sum::<u64>(), 1609058859115);

    Ok(())
}

#[test]
pub fn part_one_test_case() -> Result<()> {
    let input = include_str!("resources/test_case.txt");
    let day_0 = input.split(",").map(|x| x.parse::<u64>().unwrap()).collect();
    let laternfishes = part_one(day_0, 18)?;

    assert_eq!(laternfishes.len(), 26);

    Ok(())
}

#[test]
pub fn part_two_test_case() -> Result<()> {
    let input = include_str!("resources/test_case.txt");
    let day_0 = input.split(",").map(|x| x.parse::<u64>().unwrap()).collect();
    let laternfishes = part_two(day_0, 256)?;

    assert_eq!(laternfishes.into_iter().sum::<u64>(), 26984457539);

    Ok(())
}
