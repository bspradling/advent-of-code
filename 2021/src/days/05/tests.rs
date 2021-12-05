use crate::days::five::{part_one, OceanFloor, HydrothermalVent, part_two};
use anyhow::Result;

#[test]
pub fn part_one_solution() -> Result<()> {
    let input = include_str!("resources/input.txt");
    let (remaining, floor) = OceanFloor::parse(input)?;
    let solution = part_one(floor)?;

    assert_eq!(remaining, "");
    assert_eq!(solution, 8350);

    Ok(())
}

#[test]
pub fn part_two_solution() -> Result<()> {
    let input = include_str!("resources/input.txt");
    let (remaining, floor) = OceanFloor::parse(input)?;
    let solution = part_two(floor)?;

    assert_eq!(remaining, "");
    assert_eq!(solution, 19374);

    Ok(())
}

#[test]
pub fn part_one_test_case() -> Result<()> {
    let input = include_str!("resources/test_case.txt");

    let (remaining, floor) = OceanFloor::parse(input)?;
    let actual = part_one(floor)?;

    assert_eq!(remaining, "");
    assert_eq!(actual, 5);

    Ok(())
}

#[test]
pub fn part_two_test_case() -> Result<()> {
    let input = include_str!("resources/test_case.txt");

    let (remaining, floor) = OceanFloor::parse(input)?;
    let actual = part_two(floor.clone())?;

    assert_eq!(remaining, "");
    assert_eq!(actual, 12);

    Ok(())
}

#[test]
pub fn test_coordinate_range() -> Result<()> {
    assert_eq!((1..4).into_iter().map(|x| (1,x)).collect::<Vec<(u32,u32)>>(), vec!((1,1), (1,2), (1,3)));
    Ok(())
}

#[test]
pub fn test_foo() -> Result<()> {
    let points = OceanFloor::explode_points_to_line(&HydrothermalVent { start: (8, 0), end: (0, 8) });
    assert_eq!(points, vec![(8,0), (7,1), (6,2), (5,3), (4,4), (3,5), (2,6), (1,7), (0,8)]);
    Ok(())
}