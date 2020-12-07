use std::collections::{HashMap,HashSet};
use std::convert::From;

use regex::Regex;

use util;


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct BagType {
    color: String,
}

#[derive(Debug)]
struct BagRule {
    container: BagType,
    contents: Vec<(i32, BagType)>,
}

impl std::str::FromStr for BagRule {
    type Err = util::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"^(?P<color>[a-z ]+?) bag").unwrap();
        let container_color =
            regex
            .captures(s)
            .ok_or(util::Error::MissingRegex)?
            .name("color")
            .unwrap()
            .as_str()
            .to_owned();

        let regex = Regex::new(r"(?P<num>[0-9]+) (?P<color>[a-z ]+?) bag")
            .unwrap();
        let contents =
            regex
            .captures_iter(s)
            .map(|cap| (cap
                        .name("num").unwrap()
                        .as_str()
                        .parse::<i32>().unwrap(),

                        BagType{color:
                                cap
                                .name("color").unwrap()
                                .as_str().to_owned()},
            ))
            .collect::<Vec<_>>()
            ;

        Ok(BagRule{container: BagType{color: container_color},
                   contents: contents})
    }
}

#[derive(Debug)]
struct BagGraph {
    contains: HashMap<BagType, Vec<(i32, BagType)>>,
    is_contained_by: HashMap<BagType, Vec<BagType>>,
}

impl From<Vec<BagRule>> for BagGraph {
    fn from(rules: Vec<BagRule>) -> Self {
        let mut is_contained_by = HashMap::<BagType,Vec<BagType>>::new();
        rules
            .iter()
            .map(|rule|
                 rule
                 .contents
                 .iter()
                 .map(|(_num,content_type)| (rule.container.to_owned(),
                                             content_type.to_owned()))
                 .collect::<Vec<_>>()
            )
            .flatten()
            .for_each(|(container_type, content_type)|
                      is_contained_by
                      .entry(content_type)
                      .or_insert(Vec::new())
                      .push(container_type)
            );

        
        let contains =
            rules
            .into_iter()
            .map(|rule| (rule.container, rule.contents))
            .collect::<HashMap<_,_>>()
            ;

        BagGraph{contains: contains,
                 is_contained_by: is_contained_by}
    }
}

impl BagGraph {
    fn indirectly_contains(&self, base: BagType) -> HashSet<BagType> {
        let mut output = HashSet::<BagType>::new();
        let mut unchecked = Vec::new();

        unchecked.push(base);

        while unchecked.len() > 0 {
            let color = unchecked.pop().unwrap();

            let new_colors = self.is_contained_by.get(&color);
            if let Some(new_colors) = new_colors {
                for new_color in new_colors {
                    unchecked.push(new_color.clone());
                }
            }
            output.insert(color);
        }

        output
    }

    fn num_contained(&self, base: &BagType) -> i32 {
        match self.contains.get(&base) {
            None => 0,
            Some(contents) => {
                contents
                    .iter()
                    .map(|(num,inner_bag)| num*(self.num_contained(inner_bag)+1))
                    .sum()
            }
        }
    }
}





fn main() -> Result<(), util::Error> {
    let args : Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let rules =
        util::file_lines(filename).unwrap()
        .map(|line| line?.parse::<BagRule>())
        .collect::<Result<Vec<_>,_>>()?;

    //println!("Rules = {:?}", rules);

    let graph = BagGraph::from(rules);

    //println!("Graph = {:?}", graph);

    let target = BagType{color: "shiny gold".to_string()};
    let indirectly_contains = graph.indirectly_contains(target.clone());

    println!("Bags that could hold shiny gold: {}", indirectly_contains.len()-1);
    println!("Minimum bags in shiny gold: {}", graph.num_contained(&target));

    Ok(())
}
