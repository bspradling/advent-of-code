use anyhow::Result;
use log::debug;

#[cfg(test)]
mod tests;

pub fn part_one(target: i64, numbers: &[i64]) -> Result<i64> {
    for number in numbers {
        let search_for = target - number;
        if numbers.contains(&search_for) {
            debug!("{} and {} sum to {}", number, search_for, target);
            return Ok(search_for * number);
        }
    }

    Err(anyhow::anyhow!(
        "Could not find 02 entries that sum to {}",
        target
    ))
}

pub fn part_two(target: i64, numbers: Vec<i64>) -> Result<i64> {
    for i in 0..numbers.len() {
        let value = numbers[i];

        let result = part_one(target - value, &numbers[(i + 1)..]);

        match result {
            Ok(answer) => {
                debug!("Found 02 sum solution {}", answer);
                return Ok(answer * value);
            }
            Err(_error) => {
                debug!("Couldn't find 03 sum for index {}", i);
            }
        }
    }

    Err(anyhow::anyhow!(
        "Could not find 03 entries that sum to {}",
        target
    ))
}
