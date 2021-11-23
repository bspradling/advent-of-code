#[cfg(test)]
use crate::days::ten::{part_one, part_two};

#[test]
fn test_part_one() {
    let jolts: Vec<i32> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

    assert_eq!(part_one(jolts).unwrap(), 35);
}

#[test]
fn test_part_one2() {
    let jolts: Vec<i32> = vec![
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];
    assert_eq!(part_one(jolts).unwrap(), 220);
}

#[test]
fn test_part_two() {
    let jolts: Vec<i32> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    assert_eq!(part_two(jolts).unwrap(), 8);
}

#[test]
fn test_part_two2() {
    let jolts: Vec<i32> = vec![
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];
    assert_eq!(part_two(jolts).unwrap(), 19208);
}
