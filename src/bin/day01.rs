use std::error::Error;

use itertools::Itertools;

use util;

fn main() -> Result<(), Box<dyn Error>> {
    let args : Vec<String> = std::env::args().collect();
    let filename = &args[args.len()-1];

    let values = util::parse_file(filename, |line| line.parse::<i32>())?;

    for num_elements in 2..=3 {
        values
            .iter()
            .combinations(num_elements)
            .filter(|c| c.iter().map(|x| *x).sum::<i32>()==2020)
            .map(|c| println!("{:?}, prod={}", c, c.iter().map(|x| *x).product::<i32>()) )
            .last()
            ;
    }

    Ok(())
}
