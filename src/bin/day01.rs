use std::error::Error;

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[args.len() - 1];

    let values = util::parse_file(filename, |line| line.parse::<i32>())?;

    for num_elements in 2..=3 {
        values
            .iter()
            .combinations(num_elements)
            .filter(|c| c.iter().copied().sum::<i32>() == 2020)
            .map(|c| {
                println!("{:?}, prod={}", c, c.iter().copied().product::<i32>())
            })
            .last();
    }

    Ok(())
}
