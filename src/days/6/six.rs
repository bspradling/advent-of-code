use anyhow::Result;
use log::info;
use std::collections::HashSet;

pub async fn solve() -> Result<()> {
    let part_one: usize = include_str!("resources/day6.txt")
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .flat_map(|person| person.chars())
                .collect::<HashSet<char>>()
                .len()
        })
        .sum();

    let part_two: usize = include_str!("resources/day6.txt")
        .split("\n\n")
        .map(|group| {
            let mut group = group
                .lines()
                .map(|person| person.chars().collect::<HashSet<char>>());

            group
                .next()
                .map(|set| {
                    group
                        .fold(set, |set1, set2| {
                            set1.intersection(&set2).copied().collect()
                        })
                        .len()
                })
                .unwrap()
        })
        .sum();

    info!("Part 1 Answer is: {}", part_one);
    info!("Part 2 Answer is: {}", part_two);

    Ok(())
}
