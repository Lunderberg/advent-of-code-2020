use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Tile {
    num: i64,
    values: Vec<bool>,
    width: usize,
    height: usize,
}

impl Display for Tile {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "Tile: {}\n", self.num)?;
        for row in 0..self.height {
            let row_str = self.values[self.width * row..self.width * (row + 1)]
                .iter()
                .map(|&bit| if bit { '#' } else { '.' })
                .collect::<String>();
            write!(fmt, "{}\n", row_str)?;
        }
        Ok(())
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum SubLocation {
    Top,
    Bottom,
    Left,
    Right,
}

impl SubLocation {
    fn iter() -> std::slice::Iter<'static, SubLocation> {
        static LOCS: [SubLocation; 4] = [
            SubLocation::Top,
            SubLocation::Bottom,
            SubLocation::Left,
            SubLocation::Right,
        ];
        LOCS.iter()
    }

    fn opposite(&self) -> SubLocation {
        match self {
            SubLocation::Top => SubLocation::Bottom,
            SubLocation::Bottom => SubLocation::Top,
            SubLocation::Left => SubLocation::Right,
            SubLocation::Right => SubLocation::Left,
        }
    }

    fn delta(&self) -> (i32, i32) {
        match self {
            SubLocation::Top => (0, -1),
            SubLocation::Bottom => (0, 1),
            SubLocation::Left => (-1, 0),
            SubLocation::Right => (1, 0),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Rotation {
    Rot0,
    Rot90,
    Rot180,
    Rot270,
}

#[derive(Copy, Clone, Debug)]
struct Transformation {
    rotation: Rotation,
    flipped: bool,
}

impl Display for Transformation {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let flip_char = if self.flipped { 'F' } else { ' ' };
        let rot_str = match self.rotation {
            Rotation::Rot0 => "0  ",
            Rotation::Rot90 => "90 ",
            Rotation::Rot180 => "180",
            Rotation::Rot270 => "270",
        };
        write!(fmt, "{}+{}", flip_char, rot_str)?;
        Ok(())
    }
}

impl Transformation {
    fn iter() -> std::slice::Iter<'static, Transformation> {
        static TRANSFORMS: [Transformation; 8] = [
            Transformation {
                rotation: Rotation::Rot0,
                flipped: false,
            },
            Transformation {
                rotation: Rotation::Rot90,
                flipped: false,
            },
            Transformation {
                rotation: Rotation::Rot180,
                flipped: false,
            },
            Transformation {
                rotation: Rotation::Rot270,
                flipped: false,
            },
            Transformation {
                rotation: Rotation::Rot0,
                flipped: true,
            },
            Transformation {
                rotation: Rotation::Rot90,
                flipped: true,
            },
            Transformation {
                rotation: Rotation::Rot180,
                flipped: true,
            },
            Transformation {
                rotation: Rotation::Rot270,
                flipped: true,
            },
        ];
        TRANSFORMS.iter()
    }
}

trait BitIterator {
    fn to_int(self) -> u64;
}

impl<T, F> BitIterator for T
where
    T: Iterator<Item = F>,
    F: std::borrow::Borrow<bool>,
{
    fn to_int(self) -> u64 {
        self.fold(0, |acc, bit| 2 * acc + (*bit.borrow() as u64))
    }
}

impl Tile {
    fn new(text: &str) -> Result<Tile, util::Error> {
        let mut lines = text.lines();

        let first_line = lines.next().unwrap();
        let num = Regex::new(r"Tile (?P<num>\d+):")
            .unwrap()
            .captures(first_line)
            .ok_or(util::Error::NoneError)?
            .name("num")
            .unwrap()
            .as_str()
            .parse::<i64>()?;

        let loc_values: Vec<_> = lines
            .enumerate()
            .map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| (x, y, c))
            })
            .flatten()
            .collect();

        let width = loc_values.iter().map(|(x, _y, _c)| x).max().unwrap() + 1;
        let height = loc_values.iter().map(|(_x, y, _c)| y).max().unwrap() + 1;
        let values = loc_values.iter().map(|(_x, _y, c)| *c == '#').collect();

        Ok(Tile {
            num,
            values,
            height,
            width,
        })
    }

    fn get_values_at(&self, subloc: SubLocation) -> u64 {
        match subloc {
            SubLocation::Top => self.values[..self.width].iter().to_int(),
            SubLocation::Bottom => self.values
                [self.values.len() - self.width..]
                .iter()
                .to_int(),

            SubLocation::Left => {
                self.values.iter().step_by(self.width).to_int()
            }

            SubLocation::Right => self
                .values
                .iter()
                .skip(self.width - 1)
                .step_by(self.width)
                .to_int(),
        }
    }

    fn transform(&self, transform: Transformation) -> Tile {
        let new_values: Vec<_> = self
            .values
            .iter()
            .enumerate()
            .map(|(i, v)| (i % self.width, i / self.width, v))
            .map(|(x, y, v)| {
                if transform.flipped {
                    (self.width - 1 - x, y, v)
                } else {
                    (x, y, v)
                }
            })
            .map(|(x, y, v)| match transform.rotation {
                Rotation::Rot0 => (x, y, v),
                Rotation::Rot90 => (self.width - 1 - y, x, v),
                Rotation::Rot180 => (self.width - 1 - x, self.width - 1 - y, v),
                Rotation::Rot270 => (y, self.width - 1 - x, v),
            })
            .sorted_by_key(|(x, y, _v)| y * self.width + x)
            .map(|(_x, _y, v)| *v)
            .collect();

        Tile {
            num: self.num,
            values: new_values,
            width: self.width,
            height: self.height,
        }
    }

    fn sea_monster() -> Vec<(usize, usize)> {
        let output: Vec<(usize, usize)> = vec![
            "                  # ",
            "#    ##    ##    ###",
            " #  #  #  #  #  #   ",
        ]
        .iter()
        .enumerate()
        .map(|(y, line)| line.chars().enumerate().map(move |(x, c)| (x, y, c)))
        .flatten()
        .filter(|(_x, _y, c)| *c == '#')
        .map(|(x, y, _c)| (x, y))
        .collect::<Vec<_>>();

        output
    }

    fn sea_monster_at(&self, x: usize, y: usize) -> bool {
        if (x + 20 >= self.width) || (y + 2 >= self.height) {
            return false;
        }

        Self::sea_monster().iter().all(|(dx, dy)| {
            let index = self.width * (y + dy) + (x + dx);
            self.values[index]
        })
    }

    fn count_sea_monsters(&self) -> usize {
        (0..self.width)
            .cartesian_product(0..self.height)
            .filter(|&(x, y)| self.sea_monster_at(x, y))
            .count()
    }
}

#[derive(Debug)]
struct TileLocation {
    x: i32,
    y: i32,
    transform: Transformation,
    tile_num: i64,
}

fn tile_layout(tiles: &Vec<Tile>) -> Vec<TileLocation> {
    let mut tiles_remaining: Vec<_> = tiles.iter().collect();

    let mut placement = Vec::new();

    // Stack of places where a puzzle piece could be placed
    let mut possible_places: Vec<(i32, i32)> = vec![(0, 0)];

    // Record of which places have either been used, or already added
    // to possible_places
    let mut used_places: HashSet<(i32, i32)> = HashSet::new();
    used_places.insert((0, 0));

    // Conditions that must be met for a new puzzle piece to be valid at a given location
    let mut conditions_lookup: HashMap<(i32, i32), Vec<(SubLocation, u64)>> =
        HashMap::new();

    // Assume no contradictions arise when looking up values
    while !possible_places.is_empty() {
        let new_loc = possible_places.pop().unwrap();
        let conditions = conditions_lookup.entry(new_loc).or_default();

        // Find a new tile that matches all conditions on the location
        // being searched.
        let new_tile = tiles_remaining
            .iter()
            .enumerate()
            .cartesian_product(Transformation::iter())
            .map(|((i, tile), transform)| {
                (i, tile.transform(*transform), transform)
            })
            .find(|(_i, tile, _transform)| {
                conditions
                    .iter()
                    .all(|&(subloc, x)| tile.get_values_at(subloc) == x)
            });

        if let Some((index, tile, transform)) = new_tile {
            // Add to the output vector
            placement.push(TileLocation {
                x: new_loc.0,
                y: new_loc.1,
                tile_num: tile.num,
                transform: *transform,
            });

            // Remove from the list of tiles that we could still place
            tiles_remaining.remove(index);

            // Add to the list of locations that we could add the next
            // tile.
            SubLocation::iter()
                .map(|subloc| subloc.delta())
                .map(|(dx, dy)| (new_loc.0 + dx, new_loc.1 + dy))
                .filter(|adjacent| !used_places.contains(adjacent))
                .collect::<Vec<_>>()
                .iter()
                .for_each(|&adjacent| {
                    possible_places.push(adjacent);
                    used_places.insert(adjacent);
                });

            // Also, append to the conditions required for all
            // adjacent tiles.
            SubLocation::iter()
                .map(|subloc| (subloc, subloc.delta()))
                .map(|(subloc, (dx, dy))| {
                    (subloc, (new_loc.0 + dx, new_loc.1 + dy))
                })
                .for_each(|(subloc, adjacent)| {
                    let condition =
                        (subloc.opposite(), tile.get_values_at(*subloc));
                    conditions_lookup
                        .entry(adjacent)
                        .or_default()
                        .push(condition);
                });
        }
    }

    // If this fails, then we need to handle contradictions and
    // backtracking in the tile placement.
    assert!(tiles_remaining.is_empty());

    placement
}

#[allow(dead_code)]
fn print_layout(placement: &Vec<TileLocation>) {
    let xmin = placement.iter().map(|p| p.x).min().unwrap();
    let xmax = placement.iter().map(|p| p.x).max().unwrap();
    let ymin = placement.iter().map(|p| p.y).min().unwrap();
    let ymax = placement.iter().map(|p| p.y).max().unwrap();

    for y in ymin..=ymax {
        for x in xmin..=xmax {
            let s = placement
                .iter()
                .find(|p| (p.x == x) && (p.y == y))
                .map(|p| p.tile_num.to_string())
                .or(Some("    ".to_string()))
                .unwrap();
            print!(" ({}) ", s);
        }
        print!("\n");
    }
}

#[allow(dead_code)]
fn print_tiles(tiles: &Vec<Tile>, placement: &Vec<TileLocation>) {
    let xmin = placement.iter().map(|p| p.x).min().unwrap();
    let xmax = placement.iter().map(|p| p.x).max().unwrap();
    let ymin = placement.iter().map(|p| p.y).min().unwrap();
    let ymax = placement.iter().map(|p| p.y).max().unwrap();

    let row_str = |x, y, row| {
        placement
            .iter()
            .find(|p| (p.x == x) && (p.y == y))
            .map(|p| {
                let tile = tiles
                    .iter()
                    .find(|t| t.num == p.tile_num)
                    .unwrap()
                    .transform(p.transform);
                tile.values[row * tile.width..(row + 1) * tile.width]
                    .iter()
                    .map(|&bit| if bit { '#' } else { '.' })
                    .collect::<String>()
            })
            .or(Some("          ".to_string()))
            .unwrap()
    };
    let tile_str = |x, y| {
        placement
            .iter()
            .find(|p| (p.x == x) && (p.y == y))
            .map(|p| format!("-{}- {} ", p.tile_num, p.transform))
            .or(Some("             ".to_string()))
            .unwrap()
    };

    let num_rows = tiles.iter().map(|t| t.height).max().unwrap();

    for y in ymin..=ymax {
        for x in xmin..=xmax {
            print!("{}", tile_str(x, y));
        }
        print!("\n");

        for row in 0..num_rows {
            for x in xmin..=xmax {
                print!("{}   ", row_str(x, y, row));
            }
            print!("\n");
        }
        print!("\n\n");
    }
}

fn merge_images(tiles: &Vec<Tile>, placement: &Vec<TileLocation>) -> Tile {
    let xmin = placement.iter().map(|p| p.x).min().unwrap();
    let xmax = placement.iter().map(|p| p.x).max().unwrap();
    let ymin = placement.iter().map(|p| p.y).min().unwrap();
    let ymax = placement.iter().map(|p| p.y).max().unwrap();

    let tile_map: HashMap<(i32, i32), Tile> = placement
        .iter()
        .map(|p| {
            (
                (p.x, p.y),
                tiles
                    .iter()
                    .find(|t| t.num == p.tile_num)
                    .unwrap()
                    .transform(p.transform),
            )
        })
        .collect();

    let tile_width = tiles.iter().next().unwrap().width;
    let tile_height = tiles.iter().next().unwrap().width;

    let width = (tile_width - 2) * ((xmax - xmin + 1) as usize);
    let height = (tile_height - 2) * ((ymax - ymin + 1) as usize);

    let values: Vec<_> = (ymin..=ymax)
        .cartesian_product(1..(tile_height - 1))
        .cartesian_product(xmin..=xmax)
        .map(|((y, row), x)| {
            let tile = tile_map.get(&(x, y)).unwrap();
            tile.values[row * tile_width + 1..(row + 1) * tile_width - 1].iter()
        })
        .flatten()
        .map(|x| *x)
        .collect();

    Tile {
        num: 0,
        width,
        height,
        values,
    }
}

fn main() -> Result<(), util::Error> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let text = std::fs::read_to_string(filename)?;
    let tiles = text
        .split("\n\n")
        .map(|section| Tile::new(section))
        .collect::<Result<Vec<_>, _>>()?;

    let placement = tile_layout(&tiles);

    // print_layout(&placement);
    // print_tiles(&tiles, &placement);

    let xmin = placement.iter().map(|p| p.x).min().unwrap();
    let xmax = placement.iter().map(|p| p.x).max().unwrap();
    let ymin = placement.iter().map(|p| p.y).min().unwrap();
    let ymax = placement.iter().map(|p| p.y).max().unwrap();

    let corner_prod = placement
        .iter()
        .filter(|p| {
            ((p.x == xmin) || (p.x == xmax)) && ((p.y == ymin) || (p.y == ymax))
        })
        .map(|p| p.tile_num)
        .product::<i64>();
    println!("Part 1, corner prod = {}", corner_prod);

    let image = merge_images(&tiles, &placement);

    let num_sea_monsters = Transformation::iter()
        .map(|&t| image.transform(t).count_sea_monsters())
        .max()
        .unwrap();

    let choppy_water =
        image.values.iter().map(|&bit| bit as usize).sum::<usize>()
            - num_sea_monsters * Tile::sea_monster().len();

    println!("Part 2, choppy water = {}", choppy_water);

    Ok(())
}
