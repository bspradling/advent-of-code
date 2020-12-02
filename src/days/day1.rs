use anyhow::Result;
use std::fs::File;
use std::io::{BufReader, BufRead};
use log::{debug, info};

pub async fn solve() -> Result<()> {
    let file = File::open("resources/day1.txt")?;
    let reader = BufReader::new(file);

    let numbers: Vec<i32> = reader.lines()
        .map(|line| {
            let string = line.expect("Could not parse line!");
            string.parse::<i32>().expect("Could not convert string to i32!")
        })
        .collect();

    info!("Part 1 Answer is: {}", part_one(2020, numbers.clone())?);
    info!("Part 2 Answer is: {}", part_two(2020, numbers.clone())?);

    Ok(())
}

fn part_one(target: i32, numbers: Vec<i32>) -> Result<i32> {
    for number in &numbers {
        let search_for = target - number;
        if numbers.contains(&search_for) {
            debug!("{} and {} sum to {}", number, search_for, target);
            return Ok(search_for * number);
        }
    }

    Err(anyhow::anyhow!("Could not find two entries that sum to {}", target))
}

fn part_two(target: i32, numbers: Vec<i32>) -> Result<i32> {

    for i in 0..numbers.len() {
        let value = numbers[i];

        let result = part_one(target - value, numbers[(i+1)..].to_vec());

        match result {
            Ok(answer) => {
                debug!("Found two sum solution {}", answer);
                return Ok(answer * value)
            },
            Err(_error) => {
                debug!("Couldn't find three sum for index {}", i);
            }
        }
    }

    Err(anyhow::anyhow!("Could not find three entries that sum to {}", target))
}