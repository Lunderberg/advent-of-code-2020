use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Rule {
    name: String,
    ranges: Vec<(i32, i32)>,
}

impl Rule {
    fn matches(&self, x: i32) -> bool {
        self.ranges
            .iter()
            .any(|&(low, high)| (low <= x) && (x <= high))
    }
}

#[derive(Debug)]
struct Ticket {
    values: Vec<i32>,
}

fn parse_text(text: &str) -> (Vec<Rule>, Vec<Ticket>) {
    let rule_regex = Regex::new(concat!(
        r"(?P<name>[a-z ]+)",
        r":\s+",
        r"(?P<n1>\d+)-(?P<n2>\d+)",
        r" or ",
        r"(?P<n3>\d+)-(?P<n4>\d+)",
    ))
    .unwrap();

    let rules: Vec<_> = rule_regex
        .captures_iter(text)
        .map(|cap| {
            let name = cap.name("name").unwrap().as_str().to_owned();
            let ranges = vec![
                (
                    cap.name("n1").unwrap().as_str().parse::<i32>().unwrap(),
                    cap.name("n2").unwrap().as_str().parse::<i32>().unwrap(),
                ),
                (
                    cap.name("n3").unwrap().as_str().parse::<i32>().unwrap(),
                    cap.name("n4").unwrap().as_str().parse::<i32>().unwrap(),
                ),
            ];

            Rule { name, ranges }
        })
        .collect();

    let ticket_regex = Regex::new(r"((\d+),)+\d+").unwrap();
    let tickets = ticket_regex
        .find_iter(text)
        .map(|mat| {
            let values: Vec<_> = mat
                .as_str()
                .split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            Ticket { values }
        })
        .collect();

    (rules, tickets)
}

fn main() -> Result<(), util::Error> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let text = std::fs::read_to_string(filename)?;
    let (rules, tickets) = parse_text(&text);

    let scanning_error_rate = tickets
        .iter()
        .map(|t| t.values.iter())
        .flatten()
        .filter(|&x| !rules.iter().any(|r| r.matches(*x)))
        .sum::<i32>();

    println!("Part a, error rate = {}", scanning_error_rate);

    let tickets: Vec<_> = tickets
        .iter()
        .filter(|&t| {
            !t.values
                .iter()
                .any(|&x| rules.iter().all(|r| !r.matches(x)))
        })
        .collect();

    let rule_map: Vec<_> = rules
        .iter()
        .cartesian_product(0usize..rules.len())
        .filter(|&(rule, index)| {
            tickets.iter().all(|t| rule.matches(t.values[index]))
        })
        .map(|(rule, index)| (rule.name.to_owned(), index))
        .collect::<Vec<_>>();

    let mut rule_possibilities: HashMap<String, HashSet<usize>> =
        HashMap::new();
    rule_map.iter().for_each(|(rule, index)| {
        rule_possibilities
            .entry(rule.to_string())
            .or_insert_with(HashSet::new)
            .insert(*index);
    });

    // If a rule is known, then it can't be used for any other fields.
    // Identify known fields, then remove them as possibilities from
    // other fields.
    loop {
        let known_indices: Vec<usize> = rule_possibilities
            .iter()
            .filter(|(_rule, indices)| indices.len() == 1)
            .map(|(_rule, indices)| *indices.iter().next().unwrap())
            .collect();

        let change_made = known_indices.iter().any(|known_index| {
            rule_possibilities.iter_mut().any(|(_rule, indices)| {
                if indices.len() > 1 {
                    indices.remove(known_index)
                } else {
                    false
                }
            })
        });

        if !change_made {
            break;
        }
    }

    println!("Rule map = {:?}", rule_possibilities);

    let departure_product = rule_possibilities
        .iter()
        .filter(|(rule, _indices)| rule.starts_with("departure"))
        .map(|(_rule, indices)| {
            tickets[0].values[*indices.iter().next().unwrap()] as i64
        })
        .product::<i64>();

    println!("Part b, departure product = {}", departure_product);

    Ok(())
}
