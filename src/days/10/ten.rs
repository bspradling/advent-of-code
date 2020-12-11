use anyhow::Result;
use itertools::Itertools;
use log::{debug, info};

mod tests;

pub async fn solve() -> Result<()> {
    let adapters = include_str!("resources/day10.rs")
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    info!("Part 1 Answer is: {}", part_one(adapters.clone()).unwrap());
    info!("Part 2 Answer is: {}", part_two(adapters.clone()).unwrap());

    Ok(())
}

fn part_one(mut adapters: Vec<i32>) -> Result<usize> {
    adapters.push(0);
    adapters.sort();

    println!("sorted {:?}", adapters);
    println!(
        "differences {:?}",
        adapters
            .iter()
            .tuple_windows()
            .into_iter()
            .map(|(a, b)| b - a)
            .collect::<Vec<i32>>()
    );

    let (one_jolts, three_jolts): (Vec<i32>, Vec<i32>) = adapters
        .iter()
        .tuple_windows()
        .into_iter()
        .map(|(a, b)| b - a)
        .partition(|&n| n < 2);

    let one_jolts = one_jolts.len();
    let three_jolts = three_jolts.len() + 1;
    debug!("{}", one_jolts);
    debug!("{}", three_jolts);

    Ok(one_jolts * three_jolts)
}

fn part_two(mut adapters: Vec<i32>) -> Result<i64> {
    let tribonacci: Vec<i64> = vec![0, 1, 2, 4, 7, 13, 24, 44, 81];
    adapters.push(0);
    adapters.sort();

    let jolt_deltas: Vec<i32> = adapters
        .iter()
        .tuple_windows()
        .into_iter()
        .map(|(a, b)| b - a)
        .collect();

    let mut permutations: i64 = 1;

    println!("{:?}", jolt_deltas);

    for (valid, group) in &jolt_deltas.into_iter().group_by(|i| i < &3) {
        if valid {
            permutations *= tribonacci[group.count()];
            println!("{}", permutations);
        }
    }

    Ok(permutations)
}
