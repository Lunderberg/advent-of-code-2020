use util;

#[derive(Debug)]
struct BoatState {
    x: i32,
    y: i32,
    facing: Direction,
}

impl BoatState {
    fn apply_command(&self, c: Command) -> BoatState {
        match c {
            Command::Move(dir, dist) => match dir {
                Direction::North => BoatState {
                    x: self.x,
                    y: self.y + dist,
                    facing: self.facing,
                },
                Direction::South => BoatState {
                    x: self.x,
                    y: self.y - dist,
                    facing: self.facing,
                },
                Direction::East => BoatState {
                    x: self.x + dist,
                    y: self.y,
                    facing: self.facing,
                },
                Direction::West => BoatState {
                    x: self.x - dist,
                    y: self.y,
                    facing: self.facing,
                },
            },

            Command::Forward(dist) => self.apply_command(Command::Move(self.facing, dist)),

            Command::RotateLeft => {
                let new_facing = match self.facing {
                    Direction::North => Direction::West,
                    Direction::West => Direction::South,
                    Direction::South => Direction::East,
                    Direction::East => Direction::North,
                };
                BoatState {
                    x: self.x,
                    y: self.y,
                    facing: new_facing,
                }
            }

            Command::RotateRight => {
                let new_facing = match self.facing {
                    Direction::North => Direction::East,
                    Direction::West => Direction::North,
                    Direction::South => Direction::West,
                    Direction::East => Direction::South,
                };
                BoatState {
                    x: self.x,
                    y: self.y,
                    facing: new_facing,
                }
            }

            Command::TurnAround => {
                let new_facing = match self.facing {
                    Direction::North => Direction::South,
                    Direction::West => Direction::East,
                    Direction::South => Direction::North,
                    Direction::East => Direction::West,
                };
                BoatState {
                    x: self.x,
                    y: self.y,
                    facing: new_facing,
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Copy, Clone)]
enum Command {
    Move(Direction, i32),
    Forward(i32),
    RotateLeft,
    RotateRight,
    TurnAround,
}

impl std::str::FromStr for Command {
    type Err = util::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let c = chars.next().ok_or(util::Error::NoneError)?;
        let amount = chars.as_str().parse::<i32>()?;

        match c {
            'N' => Ok(Command::Move(Direction::North, amount)),
            'S' => Ok(Command::Move(Direction::South, amount)),
            'E' => Ok(Command::Move(Direction::East, amount)),
            'W' => Ok(Command::Move(Direction::West, amount)),
            'F' => Ok(Command::Forward(amount)),
            'L' => match amount {
                90 => Ok(Command::RotateLeft),
                180 => Ok(Command::TurnAround),
                270 => Ok(Command::RotateRight),
                _ => Err(util::Error::InvalidValue(s.to_owned())),
            },
            'R' => match amount {
                90 => Ok(Command::RotateRight),
                180 => Ok(Command::TurnAround),
                270 => Ok(Command::RotateLeft),
                _ => Err(util::Error::InvalidValue(s.to_owned())),
            },
            _ => Err(util::Error::InvalidValue(s.to_owned())),
        }
    }
}

#[derive(Debug)]
struct WayPoint {
    waypoint_x: i32,
    waypoint_y: i32,
    boat_x: i32,
    boat_y: i32,
}

impl WayPoint {
    fn apply_command(&mut self, c: Command) {
        match c {
            Command::Move(dir, dist) => match dir {
                Direction::North => {
                    self.waypoint_y += dist;
                }
                Direction::South => {
                    self.waypoint_y -= dist;
                }
                Direction::East => {
                    self.waypoint_x += dist;
                }
                Direction::West => {
                    self.waypoint_x -= dist;
                }
            },

            Command::Forward(num) => {
                self.boat_x += num * self.waypoint_x;
                self.boat_y += num * self.waypoint_y;
            }

            Command::RotateLeft => {
                let temp = (-self.waypoint_y, self.waypoint_x);
                self.waypoint_x = temp.0;
                self.waypoint_y = temp.1;
            }

            Command::RotateRight => {
                let temp = (self.waypoint_y, -self.waypoint_x);
                self.waypoint_x = temp.0;
                self.waypoint_y = temp.1;
            }

            Command::TurnAround => {
                let temp = (-self.waypoint_x, -self.waypoint_y);
                self.waypoint_x = temp.0;
                self.waypoint_y = temp.1;
            }
        }
    }
}

fn main() -> Result<(), util::Error> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let commands = std::fs::read_to_string(filename)?
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| line.parse::<Command>())
        .collect::<Result<Vec<_>, _>>()?;

    let initial = BoatState {
        x: 0,
        y: 0,
        facing: Direction::East,
    };

    println!("Initial = {:?}", initial);
    let final_pos = commands
        .iter()
        .fold(initial, |boat, command| boat.apply_command(*command));

    println!(
        "Part a, pos = ({}, {}), Manhattan={}",
        final_pos.x,
        final_pos.y,
        final_pos.x.abs() + final_pos.y.abs()
    );

    let mut waypoint = WayPoint {
        waypoint_x: 10,
        waypoint_y: 1,
        boat_x: 0,
        boat_y: 0,
    };
    commands.iter().for_each(|c| waypoint.apply_command(*c));
    println!(
        "Part b, pos = ({}, {}), Manhattan={}",
        waypoint.boat_x,
        waypoint.boat_y,
        waypoint.boat_x.abs() + waypoint.boat_y.abs()
    );

    Ok(())
}
