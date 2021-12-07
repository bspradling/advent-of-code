use anyhow::Result;

#[derive(Debug)]
struct Position {
    _coordinates: (usize, usize),
    _position_type: PositionType,
}

impl From<((usize, usize), char)> for Position {
    fn from(tuple: ((usize, usize), char)) -> Self {
        Position {
            _coordinates: tuple.0,
            _position_type: match tuple.1 {
                'L' | '#' => PositionType::SEAT,
                _ => PositionType::FLOOR,
            },
        }
    }
}

#[derive(Debug)]
enum PositionType {
    FLOOR,
    SEAT,
}

#[derive(Debug)]
enum SeatStatus {
    _EMPTY,
    _OCCUPIED,
}

pub async fn solve() -> Result<()> {
    let _seating = include_str!("resources/day11.txt")
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(column, character)| Position::from(((row, column), character)))
                .collect::<Vec<Position>>()
        })
        .collect::<Vec<Vec<Position>>>();

    Ok(())
}
