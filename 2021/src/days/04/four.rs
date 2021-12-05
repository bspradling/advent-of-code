use self::nom::bytes::complete::tag;
use self::nom::character::complete::{digit1, multispace0, space1};
use self::nom::combinator::{eof, map_res};
use self::nom::multi::{many1, many_m_n, many_till, separated_list1};
use self::nom::sequence::terminated;
use self::nom::IResult;
use anyhow::Result;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::str::FromStr;

extern crate nom;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BingoBoard {
    pub sum: u32,
    pub rows: Vec<HashSet<u32>>,
    pub columns: Vec<HashSet<u32>>,
}

impl BingoBoard {
    pub fn new(numbers: Vec<Vec<u32>>) -> Result<Self> {
        Ok(BingoBoard {
            sum: numbers.iter().flatten().sum(),
            rows: numbers
                .iter()
                .map(|row| HashSet::from_iter(row.iter().cloned()))
                .collect(),
            columns: (0..numbers[0].len())
                .map(|i| numbers.iter().map(|inner| inner[i].clone()).collect())
                .collect(),
        })
    }

    pub fn winner(&self, numbers: &Vec<u32>) -> bool {
        self.rows
            .iter()
            .chain(self.columns.iter())
            .find(|x| x.is_subset(&numbers.iter().map(|x| x.clone()).collect()))
            .is_some()
    }

    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, numbers) = terminated(many_m_n(0, 5, Self::parse_row), multispace0)(input)?;
        assert_eq!(
            numbers.len(),
            5,
            "A Bingo Board must have exactly 5 rows in a board!"
        );
        Ok((rest, BingoBoard::new(numbers).unwrap()))
    }

    fn parse_row(input: &str) -> IResult<&str, Vec<u32>> {
        let result = terminated(separated_list1(space1, Self::parse_number), multispace0)(input);
        let (rest, row) = result.unwrap();

        assert_eq!(
            row.len(),
            5,
            "A Bingo Board must have exactly 5 numbers in a row!"
        );
        Ok((rest, row))
    }

    fn parse_number(input: &str) -> IResult<&str, u32> {
        map_res(many1(digit1), |x| {
            u32::from_str(x.into_iter().collect::<String>().as_str())
        })(input)
    }
}

#[derive(Clone)]
pub struct BingoState {
    pub numbers: Vec<u32>,
    pub boards: Vec<BingoBoard>,
}

impl BingoState {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, numbers) = Self::parse_numbers(input)?;
        let (rest, (boards, _)) = many_till(BingoBoard::parse, eof)(rest)?;

        Ok((rest, BingoState { numbers, boards }))
    }

    fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
        terminated(separated_list1(tag(","), Self::parse_number), multispace0)(input)
    }

    fn parse_number(input: &str) -> IResult<&str, u32> {
        map_res(many1(digit1), |x| {
            u32::from_str(x.into_iter().collect::<String>().as_str())
        })(input)
    }
}

#[derive(Clone, Debug, Eq)]
pub struct WinningState {
    pub called_numbers: Vec<u32>,
    pub board: BingoBoard,
}

impl WinningState {
    pub fn result(&self) -> Result<u32> {
        let unmarked_sum = self
            .board
            .rows
            .iter()
            .flatten()
            .filter(|x| self.called_numbers.contains(x))
            .sum::<u32>();

        Ok((self.board.sum - unmarked_sum) * self.called_numbers.iter().last().unwrap())
    }
}

impl PartialEq<Self> for WinningState {
    fn eq(&self, other: &Self) -> bool {
        self.board.eq(&other.board)
    }
}

pub fn part_one(state: BingoState) -> Result<Option<WinningState>> {
    let mut called_numbers: Vec<u32> = Vec::new();

    for num in state.numbers {
        called_numbers.push(num);
        let winning_board = state.boards.iter().find(|x| x.winner(&called_numbers));

        match winning_board {
            Some(board) => {
                return Ok(Some(WinningState {
                    called_numbers: called_numbers.clone(),
                    board: board.clone(),
                }))
            }
            None => continue,
        }
    }

    Ok(None)
}

pub fn part_two(state: BingoState) -> Result<Option<WinningState>> {
    let mut called_numbers: Vec<u32> = Vec::new();
    let mut remaining_boards = state.boards;
    let mut winners: Vec<WinningState> = Vec::new();

    for num in state.numbers {
        called_numbers.push(num);
        let (winning_boards, losing_boards): (Vec<BingoBoard>, Vec<BingoBoard>) = remaining_boards
            .into_iter()
            .partition(|x| x.winner(&called_numbers));

        for winner in winning_boards {
            let winner = WinningState {
                called_numbers: called_numbers.clone(),
                board: winner.clone(),
            };

            if !winners.contains(&winner) {
                winners.push(winner);
            }
        }

        remaining_boards = losing_boards;
    }

    if winners.is_empty() {
        return Ok(None);
    }

    println!("{:?}", winners);
    Ok(winners.last().cloned())
}
