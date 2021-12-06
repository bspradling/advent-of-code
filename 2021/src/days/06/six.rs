use anyhow::Result;

#[cfg(test)]
mod tests;

pub fn part_one(mut laternfishes: Vec<u64>, days: u64) -> Result<Vec<u64>> {

    for _ in 0..days {
        let mut baby_fishes: Vec<u64> = vec![];
        let processed = laternfishes.iter().map(|x| {
            if *x == 0 {
                baby_fishes.push(8);
                6
            }
            else {
                x - 1
            }
        }).collect::<Vec<u64>>();
        laternfishes = processed.into_iter().chain(baby_fishes.into_iter()).collect::<Vec<u64>>();
    }

    Ok(laternfishes)
}

pub fn part_two(laternfishes: Vec<u64>, days: u64) -> Result<Vec<u64>> {
    let mut counts_by_age = vec![0,0,0,0,0,0,0,0,0];
    laternfishes.into_iter().for_each(|x| counts_by_age[x as usize] += 1);

    for _ in 0..days {
        let new_fishes = counts_by_age[0];

        for x in 1..9 {
            counts_by_age[x - 1] = counts_by_age[x];
        }
        counts_by_age[6] = counts_by_age[6] + new_fishes;
        counts_by_age[8] = new_fishes;
        println!("{:?}", counts_by_age);
    }

    Ok(counts_by_age.into_iter().collect())
}
