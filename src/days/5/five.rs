use anyhow::Result;
use log::{debug, info};
use std::collections::HashSet;
use std::ops::Range;

mod tests;

pub struct BoardingTicket {
    row: &'static str,
    column: &'static str,
}

impl From<&'static str> for BoardingTicket {
    fn from(seating_assignment: &'static str) -> Self {
        let tuple = seating_assignment.split_at(7);
        return Self {
            row: tuple.0,
            column: tuple.1,
        };
    }
}

impl BoardingTicket {
    pub fn row(&self) -> &str {
        &self.row
    }

    pub fn column(&self) -> &str {
        &self.column
    }

    pub fn seat_id(&self) -> i32 {
        let row = self.binary_process(self.row, 0..127);
        let column = self.binary_process(self.column, 0..7);

        row * 8 + column
    }

    fn binary_process(&self, indicator: &str, mut range: Range<i32>) -> i32 {
        for char in indicator.chars() {
            debug!("Character {}, Range {:?}", char, range);
            let half = (&range.start + &range.end) / 2;
            match char {
                'F' | 'L' => range.end = half,
                'B' | 'R' => range.start = half + 1,
                _ => panic!(
                    "Invalid ticket! Row: {}, Column: {}",
                    self.row(),
                    self.column()
                ),
            }
        }

        return match indicator.chars().last() {
            Some('F') | Some('L') => range.start,
            Some('B') | Some('R') => range.end,
            _ => panic!("Invalid ticket!"),
        };
    }
}

pub async fn solve() -> Result<()> {
    let tickets = include_str!("resources/day5.txt")
        .lines()
        .map(BoardingTicket::from)
        .collect::<Vec<BoardingTicket>>();

    info!("Part 1 Answer is: {}", part_one(&tickets).unwrap());
    info!("Part 2 Answer is: {}", part_two(&tickets));

    Ok(())
}

fn part_one(tickets: &Vec<BoardingTicket>) -> Option<i32> {
    tickets.iter().map(|ticket| ticket.seat_id()).max()
}

fn part_two(tickets: &Vec<BoardingTicket>) -> i32 {
    let set = tickets
        .iter()
        .map(|ticket| ticket.seat_id())
        .collect::<HashSet<i32>>();

    let list = set
        .iter()
        .filter(|e| set.contains(&(*e + 2)) && !set.contains(&(*e + 1)))
        .collect::<Vec<&i32>>();

    *list.first().unwrap() + 1
}
