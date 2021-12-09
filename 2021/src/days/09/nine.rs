use anyhow::Result;
use std::collections::HashSet;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OceanHeight {
    pub coords: (usize, usize),
    pub value: u32,
}

pub fn part_one(input: &Vec<Vec<u32>>) -> Result<Vec<OceanHeight>> {
    let mut low_points: Vec<OceanHeight> = Vec::new();

    for (i, line) in input.iter().enumerate() {
        for (j, current_height) in line.iter().enumerate() {
            let adjacent_coords = find_adjacent(i, j, input.len(), input.get(i).unwrap().len())?;
            let adjacent_heights = adjacent_coords
                .iter()
                .map(|(x, y)| input.get(*x).unwrap().get(*y).unwrap().clone())
                .collect::<Vec<u32>>();

            let lower_adjacent_count = adjacent_heights
                .iter()
                .filter(|x| **x > *current_height)
                .count();

            if lower_adjacent_count == adjacent_heights.len() {
                low_points.push(OceanHeight {
                    coords: (i, j),
                    value: current_height.clone(),
                })
            }
        }
    }

    Ok(low_points)
}

pub fn part_two(input: &Vec<Vec<u32>>) -> Result<Vec<i32>> {
    let low_points = part_one(&input)?;
    let basin_sizes = low_points
        .iter()
        .map(|point| {
            let (x, y) = point.coords;
            let mut visited: HashSet<(usize, usize)> = HashSet::new();
            let mut basin_search = vec![(x, y)];
            let mut basin_size = 0;
            while !basin_search.is_empty() {
                let coords = basin_search.pop().unwrap();

                if visited.contains(&coords) {
                    continue;
                };
                visited.insert(coords);
                basin_size += 1;
                let current_height = input.get(coords.0).unwrap().get(coords.1).unwrap();
                let lower_adjacent_coords =
                    find_adjacent(coords.0, coords.1, input.len(), input.get(x).unwrap().len())
                        .unwrap()
                        .into_iter()
                        .filter(|(i, j)| {
                            let adjacent_height = *input.get(*i).unwrap().get(*j).unwrap();
                            adjacent_height > *current_height && adjacent_height != 9
                        })
                        .collect::<Vec<(usize, usize)>>();

                basin_search.extend(lower_adjacent_coords)
            }
            basin_size
        })
        .collect::<Vec<i32>>();
    Ok(basin_sizes)
}

pub fn find_adjacent(
    i: usize,
    j: usize,
    max_i: usize,
    max_j: usize,
) -> Result<Vec<(usize, usize)>> {
    let lower_i = if i != 0 { Some(i - 1) } else { None };
    let upper_i = if i + 1 < max_i { Some(i + 1) } else { None };
    let lower_j = if j != 0 { Some(j - 1) } else { None };
    let upper_j = if j + 1 < max_j { Some(j + 1) } else { None };

    let result = [
        (lower_i, Some(j)),
        (upper_i, Some(j)),
        (Some(i), lower_j),
        (Some(i), upper_j),
    ]
    .iter()
    .filter(|(x, y)| x.is_some() && y.is_some())
    .map(|(x, y)| (x.unwrap(), y.unwrap()))
    .collect::<Vec<(usize, usize)>>();

    Ok(result)
}
