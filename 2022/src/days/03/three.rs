use anyhow::Result;
use itertools::Itertools;
use std::collections::HashSet;

#[cfg(test)]
mod tests;

pub fn part_one(compartments: Vec<(&str, &str)>) -> Result<u32> {
    Ok(compartments
        .into_iter()
        .map(|(first, second)| {
            let first_characters: HashSet<char> = first.chars().collect();
            second.chars().find(|c| first_characters.contains(&c))
        })
        .fold(0, |sum, x: Option<char>| match x {
            Some(y) => match y as u32 {
                y if y >= 65 && y <= 90 => sum + (y - 38),
                y if y >= 97 && y <= 122 => sum + (y - 96),
                _ => panic!("An invalid character: {}", y),
            },
            None => panic!("ahh"),
        }))
}

pub fn part_two(rucksacks: Vec<&str>) -> Result<u32> {
    Ok(rucksacks
        .chunks(3)
        .into_iter()
        .map(|group| {
            group
                .iter()
                .map(|x| x.chars().collect())
                .fold(group[0].chars().collect::<HashSet<char>>(), |acc, cur| {
                    acc.intersection(&cur).copied().collect()
                })
                .into_iter()
                .exactly_one()
                .expect("there should only be one")
        })
        .fold(0, |sum, x: char| match x as u32 {
            y if y >= 65 && y <= 90 => sum + (y - 38),
            y if y >= 97 && y <= 122 => sum + (y - 96),
            _ => panic!("An invalid character: {}", x),
        }))
}
