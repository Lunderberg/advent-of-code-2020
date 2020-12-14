use std::fs::File;
use std::io::{self, BufRead, Lines};
use std::path::Path;

pub fn parse_file<T, E: 'static, P>(
    filename: P,
    func: fn(&String) -> Result<T, E>,
) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
    E: std::error::Error,
{
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();

    Ok(lines
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .map(func)
        .collect::<Result<Vec<_>, _>>()?)
}

pub fn file_lines<P>(
    filename: P,
) -> Result<Lines<io::BufReader<File>>, io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[derive(Debug)]
pub enum Error {
    WrongInt(std::num::ParseIntError),
    IoError(std::io::Error),
    MissingRegex,
    InvalidValue(String),
    NoneError,
    EarlyFailure,
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::WrongInt(e)
    }
}

// Once the try_trait has been stabilized
// impl From<core::option::NoneError> for Error {
//     fn from(e: core::option::NoneError) -> Self {
//         Error::NoneError
//     }
// }
