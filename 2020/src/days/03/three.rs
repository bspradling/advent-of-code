use anyhow::Result;
use log::{debug, info, trace};

struct Slope {
    x: usize,
    y: usize,
}

impl Slope {
    pub fn new(x: usize, y: usize) -> Slope {
        Slope { x, y }
    }
}

#[derive(Clone, Debug)]
enum GridType {
    Open,
    Tree,
}

#[derive(Debug)]
struct GridSpace {
    grid_type: GridType,
}

impl GridSpace {
    fn from(character: char) -> GridSpace {
        match character {
            '.' => GridSpace {
                grid_type: GridType::Open,
            },
            '#' => GridSpace {
                grid_type: GridType::Tree,
            },
            _ => panic!("Invalid board input!"),
        }
    }

    fn grid_type(&self) -> GridType {
        self.grid_type.clone()
    }
}

pub async fn solve() -> Result<()> {
    info!("Part 01 Answer is: {}", count_trees(&Slope::new(3, 1))?);
    let product: usize = [
        Slope::new(1, 1),
        Slope::new(3, 1),
        Slope::new(5, 1),
        Slope::new(7, 1),
        Slope::new(1, 2),
    ]
    .iter()
    .map(|slope| count_trees(slope).expect("Unable to count trees!"))
    .product();
    info!("Part 02 Answer is: {}", product);
    Ok(())
}

fn count_trees(slope: &Slope) -> Result<usize> {
    let puzzle_input: Vec<Vec<GridSpace>> = include_str!("resources/day3.txt")
        .lines()
        .map(|lines| {
            lines
                .chars()
                .map(|character| GridSpace::from(character))
                .collect()
        })
        .collect();

    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;

    trace!("{:?}", puzzle_input);
    trace!("Puzzle Height: {}", puzzle_input.len());

    while y < puzzle_input.len() {
        match puzzle_input[y][x].grid_type() {
            GridType::Tree => {
                debug!("Tree found at coordinates {}, {}", y, x);
                trees += 1
            }
            _ => debug!("Open space found at coordinates {}, {}", y, x),
        }

        let row_length = puzzle_input[y].len();
        x += slope.x;
        y += slope.y;

        trace!("Row Length: {}", row_length);
        if x >= row_length {
            x = x - row_length;
        }
    }

    Ok(trees)
}
