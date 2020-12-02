use std::error::Error;

use regex::Regex;

use util;

#[derive(Debug)]
struct Password {
    lower_bound: i32,
    upper_bound: i32,
    check_letter: char,
    password: String,
}

#[derive(Debug)]
struct PasswordParseError;

impl std::fmt::Display for PasswordParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Could not parse line")
    }
}

impl std::error::Error for PasswordParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

impl From<std::num::ParseIntError> for PasswordParseError {
    fn from(_ : std::num::ParseIntError) -> Self {
        PasswordParseError
    }
}

impl std::str::FromStr for Password {
    type Err = PasswordParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reg = Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<char>[a-z]): (?P<password>[a-z]+)$").unwrap();

        let captures = reg.captures(s).ok_or(PasswordParseError)?;

        let lower_bound = captures
            .name("min")
            .ok_or(PasswordParseError)?
            .as_str()
            .parse::<i32>()?
            ;
        let upper_bound = captures
            .name("max")
            .ok_or(PasswordParseError)?
            .as_str()
            .parse::<i32>()?
            ;
        let check_letter = captures
            .name("char")
            .ok_or(PasswordParseError)?
            .as_str()
            .chars().next()
            .ok_or(PasswordParseError)?
            ;
        let password = captures
            .name("password")
            .ok_or(PasswordParseError)?
            .as_str()
            ;


        Ok(Password{lower_bound: lower_bound,
                 upper_bound: upper_bound,
                 check_letter: check_letter,
                 password: password.to_string(),
        })
    }
}

impl Password {
    fn check_validity_v1(&self) -> bool {
        let num_char = self.password
            .chars()
            .filter(|c| *c==self.check_letter)
            .count() as i32
            ;

        (num_char >= self.lower_bound) && (num_char <= self.upper_bound)
    }

    fn check_validity_v2(&self) -> bool {
        let bytes = self.password.as_bytes();
        let char1 = bytes[(self.lower_bound - 1) as usize] as char;
        let char2 = bytes[(self.upper_bound - 1) as usize] as char;
        (char1==self.check_letter) ^ (char2==self.check_letter)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args : Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let values = util::parse_file(filename, |line| line.parse::<Password>())?;

    let num_valid = values
        .iter()
        .filter(|p| p.check_validity_v1())
        .count();

    println!("Part 1: {}/{} valid", num_valid, values.len());

    let num_valid = values
        .iter()
        .filter(|p| p.check_validity_v2())
        .count();

    println!("Part 2: {}/{} valid", num_valid, values.len());

    Ok(())
}
