use anyhow::Result;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug)]
pub struct Interval {
    start: i32,
    end: i32,
}

impl Interval {
    pub fn parse(string: &str) -> Result<Self> {
        let splitted_string = string.split("-").collect::<Vec<&str>>();

        Ok(Interval {
            start: splitted_string[0].parse::<i32>()?,
            end: splitted_string[1].parse::<i32>()?,
        })
    }

    pub fn contains(self, interval: &Interval) -> bool {
        return (self.start <= interval.start && self.end >= interval.start)
            || (self.start <= interval.end && self.end >= interval.end);
    }

    pub fn fully_contains(self, interval: &Interval) -> bool {
        return self.start <= interval.start && self.end >= interval.end;
    }
}
pub fn part_one(input: Vec<Vec<Interval>>) -> Result<i32> {
    Ok(input.iter().fold(0, |acc, x| {
        match x[0].clone().fully_contains(&x[1]) || x[1].clone().fully_contains(&x[0]) {
            true => acc + 1,
            false => acc,
        }
    }))
}

pub fn part_two(input: Vec<Vec<Interval>>) -> Result<i32> {
    Ok(input.iter().fold(0, |acc, x| {
        match x[0].clone().contains(&x[1]) || x[1].clone().contains(&x[0]) {
            true => acc + 1,
            false => acc,
        }
    }))
}
