use anyhow::Result;
use itertools::Itertools;
use log::info;
use regex::Regex;
use std::collections::HashMap;

pub async fn solve() -> Result<()> {
    let identity_cards: Vec<HashMap<&str, &str>> = include_str!("resources/day4.txt")
        .split("\n\n") //split on blank lines
        .map(|entry| {
            entry
                .split_whitespace()
                .flat_map(|field| field.split(":"))
                .tuples()
                .collect::<HashMap<&str, &str>>()
        })
        .collect();

    info!("Part 01 Answer is {}", part_one(identity_cards.clone())?);
    info!("Part 02 Answer is {}", part_two(identity_cards.clone())?);

    Ok(())
}

fn part_one(identity_cards: Vec<HashMap<&str, &str>>) -> Result<usize> {
    Ok(identity_cards
        .into_iter()
        .filter(validate_required_fields)
        .count())
}

fn part_two(identity_cards: Vec<HashMap<&str, &str>>) -> Result<usize> {
    Ok(identity_cards
        .into_iter()
        .filter(validate_required_fields)
        .filter(validate_field_values)
        .count())
}

fn validate_required_fields(map: &HashMap<&str, &str>) -> bool {
    vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|key| map.contains_key(key))
}

fn validate_field_values(fields: &HashMap<&str, &str>) -> bool {
    return fields
        .iter()
        .all(|(key, value)| validate_field(key, value).unwrap_or(false));
}

fn validate_field(key: &str, value: &str) -> Result<bool> {
    return match key {
        "byr" => {
            let year = value.parse::<i32>()?;
            Ok(value.len() == 4 && year >= 1920 && year <= 2002)
        }
        "iyr" => {
            let year = value.parse::<i32>()?;
            Ok(value.len() == 4 && year >= 2010 && year <= 2020)
        }
        "eyr" => {
            let year = value.parse::<i32>()?;
            Ok(value.len() == 4 && year >= 2020 && year <= 2030)
        }
        "hgt" => match value.split_at(value.len() - 2) {
            (height, "cm") => {
                let parse = height.parse::<i32>()?;
                Ok(parse >= 150 && parse <= 193)
            }
            (height, "in") => {
                let parse = height.parse::<i32>()?;
                Ok(parse >= 59 && parse <= 76)
            }
            (_, _) => Ok(false),
        },
        "hcl" => {
            let regex = Regex::new("^#[0-9a-f]{06}$")?;
            Ok(regex.is_match(value))
        }
        "ecl" => {
            let regex = Regex::new("amb|blu|brn|gry|grn|hzl|oth")?;
            Ok(regex.is_match(value))
        }
        "pid" => Ok(value.len() == 9 && value.chars().all(|e| e.is_numeric())),
        "cid" => Ok(true),
        _ => Ok(false),
    };
}
