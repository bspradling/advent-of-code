use anyhow::Result;

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Selection {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Eq, PartialEq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

#[derive(Debug)]
pub struct RockPaperScissors {
    opponent: Selection,
    myself: Selection,
}

pub fn part_one(games: Vec<RockPaperScissors>) -> Result<u32> {
    Ok(games
        .iter()
        .map(|game| game.game_score() + game.selection_score())
        .sum())
}

pub fn part_two(games: Vec<RockPaperScissors>) -> Result<u32> {
    Ok(games
        .iter()
        .map(|game| game.game_score() + game.selection_score())
        .sum())
}

impl RockPaperScissors {
    pub fn from(game: &str) -> Self {
        Self {
            opponent: Selection::from(&game[..1]),
            myself: Selection::from(&game[2..]),
        }
    }

    pub fn from_outcome(game: &str) -> Self {
        let outcome = Outcome::from(&game[2..]);
        let opponent = Selection::from(&game[..1]);
        Self {
            opponent: opponent.clone(),
            myself: Outcome::to_select(outcome, &opponent),
        }
    }

    pub fn selection_score(&self) -> u32 {
        match self.myself {
            Selection::Rock => 1,
            Selection::Paper => 2,
            Selection::Scissors => 3,
        }
    }

    pub fn game_score(&self) -> u32 {
        if self.myself == self.opponent {
            return 3;
        }

        if self.myself.beats() == self.opponent {
            return 6;
        }

        return 0;
    }
}

impl Selection {
    pub fn from(string: &str) -> Self {
        match string {
            "A" | "X" => Selection::Rock,
            "B" | "Y" => Selection::Paper,
            "C" | "Z" => Selection::Scissors,
            _ => panic!("Unsupported character {}", string),
        }
    }

    pub fn beats(&self) -> Selection {
        match self {
            Selection::Rock => Selection::Scissors,
            Selection::Paper => Selection::Rock,
            Selection::Scissors => Selection::Paper,
        }
    }

    pub fn loses(&self) -> Selection {
        match self {
            Selection::Rock => Selection::Paper,
            Selection::Paper => Selection::Scissors,
            Selection::Scissors => Selection::Rock,
        }
    }
}

impl Outcome {
    pub fn from(string: &str) -> Self {
        match string {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Unsupported character {}", string),
        }
    }

    pub fn to_select(outcome: Outcome, opponent: &Selection) -> Selection {
        match outcome {
            Outcome::Draw => opponent.clone(),
            Outcome::Win => opponent.loses().clone(),
            Outcome::Lose => opponent.beats().clone(),
        }
    }
}
