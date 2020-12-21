use std::collections::HashMap;

#[derive(Debug)]
enum MatchRule {
    SingleChar(char),
    MatchOthers(Vec<Vec<usize>>),
}

impl std::str::FromStr for MatchRule {
    type Err = util::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('"') {
            Ok(MatchRule::SingleChar(s.chars().nth(1).unwrap()))
        } else {
            Ok(MatchRule::MatchOthers(
                s.split(" | ")
                    .map(|subrule| {
                        subrule
                            .split(' ')
                            .map(|n| n.parse::<usize>())
                            .collect::<Result<Vec<_>, _>>()
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            ))
        }
    }
}

#[derive(Debug)]
struct MatchRules {
    rules: HashMap<usize, MatchRule>,
}

impl MatchRules {
    fn parse(s: &str) -> Self {
        let rules = s
            .lines()
            .filter(|line| line.contains(": "))
            .map::<Result<_, util::Error>, _>(|line| {
                let mut line = line.split(": ");
                Ok((
                    line.next()
                        .ok_or(util::Error::NoneError)?
                        .parse::<usize>()?,
                    line.next()
                        .ok_or(util::Error::NoneError)?
                        .parse::<MatchRule>()?,
                ))
            })
            .filter(|rule| rule.is_ok())
            .map(|rule| rule.unwrap())
            .collect::<HashMap<_, _>>();
        MatchRules { rules }
    }

    fn num_chars_matched(&self, i: usize, s: &str) -> Vec<usize> {
        let rule = &self.rules[&i];

        match rule {
            MatchRule::SingleChar(c) => {
                if s.starts_with(*c) {
                    vec![1]
                } else {
                    Vec::new()
                }
            }
            MatchRule::MatchOthers(options) => options
                .iter()
                .map(|subrules| {
                    subrules.iter().fold::<Vec<usize>, _>(
                        vec![0],
                        |acc, subrule| {
                            acc.iter()
                                .map(|prev_chars| {
                                    self.num_chars_matched(
                                        *subrule,
                                        &s[*prev_chars..],
                                    )
                                    .iter()
                                    .map(|new_chars| prev_chars + new_chars)
                                    .collect::<Vec<usize>>()
                                })
                                .flatten()
                                .collect()
                        },
                    )
                })
                .flatten()
                .collect(),
        }
    }

    fn matches(&self, i: usize, s: &str) -> bool {
        self.num_chars_matched(i, s)
            .iter()
            .any(|&num_chars| num_chars == s.len())
    }
}

fn main() -> Result<(), util::Error> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let text = std::fs::read_to_string(filename)?;
    let mut sections = text.split("\n\n");
    let rules =
        MatchRules::parse(sections.next().ok_or(util::Error::NoneError)?);

    let messages = sections
        .next()
        .ok_or(util::Error::NoneError)?
        .lines()
        .collect::<Vec<_>>();

    let num_matches = messages
        .iter()
        .filter(|message| rules.matches(0, message))
        .count();
    println!("Part 1, num matches = {}", num_matches);

    let text = text
        .replace("8: 42", "8: 42 | 42 8")
        .replace("11: 42 31", "11: 42 31 | 42 11 31");

    let mut sections = text.split("\n\n");
    let rules =
        MatchRules::parse(sections.next().ok_or(util::Error::NoneError)?);

    let num_matches = messages
        .iter()
        .filter(|message| rules.matches(0, message))
        .count();
    println!("Part 2, num matches = {}", num_matches);

    Ok(())
}
