use anyhow::Result;

#[cfg(test)]
mod tests;

pub fn part_one(calories: Vec<Vec<u32>>) -> Result<u32> {
    Ok(calories.iter().map(|x| x.iter().sum()).max().unwrap())
}

pub fn part_two(calories: Vec<Vec<u32>>) -> Result<u32> {
    let mut elves = calories
        .iter()
        .map(|x| x.iter().sum())
        .collect::<Vec<u32>>();
    elves.sort();

    Ok(elves.iter().rev().take(3).sum())
}
