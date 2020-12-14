use itertools::Itertools;
use std::collections::HashMap;

use util;

#[derive(Debug, Copy, Clone)]
struct Mask {
    mask_0: u64,
    mask_1: u64,
    mask_x: u64,
}

#[derive(Debug, Copy, Clone)]
enum Command {
    Mask(Mask),
    Memset { location: u64, value: u64 },
}

impl std::str::FromStr for Command {
    type Err = util::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("mask") {
            let mask_str = s.split_whitespace().last().unwrap();
            let mask_0 = mask_str
                .chars()
                .fold(0u64, |acc, c| 2 * acc + ((c == '0') as u64));

            let mask_1 = mask_str
                .chars()
                .fold(0u64, |acc, c| 2 * acc + ((c == '1') as u64));

            let mask_x = mask_str
                .chars()
                .fold(0u64, |acc, c| 2 * acc + ((c == 'X') as u64));

            Ok(Command::Mask(Mask {
                mask_0: mask_0,
                mask_1: mask_1,
                mask_x: mask_x,
            }))
        } else if s.starts_with("mem") {
            let components: Vec<_> =
                s.split(|c| c == '[' || c == ']' || c == ' ').collect();
            let location = components[1].parse::<u64>()?;
            let value = components[4].parse::<u64>()?;

            Ok(Command::Memset {
                location: location,
                value: value,
            })
        } else {
            Err(util::Error::InvalidValue(s.to_owned()))
        }
    }
}

#[derive(Debug)]
struct DecoderState {
    mask: Mask,
    mem: HashMap<u64, u64>,
}

impl DecoderState {
    fn new() -> Self {
        Self {
            mask: Mask {
                mask_0: 0u64,
                mask_1: 0u64,
                mask_x: 0u64,
            },
            mem: HashMap::new(),
        }
    }

    fn apply_part1(&mut self, c: Command) {
        match c {
            Command::Mask(mask) => {
                self.mask = mask;
            }

            Command::Memset { location, value } => {
                self.mem.insert(
                    location,
                    (value & (!self.mask.mask_0)) | self.mask.mask_1,
                );
            }
        }
    }

    fn apply_part2(&mut self, c: Command) {
        match c {
            Command::Mask(mask) => {
                self.mask = mask;
            }

            Command::Memset { location, value } => {
                let location =
                    (location | self.mask.mask_1) & !self.mask.mask_x;

                let floating_bits: Vec<_> = (0..36)
                    .filter(|b| self.mask.mask_x & (1u64 << b) > 0)
                    .collect();

                (0..=floating_bits.len())
                    .map(|len| floating_bits.iter().combinations(len))
                    .flatten()
                    .for_each(|bitset| {
                        let location_mod = bitset
                            .iter()
                            .fold(location, |acc, &bit| acc | (1u64 << bit));
                        self.mem.insert(location_mod, value);
                    })
            }
        }
    }
}

fn main() -> Result<(), util::Error> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let commands = std::fs::read_to_string(filename)?
        .lines()
        .map(|s| s.parse::<Command>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut state = DecoderState::new();
    commands.iter().for_each(|c| state.apply_part1(*c));

    let sum = state.mem.iter().map(|(_k, v)| v).sum::<u64>();
    println!("Part a, sum = {}", sum);

    let mut state = DecoderState::new();
    commands.iter().for_each(|&c| state.apply_part2(c));

    let sum = state.mem.iter().map(|(_k, v)| v).sum::<u64>();
    println!("Part b, sum = {}", sum);

    Ok(())
}
