use std::collections::HashSet;

use itertools::Itertools;

trait GameOfLifeLocation: Eq + PartialEq + std::hash::Hash {
    fn adjacent(&self) -> Vec<Self>
    where
        Self: Sized;
}

struct GameOfLife<T>
where
    T: GameOfLifeLocation,
{
    active: HashSet<T>,
}

impl<T> GameOfLife<T>
where
    T: GameOfLifeLocation,
{
    fn next(&self) -> Self {
        let active: HashSet<_> = self
            .active
            .iter()
            .map(|pos| pos.adjacent())
            .flatten()
            .collect::<HashSet<_>>()
            .into_iter()
            .filter(|pos| {
                let num_neighbors = self.count_active_neighbors(&pos);
                let is_active = self.active.contains(&pos);
                (is_active && ((num_neighbors == 2) || (num_neighbors == 3)))
                    || (!is_active && (num_neighbors == 3))
            })
            .collect();

        Self { active }
    }

    fn count_active_neighbors(&self, loc: &T) -> usize {
        loc.adjacent()
            .iter()
            .filter(|&pos| pos != loc)
            .filter(|pos| self.active.contains(&pos))
            .count()
    }
}

#[derive(Hash, Eq, PartialEq)]
struct Cube3D {
    pos: [i32; 3],
}

impl GameOfLifeLocation for Cube3D {
    fn adjacent(&self) -> Vec<Self> {
        (-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .map(|((dx, dy), dz)| Self {
                pos: [self.pos[0] + dx, self.pos[1] + dy, self.pos[2] + dz],
            })
            .collect()
    }
}

#[derive(Hash, Eq, PartialEq)]
struct Cube4D {
    pos: [i32; 4],
}

impl GameOfLifeLocation for Cube4D {
    fn adjacent(&self) -> Vec<Self> {
        (-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .map(|(((dx, dy), dz), dw)| Self {
                pos: [
                    self.pos[0] + dx,
                    self.pos[1] + dy,
                    self.pos[2] + dz,
                    self.pos[3] + dw,
                ],
            })
            .collect()
    }
}

fn main() -> Result<(), util::Error> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let starter: HashSet<_> = std::fs::read_to_string(filename)?
        .lines()
        .enumerate()
        .map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (x, y, c)))
        .flatten()
        .filter(|&(_x, _y, c)| c == '#')
        .map(|(x, y, _c)| (x as i32, y as i32))
        .collect();

    let mut game = GameOfLife {
        active: starter
            .iter()
            .map(|(x, y)| Cube3D { pos: [*x, *y, 0] })
            .collect(),
    };
    for _ in 0..6 {
        game = game.next();
    }

    println!("Part a, num active: {}", game.active.len());

    let mut game = GameOfLife {
        active: starter
            .iter()
            .map(|(x, y)| Cube4D {
                pos: [*x, *y, 0, 0],
            })
            .collect(),
    };
    for _ in 0..6 {
        game = game.next();
    }

    println!("Part b, num active: {}", game.active.len());

    Ok(())
}
