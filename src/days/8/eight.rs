use anyhow::Result;
use log::{debug, info};
use std::collections::HashSet;

#[derive(Debug)]
struct Instruction {
    index: usize,
    command: String,
    value: i32,
}

impl Instruction {
    fn execute(&self, current_accumulation: i32, current_index: usize) -> (i32, usize) {
        return match self.command.as_str() {
            "acc" => (current_accumulation + self.value, current_index + 1),
            "jmp" => {
                debug!("Current at {} and jumping {}", current_index, self.value);
                (
                    current_accumulation,
                    (current_index as i32 + self.value) as usize,
                )
            }
            _ => (current_accumulation, current_index + 1),
        };
    }

    fn experiment(&self, current_accumulation: i32, current_index: usize) -> (i32, usize) {
        return match self.command.as_str() {
            "acc" => (current_accumulation + self.value, current_index + 1),
            "nop" => {
                debug!(
                    "Current at {} and switching nop to jumping at {}",
                    current_index, self.value
                );
                (
                    current_accumulation,
                    (current_index as i32 + self.value) as usize,
                )
            }
            _ => {
                debug!("Attempting to switch jump to nop");
                (current_accumulation, current_index + 1)
            }
        };
    }
}
impl From<(usize, &str)> for Instruction {
    fn from(tuple: (usize, &str)) -> Self {
        let x = tuple.1.split_whitespace().collect::<Vec<&str>>();
        Self {
            index: tuple.0,
            command: x[0].to_string(),
            value: x[1].parse::<i32>().unwrap(),
        }
    }
}
pub async fn solve() -> Result<()> {
    let instructions: Vec<Instruction> = include_str!("resources/day8.txt")
        .lines()
        .enumerate()
        .map(Instruction::from)
        .collect();

    info!("Part 1 Answer is: {:?}", part_one(&instructions).unwrap());
    info!("Part 2 Answer is: {:?}", part_two(&instructions).unwrap());

    Ok(())
}

fn part_one(instructions: &Vec<Instruction>) -> Result<i32> {
    let mut accumulation: i32 = 0;
    let mut index: usize = 0;
    let mut executed_instructions = HashSet::new();

    while !executed_instructions.contains(&index) {
        let (sum, next_index) = instructions[index].execute(accumulation, index);
        executed_instructions.insert(index);

        debug!("Current Accumulation: {}", accumulation);
        accumulation = sum;
        index = next_index;
    }

    Ok(accumulation)
}

fn part_two(instructions: &Vec<Instruction>) -> Result<i32> {
    let mut accumulations: (i32, i32) = (0, 0);
    let mut indexes: (usize, usize) = (0, 0);
    let mut executed_instructions = (HashSet::new(), HashSet::new());
    let mut experimenting = false;
    let mut resetting = false;

    debug!("instruction length: {}", instructions.len());

    while indexes.0 != instructions.len() {
        debug!("Indexes {:?}", indexes);
        let instruction = &instructions[indexes.0];
        let (sum, next_index) = if !resetting
            && !experimenting
            && (instruction.command == "jmp" || instruction.command == "nop")
        {
            experimenting = true;
            instruction.experiment(accumulations.0, indexes.0)
        } else {
            resetting = false;
            instruction.execute(accumulations.0, indexes.0)
        };

        executed_instructions.0.insert(indexes.0);

        if executed_instructions.0.contains(&next_index) {
            debug!(
                "Cycle detected, backtracking to {} where we've executed {:?}...",
                indexes.1, executed_instructions.1
            );
            accumulations.0 = accumulations.1;
            indexes.0 = indexes.1;
            executed_instructions.0 = executed_instructions.1.clone();
            experimenting = false;
            resetting = true;
            continue;
        }

        if experimenting {
            accumulations = (sum, accumulations.1);
            indexes = (next_index, indexes.1);
        } else {
            accumulations = (sum, sum);
            indexes = (next_index, next_index);
            executed_instructions.1.insert(indexes.0);
        }
    }

    return Ok(accumulations.0);
}
