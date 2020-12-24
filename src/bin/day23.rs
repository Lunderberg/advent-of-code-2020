use std::collections::VecDeque;

use indicatif::ProgressBar;

#[derive(Debug)]
struct Cups {
    cups: VecDeque<usize>,
    active_cup_index: usize,
}

impl std::str::FromStr for Cups {
    type Err = util::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Cups {
            cups: s
                .trim()
                .chars()
                .map(|c| c.to_string().parse::<usize>())
                .collect::<Result<VecDeque<_>, _>>()?,
            active_cup_index: 0,
        })
    }
}

impl Cups {
    fn extend(&mut self, max_val: usize) {
        while self.cups.len() < max_val {
            self.cups.push_back(self.cups.len() + 1);
        }
    }

    fn next_iter(&self) -> Self {
        let num_cups = self.cups.len();
        let max_cup_value = self.cups.iter().max().unwrap();
        let active_cup_value = self.cups[self.active_cup_index];

        let mut new_cups = self.cups.clone();
        new_cups.rotate_left((self.active_cup_index + 1) % new_cups.len());

        let pick_up: VecDeque<_> =
            (0..3).map(|_| new_cups.pop_front().unwrap()).collect();

        let destination = new_cups
            .iter()
            .filter(|c| **c != active_cup_value)
            .min_by_key(|c| {
                (max_cup_value + active_cup_value - *c) % max_cup_value
            })
            .copied()
            .unwrap();

        let destination_index =
            new_cups.iter().position(|c| *c == destination).unwrap();
        new_cups.rotate_left((destination_index + 1) % new_cups.len());

        pick_up.iter().for_each(|c| new_cups.push_back(*c));

        new_cups.rotate_right(
            (self.active_cup_index + destination_index + 5) % new_cups.len(),
        );

        Self {
            cups: new_cups,
            active_cup_index: (self.active_cup_index + 1) % num_cups,
        }
    }
}

fn main() -> Result<(), util::Error> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let text = std::fs::read_to_string(filename)?;
    let mut cups = text.parse::<Cups>()?;

    for _ in 0..100 {
        cups = cups.next_iter();
    }

    println!("Part 1, cups = {:?}", cups);

    let mut cups = text.parse::<Cups>()?;
    cups.extend(1000000);

    let bar = ProgressBar::new(10000000);
    for _ in 0..10000000 {
        bar.inc(1);
        cups = cups.next_iter()
    }
    bar.finish();

    let pos_1 = cups.cups.iter().position(|c| *c == 1).unwrap();
    println!(
        "Part 2, prod = {}",
        cups.cups[pos_1 + 1] * cups.cups[pos_1 + 2]
    );

    Ok(())
}
