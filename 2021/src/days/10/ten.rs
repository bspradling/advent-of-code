use anyhow::{anyhow, Result};
use std::collections::HashMap;

#[cfg(test)]
mod tests;

pub trait Character {}

#[derive(Debug)]
pub enum Opening {
    Brace,
    Bracket,
    Carrot,
    Parenthesis,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Closing {
    Brace,
    Bracket,
    Carrot,
    Parenthesis,
}

impl Character for Opening {}
impl Character for Closing {}

impl Opening {
    pub fn of(character: char) -> Result<Self> {
        match character {
            '{' => Ok(Self::Brace),
            '[' => Ok(Self::Bracket),
            '<' => Ok(Self::Carrot),
            '(' => Ok(Self::Parenthesis),
            _ => Err(anyhow!("Does not map to an opening character!")),
        }
    }

    pub fn contains(character: char) -> bool {
        ['{', '[', '<', '('].contains(&character)
    }

    pub fn value(&self) -> char {
        match self {
            Self::Brace => '{',
            Self::Bracket => '[',
            Self::Carrot => '<',
            Self::Parenthesis => '(',
        }
    }

    pub fn closed_by(&self) -> Closing {
        match self {
            Opening::Brace => Closing::Brace,
            Opening::Bracket => Closing::Bracket,
            Opening::Carrot => Closing::Carrot,
            Opening::Parenthesis => Closing::Parenthesis,
        }
    }
}

impl Closing {
    pub fn of(character: char) -> Result<Self> {
        match character {
            '}' => Ok(Self::Brace),
            ']' => Ok(Self::Bracket),
            '>' => Ok(Self::Carrot),
            ')' => Ok(Self::Parenthesis),
            _ => Err(anyhow!("Does not map to an closing character!")),
        }
    }
    pub fn contains(character: char) -> bool {
        ['}', ']', '>', ')'].contains(&character)
    }

    pub fn value(&self) -> char {
        match self {
            Self::Brace => '}',
            Self::Bracket => ']',
            Self::Carrot => '>',
            Self::Parenthesis => ')',
        }
    }

    pub fn opened_by(&self) -> Opening {
        match self {
            Closing::Brace => Opening::Brace,
            Closing::Bracket => Opening::Bracket,
            Closing::Carrot => Opening::Carrot,
            Closing::Parenthesis => Opening::Parenthesis,
        }
    }
}

pub fn part_one(input: Vec<Vec<char>>) -> Result<i32> {
    let scoring: HashMap<Closing, i32> = HashMap::from([
        (Closing::of(')')?, 3),
        (Closing::of(']')?, 57),
        (Closing::of('}')?, 1197),
        (Closing::of('>')?, 25137),
    ]);

    let mut score = 0;

    for line in input {
        let mut stack: Vec<Opening> = vec![];

        for character in line {
            if Closing::contains(character) {
                let x = stack.pop().unwrap(); //TODO handle case

                let expected = x.closed_by().value();
                if expected != character {
                    println!(
                        "Was supposed to be closed by {} but instead was {}",
                        expected, character
                    );

                    score += scoring.get(&Closing::of(character)?).unwrap();
                }
            } else if Opening::contains(character) {
                stack.push(Opening::of(character)?)
            }
        }
        if !stack.is_empty() {
            //TODO: incomplete
            println!("Line is incomplete!")
        }
    }

    Ok(score)
}

pub fn part_two(input: Vec<Vec<char>>) -> Result<i64> {
    let scoring: HashMap<Closing, i64> = HashMap::from([
        (Closing::of(')')?, 1),
        (Closing::of(']')?, 2),
        (Closing::of('}')?, 3),
        (Closing::of('>')?, 4),
    ]);

    let mut scores: Vec<i64> = vec![];

    for line in input {
        let mut score: i64 = 0;
        let mut stack: Vec<Opening> = vec![];
        let mut corrupted: bool = false; //Gross

        for character in line {
            if Closing::contains(character) {
                let x = stack.pop().unwrap(); //TODO handle case

                let expected = x.closed_by().value();
                if expected != character {
                    corrupted = true
                }
            } else if Opening::contains(character) {
                stack.push(Opening::of(character)?)
            }
        }
        if !stack.is_empty() && !corrupted {
            while !stack.is_empty() {
                let closing_character = stack.pop().unwrap().closed_by();

                score = (score * 5) + scoring.get(&closing_character).unwrap();
            }
            scores.push(score);
        }
    }

    scores.sort();
    println!("{:?}", scores);
    Ok(scores.get(scores.len() / 2).unwrap().clone())
}
