use crate::days::four::{part_one, part_two, BingoBoard, BingoState};
use anyhow::Result;
use std::collections::HashSet;

#[test]
pub fn part_one_solution() -> Result<()> {
    let (_, state) = BingoState::parse(include_str!("resources/input.txt"))?;
    let winner = part_one(state)?.unwrap();

    assert_eq!(winner.result()?, 10374);
    Ok(())
}

#[test]
pub fn part_two_solution() -> Result<()> {
    let (_, state) = BingoState::parse(include_str!("resources/input.txt"))?;
    let winner = part_two(state)?.unwrap();

    assert_eq!(winner.result()?, 24742);
    Ok(())
}

#[test]
pub fn part_one_test_case() -> Result<()> {
    let (_, state) = BingoState::parse(include_str!("resources/test_case.txt"))?;
    let winner = part_one(state)?.unwrap();

    assert_eq!(winner.result()?, 4512);
    Ok(())
}

#[test]
pub fn part_two_test_case() -> Result<()> {
    let (_, state) = BingoState::parse(include_str!("resources/test_case.txt"))?;
    let last_winner = part_two(state)?.unwrap();

    assert_eq!(last_winner.result()?, 1924);
    Ok(())
}

#[test]
pub fn test_digit_parsing() -> Result<()> {
    let input = "0";
    let (_, actual) = BingoBoard::parse_number(input)?;
    assert_eq!(actual, 0);
    Ok(())
}

#[test]
pub fn test_row_parsing_returns_successfully() -> Result<()> {
    let input = "0 1 2 3 4\n";
    let (rest, actual) = BingoBoard::parse_row(input)?;
    assert_eq!(rest, "");
    assert_eq!(actual, vec![0, 1, 2, 3, 4]);
    Ok(())
}

#[test]
pub fn test_row_whitespace_parsing_works_with_board() -> Result<()> {
    let input = "0 1 2 3 4
    5 6 7 8 9
    0 1 2 3 4
    5 6 7 8 9
    0 1 2 3 4
    ";

    let (rest, actual) = BingoBoard::parse_row(input)?;
    assert_eq!(
        rest,
        "5 6 7 8 9
    0 1 2 3 4
    5 6 7 8 9
    0 1 2 3 4
    "
    );
    assert_eq!(actual, vec![0, 1, 2, 3, 4]);
    Ok(())
}

#[test]
pub fn test_board_parsing() -> Result<()> {
    let input = "45 57 55 43 31
    32 52 79 65 80
    21 98 16 64  6
    19 78 48 59 51
    37  2 69 56 99
    ";

    let (rest, result) = BingoBoard::parse(input)?;

    assert_eq!(rest, "");
    assert_eq!(
        result,
        BingoBoard::new(vec![
            vec![45, 57, 55, 43, 31],
            vec![32, 52, 79, 65, 80],
            vec![21, 98, 16, 64, 6],
            vec![19, 78, 48, 59, 51],
            vec![37, 2, 69, 56, 99],
        ])?
    );

    Ok(())
}

#[test]
pub fn test_input_parsing() -> Result<()> {
    let input = include_str!("resources/input.txt");
    let (remaining, state) = BingoState::parse(input)?;

    assert_eq!(remaining, "");
    assert_eq!(state.numbers.is_empty(), false);
    assert_eq!(state.boards.is_empty(), false);
    Ok(())
}

#[test]
pub fn test_board_storage() -> Result<()> {
    let board = BingoBoard::new(vec![
        vec![45, 57, 55, 43, 31],
        vec![32, 52, 79, 65, 80],
        vec![21, 98, 16, 64, 6],
        vec![19, 78, 48, 59, 51],
        vec![37, 2, 69, 56, 99],
    ])?;

    assert_eq!(
        board.rows,
        vec![
            HashSet::from([45, 57, 55, 43, 31]),
            HashSet::from([32, 52, 79, 65, 80]),
            HashSet::from([21, 98, 16, 64, 6]),
            HashSet::from([19, 78, 48, 59, 51]),
            HashSet::from([37, 2, 69, 56, 99]),
        ]
    );
    assert_eq!(
        board.columns,
        vec![
            HashSet::from([45, 32, 21, 19, 37]),
            HashSet::from([57, 52, 98, 78, 2]),
            HashSet::from([55, 79, 16, 48, 69]),
            HashSet::from([43, 65, 64, 59, 56]),
            HashSet::from([31, 80, 6, 51, 99]),
        ]
    );
    Ok(())
}
