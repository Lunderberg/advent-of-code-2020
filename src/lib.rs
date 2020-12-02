use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::error::Error;

pub fn parse_file<T, E: 'static, P>
    (filename: P, func: fn(&String) -> Result<T, E>) -> Result<Vec<T>, Box<dyn Error>>
where P: AsRef<Path>,
      E: std::error::Error, {

    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();

    Ok(lines
       .collect::<Result<Vec<_>,_>>()?
       .iter()
       .map(func)
       .collect::<Result<Vec<_>,_>>()?
    )
}
