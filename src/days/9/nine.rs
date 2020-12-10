use anyhow::{anyhow, Result};
use log::{debug, info};

use crate::days::one::part_one as two_sum;

pub async fn solve() -> Result<()> {
    let numbers = include_str!("resources/day9.txt")
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let part_one = part_one(&numbers).unwrap();
    info!("Part 1 Answer is {}", part_one);
    info!("Part 1 Answer is {}", part_two(&numbers, part_one).unwrap());
    Ok(())
}

fn part_one(numbers: &Vec<i64>) -> Result<i64> {
    let preamble_size = 25;

    for (index, number) in numbers[preamble_size..].iter().enumerate() {
        let preamble = &numbers[index..index + preamble_size];
        return match two_sum(number.clone(), preamble) {
            Ok(_) => continue,
            Err(_) => Ok(number.clone()),
        };
    }

    Err(anyhow!(
        "Did not find an invalid value in the XMAS protocol!"
    ))
}

fn part_two(numbers: &Vec<i64>, invalid_result: i64) -> Result<i64> {
    for (index, _) in numbers.iter().enumerate() {
        let mut boundary = index + 1;
        loop {
            let current = &numbers[index..boundary];
            let sum: i64 = numbers[index..boundary].iter().sum();

            match sum {
                x if x == invalid_result => {
                    debug!("Contiguous set is {:?}", current);
                    return Ok(current.iter().min().unwrap() + current.iter().max().unwrap());
                }
                x if x > invalid_result => break,
                _ => boundary += 1,
            }
        }
    }

    Err(anyhow!("Couldn't find a weakness in the XMAS protocol!"))
}
