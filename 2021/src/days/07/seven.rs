use anyhow::Result;
use std::cmp::min;

#[cfg(test)]
mod tests;

pub fn part_one(crab_locations: Vec<i32>) -> Result<i32> {
    let max = crab_locations.iter().max().unwrap();

    Ok((0..*max).fold(i32::max_value(), |minimum_fuel, index| {
        let fuel_needed = crab_locations.iter().fold(0, |fuel_needed, current| {
            fuel_needed + (current - index).abs()
        });
        min(minimum_fuel, fuel_needed)
    }))
}

pub fn part_two(crab_locations: Vec<i32>) -> Result<i32> {
    let max = crab_locations.iter().max().unwrap();

    Ok((0..*max).fold(i32::max_value(), |minimum_fuel, index| {
        let fuel_needed = crab_locations.iter().fold(0, |fuel_needed, current| {
            // fuel_needed + (0..((current - index).abs() + 1)).sum::<i32>()
            let distance = (current - index).abs();
            fuel_needed + (distance * (distance + 1)) / 2
        });
        min(minimum_fuel, fuel_needed)
    }))
}
