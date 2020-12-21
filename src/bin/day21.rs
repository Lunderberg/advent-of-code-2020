use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug)]
struct Recipe {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl std::str::FromStr for Recipe {
    type Err = util::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sections = s.split(" (contains ");
        let ingredients: HashSet<_> = sections
            .next()
            .ok_or(util::Error::NoneError)?
            .split(" ")
            .map(|s| s.to_owned())
            .collect();

        let allergens: HashSet<_> = sections
            .next()
            .map(|s| s[..s.len() - 1].to_owned())
            .ok_or(util::Error::NoneError)?
            .split(", ")
            .map(|s| s.to_owned())
            .collect();

        Ok(Self {
            ingredients,
            allergens,
        })
    }
}

fn main() -> Result<(), util::Error> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let text = std::fs::read_to_string(filename)?;
    let recipes = text
        .lines()
        .map(|line| line.parse::<Recipe>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut possible_sources: HashMap<String, HashSet<String>> = HashMap::new();
    recipes.iter().for_each(|recipe| {
        recipe.allergens.iter().for_each(|allergen| {
            possible_sources
                .entry(allergen.to_string())
                .or_insert_with(|| {
                    recipe.ingredients.iter().map(|x| x.to_owned()).collect()
                })
                .retain(|ingredient| recipe.ingredients.contains(ingredient));
        });
    });

    loop {
        let known_sources: Vec<(String, String)> = possible_sources
            .iter()
            .filter(|(_allergen, sources)| sources.len() == 1)
            .map(|(allergen, sources)| {
                (
                    allergen.to_owned(),
                    sources.iter().next().unwrap().to_owned(),
                )
            })
            .collect::<Vec<_>>();

        let changes_made = known_sources.iter().any(|(allergen, source)| {
            possible_sources
                .iter_mut()
                .any(|(other_allergen, sources)| {
                    (allergen != other_allergen)
                        && sources.remove(&source.to_string())
                })
        });

        if !changes_made {
            break;
        }
    }

    let may_have_allergens = possible_sources
        .iter()
        .map(|(_, value)| value)
        .fold(HashSet::new(), |acc, set| {
            acc.union(set).map(|x| x.to_owned()).collect()
        });

    let safe_ingredient_usage = recipes
        .iter()
        .map(|r| {
            r.ingredients
                .iter()
                .filter(|&i| !may_have_allergens.contains(i))
                .count()
        })
        .sum::<usize>();

    println!("Part 1, safe ingredient usage = {}", safe_ingredient_usage);

    println!("Possible sources = {:?}", possible_sources);

    let canonically_dangerous = possible_sources
        .iter()
        .map(|(allergen, sources)| (allergen, sources.iter().next().unwrap()))
        .sorted_by_key(|(allergen, _source)| allergen.to_owned())
        .map(|(_allergen, source)| source)
        .join(",");

    println!("Canonically dangerous = {}", canonically_dangerous);

    Ok(())
}
