fn transform(num_loops: u64, subject_number: u64) -> u64 {
    let ring_size = 20201227u64;
    (0..num_loops).fold(1, |acc, _| (acc * subject_number) % ring_size)
}

fn find_num_loops(val: u64) -> u64 {
    let subject_number = 7u64;
    let ring_size = 20201227u64;
    (0..)
        .scan(1, |acc, _| {
            *acc = (*acc * subject_number) % ring_size;
            Some(*acc)
        })
        .enumerate()
        .map(|(iter, prod)| (1 + iter as u64, prod))
        .filter(|(_iter, prod)| *prod == val)
        .map(|(iter, _prod)| iter)
        .next()
        .unwrap()
}

fn main() -> Result<(), util::Error> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let text = std::fs::read_to_string(filename)?;
    let pub_keys = text
        .lines()
        .map(|line| line.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;

    let num_loops: Vec<_> =
        pub_keys.iter().map(|x| find_num_loops(*x)).collect();

    println!("pub keys = {:?}", pub_keys);
    println!("num_loops = {:?}", num_loops);

    println!("Key (0->1): {}", transform(num_loops[0], pub_keys[1]));
    println!("Key (1->0): {}", transform(num_loops[1], pub_keys[0]));

    Ok(())
}
