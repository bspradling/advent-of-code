use anyhow::Result;

#[cfg(test)]
mod tests;

pub struct OceanFloor {
    pub floor: Vec<Vec<u32>>,
}

impl OceanFloor {
    pub fn parse(_input: &str) -> Result<Self> {
        Ok(OceanFloor { floor: Vec::new() })
    }

    pub fn dangerous_locations(&self) -> Result<usize> {
        Ok(self.floor.iter().flatten().filter(|x| **x > 2).count())
    }
}

pub fn part_one(floor: OceanFloor) -> Result<usize> {
    floor.dangerous_locations()
}

pub fn part_two() -> Result<()> {
    Ok(())
}
