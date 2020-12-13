use std::fs::read_to_string;

#[derive(Debug)]
struct CustomsForm {
    answers: Vec<bool>,
}

#[derive(Debug)]
struct CustomsFormError;

impl std::str::FromStr for CustomsForm {
    type Err = CustomsFormError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut answers = vec![false; 26];
        s.chars().for_each(|c| {
            let index = ((c as u32) - ('a' as u32)) as usize;
            answers[index] = true;
        });
        Ok(CustomsForm { answers: answers })
    }
}

#[derive(Debug)]
struct CustomsGroup {
    forms: Vec<CustomsForm>,
}

impl std::str::FromStr for CustomsGroup {
    type Err = CustomsFormError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CustomsGroup {
            forms: s
                .split("\n")
                .filter(|line| line.len() > 0)
                .map(|line| line.parse::<CustomsForm>())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl CustomsGroup {
    fn any(&self) -> Vec<bool> {
        self.forms.iter().fold(vec![false; 26], |acc, val| {
            acc.iter()
                .zip(val.answers.iter())
                .map(|(a, b)| *a || *b)
                .collect()
        })
    }

    fn all(&self) -> Vec<bool> {
        self.forms.iter().fold(vec![true; 26], |acc, val| {
            acc.iter()
                .zip(val.answers.iter())
                .map(|(a, b)| *a && *b)
                .collect()
        })
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let contents = read_to_string(filename).unwrap();
    let groups: Vec<_> = contents
        .split("\n\n")
        .map(|section| section.parse::<CustomsGroup>())
        .collect::<Result<_, _>>()
        .unwrap();

    let sum: i32 = groups
        .iter()
        .map(|g| g.any().iter().map(|b| *b as i32).sum::<i32>())
        .sum();
    println!("Part a, sum = {}", sum);

    let sum: i32 = groups
        .iter()
        .map(|g| g.all())
        .map(|g| g.iter().map(|b| *b as i32).sum::<i32>())
        .sum();
    println!("Part b, sum = {}", sum);
}
