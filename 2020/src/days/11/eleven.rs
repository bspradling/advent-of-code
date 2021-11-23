use anyhow::Result;

#[derive(Debug)]
struct Position {
    coordinates: (usize, usize),
    r#type: PositionType,
    status: SeatStatus,
}

impl From<((usize, usize), char)> for Position {
    fn from(tuple: ((usize, usize), char)) -> Self {
        Position {
            coordinates: tuple.0,
            r#type: match tuple.1 {
                'L' | '#' => PositionType::SEAT,
                _ => PositionType::FLOOR,
            },
            status: match tuple.1 {
                'L' => SeatStatus::EMPTY,
                '#' => SeatStatus::OCCUPIED,
                _ => SeatStatus::EMPTY,
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
    EMPTY,
    OCCUPIED,
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
