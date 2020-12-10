use std::collections::HashMap;

use util;

// fn count_instances<'a, I, T>(iter: I) -> HashMap<T,i32>
//     where I: Iterator<Item = &'a T>
// {
//     let mut output = HashMap::new();
//     iter.for_each(|item| {output.entry(item).or_default(0) += 1;});
//     output
// }

fn find_num_paths(joltages: &Vec<i32>) -> HashMap<i32,i64> {
    let mut paths = HashMap::new();
    paths.insert(0, 1);



    joltages
        .iter()
        .for_each(|val| {
            if *val > 0 {
                let num_paths_to = |val| *paths.get(&val).or(Some(&0)).unwrap();
                let num_new_paths =
                    num_paths_to(val-3) +
                    num_paths_to(val-2) +
                    num_paths_to(val-1);
                paths.insert(*val, num_new_paths);
            }
        });

    paths
}

fn main() -> Result<(), util::Error> {
    let args : Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let mut joltages =
        util::file_lines(filename).unwrap()
        .map(|line| -> Result<_,util::Error> {Ok(line?.parse::<i32>()?)} )
        .collect::<Result<Vec<_>,_>>()?;

    joltages.push(0); // Wall joltage
    let device_joltage = joltages.iter().max().unwrap() + 3;
    joltages.push(device_joltage); // Device adapter

    joltages.sort();

    let mut difference_counts = HashMap::new();
    joltages[..]
        .windows(2)
        .map(|slice| slice[1] - slice[0])
        .for_each(|diff| {
            *difference_counts.entry(diff).or_insert(0) += 1;
        })
        ;

    println!("1J diffs = {}, 3J diffs = {}, prod = {}",
             difference_counts[&1],  difference_counts[&3],
             difference_counts[&1] * difference_counts[&3]);

    let num_paths = find_num_paths(&joltages);
    println!("Num paths = {}", num_paths[&device_joltage]);


    Ok(())
}
