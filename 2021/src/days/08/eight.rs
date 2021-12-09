use anyhow::Result;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, multispace0};
use nom::multi::many1;
use nom::sequence::{separated_pair, terminated};
use nom::IResult;
use std::collections::HashMap;

#[cfg(test)]
mod tests;

#[derive(Debug, Eq, PartialEq)]
pub struct FooState {
    signals: Vec<Digit>,
    display: Vec<Digit>,
}

impl FooState {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, (signals, display)) =
            separated_pair(many1(Digit::parse), tag("| "), many1(Digit::parse))(input)?;
        Ok((rest, FooState { signals, display }))
    }

    pub fn decode_signals(&self) -> Result<HashMap<char, i32>> {
        let frequencies = self
            .signals
            .iter()
            .map(|x| x.segments.iter())
            .flatten()
            .fold(HashMap::new(), |mut frequencies, segment| {
                *frequencies.entry(segment).or_insert(0) += 1;
                frequencies
            });

        let rule_set: HashMap<i32, RuleSetFn> = HashMap::from([
            (
                1,
                RuleSetFn::from(Box::new(
                    |(character, frequency): (&&char, &i32), signals: &Vec<Digit>| {
                        *frequency == 8
                            && !signals
                                .iter()
                                .filter(|x| x.segments.len() == 2)
                                .exactly_one()
                                .unwrap()
                                .segments
                                .contains(character)
                    },
                )),
            ),
            (
                2,
                RuleSetFn::from(Box::new(
                    |(_, frequency): (&&char, &i32), _: &Vec<Digit>| *frequency == 6,
                )),
            ),
            (
                3,
                RuleSetFn::from(Box::from(
                    |(character, frequency): (&&char, &i32), signals: &Vec<Digit>| {
                        *frequency == 8
                            && signals
                                .iter()
                                .filter(|x| x.segments.len() == 2)
                                .exactly_one()
                                .unwrap()
                                .segments
                                .contains(character)
                    },
                )),
            ),
            (
                4,
                RuleSetFn::from(Box::from(
                    |(character, frequency): (&&char, &i32), signals: &Vec<Digit>| {
                        *frequency == 7
                            && signals
                                .iter()
                                .filter(|x| x.segments.len() == 4)
                                .exactly_one()
                                .unwrap()
                                .segments
                                .contains(character)
                    },
                )),
            ),
            (
                5,
                RuleSetFn::from(Box::from(
                    |(_, frequency): (&&char, &i32), _: &Vec<Digit>| *frequency == 4,
                )),
            ),
            (
                6,
                RuleSetFn::from(Box::from(
                    |(_, frequency): (&&char, &i32), _: &Vec<Digit>| *frequency == 9,
                )),
            ),
            (
                7,
                RuleSetFn::from(Box::from(
                    |(character, frequency): (&&char, &i32), signals: &Vec<Digit>| {
                        *frequency == 7
                            && !signals
                                .iter()
                                .filter(|x| x.segments.len() == 4)
                                .exactly_one()
                                .unwrap()
                                .segments
                                .contains(character)
                    },
                )),
            ),
        ]);

        let x1: HashMap<char, i32> = rule_set
            .into_iter()
            .map(|(position, rule)| {
                let (character, _) = frequencies
                    .iter()
                    .filter(|x| rule(*x, &self.signals))
                    .exactly_one()
                    .unwrap();
                (*character.clone(), position.clone())
            })
            .collect();

        println!("{:?}", x1);
        Ok(x1)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Digit {
    segments: Vec<char>,
}

impl Digit {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (rest, result) = terminated(alpha1, multispace0)(input)?;
        Ok((
            rest,
            Self {
                segments: result.chars().collect::<Vec<char>>(),
            },
        ))
    }

    pub fn decode(&self, decoded_signals: HashMap<char, i32>) -> Result<char> {
        let nums: HashMap<Vec<i32>, char> = HashMap::from([
            (vec![1, 2, 3, 5, 6, 7], '0'),
            (vec![3, 6], '1'),
            (vec![1, 3, 4, 5, 7], '2'),
            (vec![1, 3, 4, 6, 7], '3'),
            (vec![2, 3, 4, 6], '4'),
            (vec![1, 2, 4, 6, 7], '5'),
            (vec![1, 2, 4, 5, 6, 7], '6'),
            (vec![1, 3, 6], '7'),
            (vec![1, 2, 3, 4, 5, 6, 7], '8'),
            (vec![1, 2, 3, 4, 6, 7], '9'),
        ]);

        let mut segments = self
            .segments
            .iter()
            .map(|x| decoded_signals.get(x).unwrap().clone())
            .collect::<Vec<i32>>();
        segments.sort();
        Ok(nums.get(&segments).unwrap().clone())
    }

    pub fn has_unique_segments(&self) -> bool {
        match self.segments.len() {
            2 => true,
            3 => true,
            4 => true,
            7 => true,
            _ => false,
        }
    }
}

pub fn part_one(input: Vec<FooState>) -> Result<i32> {
    let count = input.iter().fold(0, |sum, state| {
        sum + state
            .display
            .iter()
            .filter(|x| x.has_unique_segments())
            .count() as i32
    });

    Ok(count)
}

pub fn part_two(input: Vec<FooState>) -> Result<i32> {
    let sum = input.iter().fold(0, |sum, state| {
        let decoded_signals = state.decode_signals().unwrap();
        let display_value = state
            .display
            .iter()
            .map(|digit| digit.decode(decoded_signals.clone()).unwrap())
            .join("")
            .parse::<i32>()
            .unwrap();
        println!("{}", display_value);
        sum + display_value
    });

    Ok(sum)
}

pub type RuleSetFn = Box<dyn Fn((&&char, &i32), &Vec<Digit>) -> bool>;
