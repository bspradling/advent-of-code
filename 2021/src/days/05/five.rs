use anyhow::Result;
use nom::IResult;
use nom::multi::{many1, many_till};
use nom::combinator::{eof, map_res};
use nom::sequence::{terminated, separated_pair};
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use std::str::FromStr;
use std::collections::HashMap;
use std::cmp::{max, min};

#[cfg(test)]
mod tests;

#[derive(Clone, Debug)]
pub struct OceanFloor {
    pub vents: Vec<HydrothermalVent>,
}

#[derive(Clone, Debug)]
pub struct HydrothermalVent {
    pub start: (u32, u32),
    pub end: (u32, u32),
}

impl HydrothermalVent {
    pub fn is_non_diagonal(&self) -> bool {
        self.start.0 == self.end.0 || self.start.1 == self.end.1
    }
}

impl OceanFloor {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (_, (vents,_)) = many_till(Self::parse_line, eof)(input)?;

        Ok(("", Self { vents }))
    }

    fn parse_line(input: &str) -> IResult<&str, HydrothermalVent> {
        let (rest, (xy_1, xy_2)) =
            terminated(separated_pair(
                Self::parse_coordinates,
                tag(" -> "),
                Self::parse_coordinates
            ), newline)(input)?;

        Ok((rest, HydrothermalVent { start: xy_1, end: xy_2}))
    }

    fn parse_coordinates(input: &str) -> IResult<&str, (u32, u32)> {
        separated_pair(Self::parse_number, tag(","), Self::parse_number)(input)
    }

    fn parse_number(input: &str) -> IResult<&str, u32> {
        map_res(many1(digit1), |x| {
            u32::from_str(x.into_iter().collect::<String>().as_str())
        })(input)
    }

    pub fn dangerous_locations(&self) -> Result<usize> {
        let count = self.vents.iter()
            .map(Self::explode_points_to_line)
            .flatten()
            .fold(HashMap::<(u32, u32), u32>::new(), |mut map, element| {
                *map.entry(element).or_default() += 1;
                map
            }).into_iter()
            .filter(|(_,value)| *value >= 2)
            .count();

        Ok(count)
    }

    pub fn explode_points_to_line(vent: &HydrothermalVent) -> Vec<(u32, u32)> {
        let min_x = min(vent.start.0, vent.end.0);
        let max_x = max(vent.start.0, vent.end.0);
        let min_y = min(vent.start.1, vent.end.1);
        let max_y = max(vent.start.1, vent.end.1);

        return if vent.start.0 != vent.end.0 && vent.start.1 != vent.end.1 {
            let xs = if vent.start.0 > vent.end.0 {
                (min_x..(max_x+1)).rev().collect::<Vec<u32>>()
            } else {
                (min_x..(max_x+1)).collect::<Vec<u32>>()
            };
            let ys = if vent.start.1 > vent.end.1 {
                (min_y..(max_y+1)).rev().collect::<Vec<u32>>()
            } else {
                (min_y..(max_y+1)).collect::<Vec<u32>>()
            };
            (xs.into_iter()).zip(ys.into_iter()).collect::<Vec<(u32, u32)>>()
        } else if vent.start.0 == vent.end.0 {
            (min_y..(max_y + 1)).map(|y| (vent.start.0, y)).collect::<Vec<(u32, u32)>>()
        } else {
            (min_x..(max_x + 1)).map(|x| (x, vent.start.1)).collect::<Vec<(u32, u32)>>()
        }
    }
}

pub fn part_one(floor: OceanFloor) -> Result<usize> {
    OceanFloor {
        vents: floor.vents.into_iter().filter(HydrothermalVent::is_non_diagonal).collect()
    }.dangerous_locations()
}

pub fn part_two(floor: OceanFloor) -> Result<usize> {
    floor.dangerous_locations()
}
