use anyhow::{anyhow, Error, Result};
use std::str::FromStr;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Position {
    aim: i32,
    depth: i32,
    horizontal: i32,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    DOWN(i32),
    UP(i32),
    FORWARD(i32),
}

impl Command {
    pub fn execute_v1(&self, current_position: (i32, i32)) -> Result<(i32, i32)> {
        match self {
            Command::DOWN(value) => Ok((current_position.0, current_position.1 + value)),
            Command::UP(value) => Ok((current_position.0, current_position.1 - value)),
            Command::FORWARD(value) => Ok((current_position.0 + value, current_position.1)),
        }
    }

    pub fn execute_v2(&self, position: Position) -> Result<Position> {
        match self {
            Command::DOWN(value) => Ok(Position {
                aim: position.aim + value,
                depth: position.depth,
                horizontal: position.horizontal,
            }),
            Command::UP(value) => Ok(Position {
                aim: position.aim - value,
                depth: position.depth,
                horizontal: position.horizontal,
            }),
            Command::FORWARD(value) => Ok(Position {
                aim: position.aim,
                depth: position.depth + (position.aim * value),
                horizontal: position.horizontal + value,
            }),
        }
    }
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::rust_2015::Result<Self, Self::Err> {
        let (command, value) = s.split_at(
            s.find(' ')
                .ok_or(anyhow!("The input file is in an invalid form!"))?,
        );

        let digit = value.trim().parse::<i32>()?;

        match command.trim().to_ascii_lowercase().as_str() {
            "down" => Ok(Command::DOWN(digit)),
            "up" => Ok(Command::UP(digit)),
            "forward" => Ok(Command::FORWARD(digit)),
            _ => Err(anyhow!("Unable to parse command")),
        }
    }
}

pub fn part_one(commands: &[Command]) -> Result<(i32, i32)> {
    Ok(commands
        .into_iter()
        .fold((0, 0), |x, command| command.execute_v1(x).unwrap()))
}

pub fn part_two(commands: &[Command]) -> Result<Position> {
    let initial_position = Position {
        aim: 0,
        depth: 0,
        horizontal: 0,
    };

    Ok(commands.into_iter().fold(initial_position, |x, command| {
        command.execute_v2(x).unwrap()
    }))
}
