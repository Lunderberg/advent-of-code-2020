use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone)]
enum Direction {
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct TileLoc {
    x: i32,
    y: i32,
    z: i32,
}

struct GameOfLife {
    active: HashSet<TileLoc>,
}

impl std::ops::Add<Direction> for TileLoc {
    type Output = TileLoc;
    fn add(self, dir: Direction) -> TileLoc {
        let delta = dir.as_delta();
        TileLoc {
            x: self.x + delta[0],
            y: self.y + delta[1],
            z: self.z + delta[2],
        }
    }
}

fn parse_line(line: &str) -> Result<Vec<Direction>, util::Error> {
    let mut chars = line.chars();

    let mut output = Vec::new();

    loop {
        let char = chars.next();
        let next_dir = match char {
            None => None,
            Some('e') => Some(Direction::East),
            Some('w') => Some(Direction::West),
            Some('n') => {
                let second_char = chars.next();
                match second_char {
                    Some('e') => Some(Direction::NorthEast),
                    Some('w') => Some(Direction::NorthWest),
                    _ => {
                        return Err(util::Error::ParseError);
                    }
                }
            }
            Some('s') => {
                let second_char = chars.next();
                match second_char {
                    Some('e') => Some(Direction::SouthEast),
                    Some('w') => Some(Direction::SouthWest),
                    _ => {
                        return Err(util::Error::ParseError);
                    }
                }
            }
            Some(_) => {
                return Err(util::Error::ParseError);
            }
        };

        match next_dir {
            None => {
                break;
            }
            Some(dir) => {
                output.push(dir);
            }
        }
    }

    Ok(output)
}

impl Direction {
    fn iter() -> std::slice::Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 6] = [
            Direction::East,
            Direction::West,
            Direction::NorthEast,
            Direction::NorthWest,
            Direction::SouthEast,
            Direction::SouthWest,
        ];
        DIRECTIONS.iter()
    }

    fn as_delta(&self) -> [i32; 3] {
        match self {
            Direction::East => [1, -1, 0],
            Direction::West => [-1, 1, 0],
            Direction::NorthEast => [0, -1, 1],
            Direction::SouthWest => [0, 1, -1],
            Direction::NorthWest => [-1, 0, 1],
            Direction::SouthEast => [1, 0, -1],
        }
    }
}

impl GameOfLife {
    fn next_iter(&self) -> GameOfLife {
        GameOfLife {
            active: self
                .active
                .iter()
                .map(|&tile| Direction::iter().map(move |&dir| tile + dir))
                .flatten()
                .collect::<HashSet<_>>()
                .iter()
                .filter(|&tile| {
                    let num_neighbors = self.num_active_neighbors(*tile);
                    (num_neighbors == 2)
                        || (num_neighbors == 1 && self.active.contains(tile))
                })
                .copied()
                .collect(),
        }
    }

    fn num_active_neighbors(&self, tile: TileLoc) -> usize {
        Direction::iter()
            .map(|&dir| tile + dir)
            .filter(|adj| self.active.contains(adj))
            .count()
    }
}

fn tile_location(directions: &[Direction]) -> TileLoc {
    directions
        .iter()
        .fold(TileLoc { x: 0, y: 0, z: 0 }, |loc, &dir| loc + dir)
}

fn main() -> Result<(), util::Error> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let text = std::fs::read_to_string(filename)?;
    let tiles = text
        .lines()
        .map(|line| parse_line(line))
        .collect::<Result<Vec<_>, _>>()?;

    let mut counts = HashMap::new();
    tiles.iter().for_each(|directions| {
        let tile = tile_location(directions);
        *counts.entry(tile).or_insert(0) += 1;
    });

    let active_tiles: HashSet<TileLoc> = counts
        .iter()
        .filter(|(_loc, flips)| *flips % 2 == 1)
        .map(|(loc, _flips)| loc)
        .copied()
        .collect();

    println!("Part 1, num black tiles: {}", active_tiles.len());

    let mut game = GameOfLife {
        active: active_tiles,
    };
    for _ in 0..100 {
        game = game.next_iter();
    }
    println!("Part 2, after 100 days: {}", game.active.len());

    Ok(())
}
