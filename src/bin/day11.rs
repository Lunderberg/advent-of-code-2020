#[derive(Debug, Copy, Clone, PartialEq)]
enum CellState {
    OccupiedChair,
    EmptyChair,
    Floor,
}

#[derive(Debug, PartialEq)]
struct Ferry {
    states: Vec<CellState>,
    height: usize,
    width: usize,
}

impl Ferry {
    fn parse(filename: &str) -> Result<Self, util::Error> {
        let coordinates = std::fs::read_to_string(filename)?
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| (x, y, c))
            })
            .collect::<Vec<_>>();

        let width = coordinates.iter().map(|(x, _y, _c)| x).max().unwrap() + 1;
        let height = coordinates.iter().map(|(_x, y, _c)| y).max().unwrap() + 1;

        let cell_contents: Vec<_> = coordinates
            .iter()
            .map(|(_x, _y, c)| match c {
                '.' => Ok(CellState::Floor),
                'L' => Ok(CellState::EmptyChair),
                '#' => Ok(CellState::OccupiedChair),
                _ => Err(util::Error::InvalidValue(c.to_string())),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            states: cell_contents,
            height,
            width,
        })
    }
}

impl Ferry {
    fn in_bounds(&self, x: i32, y: i32) -> bool {
        (x >= 0)
            && (x < self.width as i32)
            && (y >= 0)
            && (y < self.height as i32)
    }

    fn as_xy(&self, loc: usize) -> (i32, i32) {
        let x = (loc % (self.width as usize)) as i32;
        let y = (loc / (self.width as usize)) as i32;
        (x, y)
    }

    fn get_value(&self, x: i32, y: i32) -> CellState {
        if self.in_bounds(x, y) {
            let x = x as usize;
            let y = y as usize;
            let loc = (y * self.width + x) as usize;
            self.states[loc]
        } else {
            CellState::Floor
        }
    }

    fn num_neighbors(&self, loc: usize) -> usize {
        let (x, y) = self.as_xy(loc);

        vec![
            (1, 1),
            (1, 0),
            (1, -1),
            (0, 1),
            (0, -1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ]
        .iter()
        .map(|(dx, dy)| {
            (self.get_value(x + dx, y + dy) == CellState::OccupiedChair)
                as usize
        })
        .sum()
    }

    fn get_visible_value(
        &self,
        mut x: i32,
        mut y: i32,
        dx: i32,
        dy: i32,
    ) -> CellState {
        while self.in_bounds(x, y) {
            x += dx;
            y += dy;
            let val = self.get_value(x, y);
            if val != CellState::Floor {
                return val;
            }
        }
        CellState::Floor
    }

    fn num_visible_neighbors(&self, loc: usize) -> usize {
        let (x, y) = self.as_xy(loc);

        vec![
            (1, 1),
            (1, 0),
            (1, -1),
            (0, 1),
            (0, -1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ]
        .iter()
        .map(|(dx, dy)| {
            (self.get_visible_value(x, y, *dx, *dy) == CellState::OccupiedChair)
                as usize
        })
        .sum()
    }

    fn next_state_part1(&self, loc: usize) -> CellState {
        let num_neighbors = self.num_neighbors(loc);

        match self.states[loc] {
            CellState::Floor => CellState::Floor,

            CellState::EmptyChair => {
                if num_neighbors == 0 {
                    CellState::OccupiedChair
                } else {
                    CellState::EmptyChair
                }
            }

            CellState::OccupiedChair => {
                if num_neighbors >= 4 {
                    CellState::EmptyChair
                } else {
                    CellState::OccupiedChair
                }
            }
        }
    }

    fn next_state_part2(&self, loc: usize) -> CellState {
        let num_neighbors = self.num_visible_neighbors(loc);

        match self.states[loc] {
            CellState::Floor => CellState::Floor,

            CellState::EmptyChair => {
                if num_neighbors == 0 {
                    CellState::OccupiedChair
                } else {
                    CellState::EmptyChair
                }
            }

            CellState::OccupiedChair => {
                if num_neighbors >= 5 {
                    CellState::EmptyChair
                } else {
                    CellState::OccupiedChair
                }
            }
        }
    }

    fn iterate_part1(&self) -> Self {
        let states = self
            .states
            .iter()
            .enumerate()
            .map(|(loc, _state)| self.next_state_part1(loc))
            .collect();

        Ferry {
            height: self.height,
            width: self.width,
            states,
        }
    }

    fn iterate_part2(&self) -> Self {
        let states = self
            .states
            .iter()
            .enumerate()
            .map(|(loc, _state)| self.next_state_part2(loc))
            .collect();

        Ferry {
            height: self.height,
            width: self.width,
            states,
        }
    }

    fn num_occupied(&self) -> usize {
        self.states
            .iter()
            .filter(|state| **state == CellState::OccupiedChair)
            .count()
    }
}

impl std::fmt::Display for Ferry {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.states
            .iter()
            .map(|s| match s {
                CellState::OccupiedChair => '#',
                CellState::EmptyChair => 'L',
                CellState::Floor => '.',
            })
            .collect::<Vec<_>>()
            .chunks(self.width)
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|c| write!(fmt, "{}", c))
                    .collect::<Result<Vec<_>, _>>()?;
                writeln!(fmt)?;
                Ok(())
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }
}

fn main() -> Result<(), util::Error> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let mut state = Ferry::parse(filename)?;
    loop {
        let next_state = state.iterate_part1();
        if state == next_state {
            break;
        }
        state = next_state;
    }

    println!("Stable number of seated, p1: {}", state.num_occupied());

    let mut state = Ferry::parse(filename)?;
    loop {
        let next_state = state.iterate_part2();
        if state == next_state {
            break;
        }
        state = next_state;
    }

    println!("Stable number of seated, p2: {}", state.num_occupied());

    Ok(())
}
