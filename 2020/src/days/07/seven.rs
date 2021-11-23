use anyhow::Result;
use log::{debug, info};
use pathfinding::prelude::topological_sort;
use scan_rules::scanner::{Everything, Number};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct BagQuantity((usize, String));

impl BagQuantity {
    pub fn count(&self) -> usize {
        self.0 .0
    }

    pub fn color(&self) -> &str {
        &self.0 .1
    }
}

impl From<(usize, String)> for BagQuantity {
    fn from(tuple: (usize, String)) -> Self {
        Self { 0: tuple }
    }
}
impl FromStr for BagQuantity {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        return match string {
            " no other bags." => Err(()),
            _ => {
                let_scan!(string; (let count: Number, let color: Everything));

                let x: Vec<&str> = color.split_whitespace().collect();
                Ok(BagQuantity {
                    0: (
                        count.parse().unwrap(),
                        format!("{} {}", x.get(0).unwrap_or(&""), x.get(1).unwrap_or(&"")),
                    ),
                })
            }
        };
    }
}

#[derive(Debug, Clone)]
struct BagRule {
    color: String,
    contains: Vec<BagQuantity>,
}

impl BagRule {
    pub fn color(&self) -> &str {
        &self.color
    }

    pub fn contains(&self) -> Vec<(usize, &str)> {
        self.contains
            .iter()
            .map(|x| (x.count(), x.color()))
            .collect()
    }
}

impl FromStr for BagRule {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = string.split(" bags contain").collect();
        Ok(BagRule {
            color: split[0].to_string(),
            contains: split[1]
                .split(",")
                .map(|e| BagQuantity::from_str(e))
                .filter_map(Result::ok)
                .collect(),
        })
    }
}
pub async fn solve() -> Result<()> {
    let bag_rules = include_str!("resources/day7.txt")
        .lines()
        .map(|e| BagRule::from_str(e).unwrap())
        .collect::<Vec<BagRule>>();

    info!("Part 01 Answer is: {:?}", part_one(&bag_rules).unwrap());
    info!("Part 02 Answer is: {:?}", part_two(&bag_rules).unwrap());

    Ok(())
}

fn part_one(bag_rules: &Vec<BagRule>) -> Result<usize> {
    let mut map = HashMap::new();
    for rule in bag_rules {
        for (_, color) in rule.contains() {
            map.entry(color)
                .or_insert_with(|| HashSet::new())
                .insert(rule.color.as_str());
        }
    }

    debug!("Map Size: {}", map.len());

    Ok(topological_sort(&["shiny gold"], |e| {
        map.get(e).cloned().unwrap_or_else(HashSet::new)
    })
    .unwrap()
    .len()
        - 1)
}

fn part_two(bag_rules: &Vec<BagRule>) -> Result<usize> {
    let bag_map: HashMap<&str, Vec<(usize, &str)>> = bag_rules
        .iter()
        .map(|x| (x.color().clone(), x.contains().clone()))
        .collect();

    let vec = topological_sort(&["shiny gold"], |e| {
        bag_map[e].iter().map(|(_, test)| *test)
    })
    .unwrap_or(Vec::new());

    let mut capacity_map: HashMap<&str, usize> = HashMap::new();
    for (c, v) in vec.into_iter().rev().map(|c| (c, &bag_map[c])) {
        let capacity = v
            .iter()
            .map(|&(count, color)| count * (1 + capacity_map[color]))
            .sum();

        debug!("Setting {} capacity to {}", c, capacity);
        capacity_map.insert(c, capacity);
    }

    Ok(capacity_map["shiny gold"])
}
