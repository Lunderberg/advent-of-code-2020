use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::error::Error;

use itertools::Itertools;

#[allow(dead_code)]
fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

//type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
//type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn parse_file(filename: &String) -> Result<Vec<i32>, Box<dyn Error>> {
    let lines = read_lines(filename)?;

    Ok(lines
       .map(|line| line.unwrap().parse::<i32>())
       .collect::<Result<Vec<_>,_>>()?
    )
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args : Vec<String> = std::env::args().collect();
    let filename = &args[args.len()-1];

    let values = parse_file(filename)?;

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
