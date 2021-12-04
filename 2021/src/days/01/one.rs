use anyhow::Result;

mod tests;

pub fn part_one(depths: &[u32]) -> Result<usize> {
    Ok(depths
        .iter()
        .zip(depths.iter().skip(1))
        .filter(|(x, y)| **x < **y)
        .count())
}

pub fn part_two(depths: &[u32]) -> Result<usize> {
    part_one(
        &depths
            .windows(3)
            .map(|x| x.iter().sum())
            .collect::<Vec<u32>>(),
    )
}
