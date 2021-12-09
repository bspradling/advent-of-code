use crate::days::eight::{part_one, part_two, Digit, FooState};
use anyhow::Result;

#[test]
pub fn part_one_solution() -> Result<()> {
    let input: Vec<FooState> = include_str!("resources/input.txt")
        .lines()
        .map(|x| FooState::parse(x).unwrap().1)
        .collect();
    let solution = part_one(input)?;

    assert_eq!(solution, 272);
    Ok(())
}

#[test]
pub fn part_two_solution() -> Result<()> {
    let input: Vec<FooState> = include_str!("resources/input.txt")
        .lines()
        .map(|x| FooState::parse(x).unwrap().1)
        .collect();
    let solution = part_two(input)?;

    assert_eq!(solution, 1007675);
    Ok(())
}

#[test]
pub fn part_one_test_case() -> Result<()> {
    let input: Vec<FooState> = include_str!("resources/test_case.txt")
        .lines()
        .map(|x| FooState::parse(x).unwrap().1)
        .collect();

    let actual = part_one(input)?;

    assert_eq!(actual, 26);
    Ok(())
}

#[test]
pub fn part_two_test_case() -> Result<()> {
    let input: Vec<FooState> = include_str!("resources/test_case.txt")
        .lines()
        .map(|x| FooState::parse(x).unwrap().1)
        .collect();

    let actual = part_two(input)?;
    assert_eq!(actual, 61229);
    Ok(())
}

#[test]
pub fn test_input_parsing() -> Result<()> {
    let input =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";
    let (rest, state) = FooState::parse(input)?;

    assert_eq!(rest, "");
    assert_eq!(state.signals.len(), 10);
    assert_eq!(state.display.len(), 4);

    Ok(())
}

#[test]
pub fn test_digit_parsing() -> Result<()> {
    let input = "cfbegad ";
    let (rest, digit) = Digit::parse(input)?;

    assert_eq!(rest, "");
    assert_eq!(
        digit,
        Digit {
            segments: vec!['c', 'f', 'b', 'e', 'g', 'a', 'd'],
        }
    );
    Ok(())
}

#[test]
pub fn determine_signals_test() -> Result<()> {
    let input =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    let (_, result) = FooState::parse(input)?;
    let _map = result.decode_signals()?;
    Ok(())
}

#[test]
pub fn test_bug() -> Result<()> {
    let input =
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef";
    let (_, x) = FooState::parse(input)?;
    let i = part_two(vec![x])?;
    assert_eq!(i, 1625);
    Ok(())
}
