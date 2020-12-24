use std::collections::HashMap;

#[allow(unused_imports)]
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Debug, Clone)]
struct Cups {
    next_cup: HashMap<i64, i64>,
    active_cup: i64,
}

impl Cups {
    fn new(s: &str, min_size: i64) -> Result<Self, util::Error> {
        let cup_order = s
            .trim()
            .chars()
            .map(|c| c.to_string().parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;

        let mut next_cup: HashMap<i64, i64> = HashMap::new();

        cup_order[..].windows(2).for_each(|slice| {
            next_cup.insert(slice[0], slice[1]);
        });

        let mut prev_cup = cup_order[cup_order.len() - 1];

        while (next_cup.len() as i64) < min_size - 1 {
            let new_val = (next_cup.len() as i64) + 2;
            next_cup.insert(prev_cup, new_val);
            prev_cup = new_val;
        }

        next_cup.insert(prev_cup, cup_order[0]);

        Ok(Self {
            next_cup,
            active_cup: cup_order[0],
        })
    }

    fn iter(&mut self) {
        let pick_up: Vec<_> = (0..3)
            .scan(self.active_cup, |acc, _| {
                *acc = self.next_cup[acc];
                Some(*acc)
            })
            .collect();

        let mut destination = self.active_cup;
        loop {
            destination -= 1;
            if destination == 0 {
                destination = self.next_cup.len() as i64;
            }
            if !pick_up.contains(&destination) {
                break;
            }
        }

        let after_destination = self.next_cup[&destination];
        let after_pick_up = self.next_cup[&pick_up[pick_up.len() - 1]];
        self.next_cup.insert(destination, pick_up[0]);
        self.next_cup
            .insert(pick_up[pick_up.len() - 1], after_destination);
        self.next_cup.insert(self.active_cup, after_pick_up);

        self.active_cup = self.next_cup[&self.active_cup];
    }

    fn next_iter(&self) -> Self {
        let mut next = self.clone();
        next.iter();
        next
    }
}

fn main() -> Result<(), util::Error> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let text = std::fs::read_to_string(filename)?;
    let mut cups = Cups::new(&text, 0)?;

    println!("cups = {:?}", cups);

    for _ in 0..100 {
        cups.iter();
    }

    println!(
        "Part 1, cups = {:?}",
        (1..10)
            .scan(1, |acc, _| {
                *acc = cups.next_cup[acc];
                Some(*acc)
            })
            .collect::<Vec<_>>()
    );

    let mut cups = Cups::new(&text, 1000000)?;

    let bar = ProgressBar::new(10000000);
    bar.set_style(
        ProgressStyle::default_bar().template(
            "{wide_bar} Elapsed: {elapsed_precise}, ETA: {eta_precise}",
        ),
    );
    for _ in 0..10000000 {
        bar.inc(1);
        cups.iter();
    }
    bar.finish();

    let after_1 = cups.next_cup[&1];
    let after_2 = cups.next_cup[&after_1];
    println!("Part 2, prod = {}", after_1 * after_2);

    Ok(())
}
