use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug)]
struct Passport {
    fields: HashMap<String, String>,
}

#[derive(Debug)]
enum PassportParseError {
    IoError(std::io::Error),
}

impl From<std::io::Error> for PassportParseError {
    fn from(e: std::io::Error) -> Self {
        PassportParseError::IoError(e)
    }
}

fn validate_field(key: &str, val: &str) -> bool {
    match key {
        "byr" => validate_byr(val),
        "iyr" => validate_iyr(val),
        "eyr" => validate_eyr(val),
        "hgt" => validate_hgt(val),
        "hcl" => validate_hcl(val),
        "ecl" => validate_ecl(val),
        "pid" => validate_pid(val),
        "cid" => true,
        _ => false,
    }
}

fn validate_byr(val: &str) -> bool {
    let val = val.parse::<i32>();
    match val {
        Ok(year) => ((year >= 1920) && (year <= 2002)),
        _ => false,
    }
}

fn validate_iyr(val: &str) -> bool {
    let val = val.parse::<i32>();
    match val {
        Ok(year) => ((year >= 2010) && (year <= 2020)),
        _ => false,
    }
}
fn validate_eyr(val: &str) -> bool {
    let val = val.parse::<i32>();
    match val {
        Ok(year) => ((year >= 2020) && (year <= 2030)),
        _ => false,
    }
}
fn validate_hgt(val: &str) -> bool {
    let reg = Regex::new(r"^(?P<val>[0-9]+)(?P<unit>in|cm)$").unwrap();
    let captures = reg.captures(val);
    if let Some(captures) = captures {
        let val = captures
            .name("val")
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        let unit = captures.name("unit").unwrap().as_str();

        match unit {
            "in" => ((val >= 59) && (val <= 76)),
            "cm" => ((val >= 150) && (val <= 193)),
            _ => false,
        }
    } else {
        false
    }
}
fn validate_hcl(val: &str) -> bool {
    let reg = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    reg.is_match(val)
}
fn validate_ecl(val: &str) -> bool {
    match val {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false,
    }
}
fn validate_pid(val: &str) -> bool {
    let reg = Regex::new(r"^[0-9]{9}$").unwrap();
    reg.is_match(val)
}

impl Passport {
    fn parse(filename: &str) -> Result<Vec<Passport>, PassportParseError> {
        let contents = read_to_string(filename)?;
        let reg = Regex::new(r"(?P<key>[a-z]+):(?P<val>[a-z0-9#]+)").unwrap();
        Ok(contents
            .split("\n\n")
            .map(|section| Passport {
                fields: reg
                    .captures_iter(section)
                    .map(|cap| {
                        (
                            cap.name("key").unwrap().as_str().to_string(),
                            cap.name("val").unwrap().as_str().to_string(),
                        )
                    })
                    .collect::<HashMap<_, _>>(),
            })
            .collect())
    }

    fn has_required_fields(&self) -> bool {
        let required_fields = vec![
            "byr", "iyr", "eyr", "hgt", "hcl", "ecl",
            "pid",
            //"cid", // The country ID, missing for North Pole
        ];
        required_fields.iter().all(|v| self.fields.contains_key(*v))
    }

    fn is_valid(&self) -> bool {
        let required_fields = vec![
            "byr", "iyr", "eyr", "hgt", "hcl", "ecl",
            "pid",
            //"cid", // The country ID, missing for North Pole
        ];
        required_fields
            .iter()
            .all(|v| self.fields.contains_key(*v) && validate_field(v, &self.fields[*v]))
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let passports = Passport::parse(filename).unwrap();
    let num_valid = passports.iter().filter(|p| p.has_required_fields()).count();
    println!("Part 1, num valid = {}", num_valid);

    let num_valid = passports.iter().filter(|p| p.is_valid()).count();
    println!("Part 2, num valid = {}", num_valid);
}
