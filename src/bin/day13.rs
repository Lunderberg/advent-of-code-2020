use util;

fn part_1(lines: &Vec<String>) -> Result<(), util::Error> {
    let start_time = lines[0].parse::<i64>()?;
    let bus_times = lines[1]
        .split(',')
        .map(|s| s.parse::<i64>())
        .filter(|x| x.is_ok())
        .collect::<Result<Vec<_>, _>>()?;

    let next_arrival_time = |period: i64| {
        let num_periods_passed = ((start_time as f64) / (period as f64)).ceil() as i64;
        period * num_periods_passed
    };

    let next_bus = bus_times
        .iter()
        .min_by_key(|x| next_arrival_time(**x))
        .unwrap();
    let wait_time = next_arrival_time(*next_bus) - start_time;

    println!(
        "Next bus = {}, wait time = {}, Part a = {}",
        next_bus,
        wait_time,
        next_bus * wait_time
    );

    Ok(())
}

fn part_2(lines: &Vec<String>) -> Result<(), util::Error> {
    let bus_offsets: Vec<_> = lines[1]
        .split(',')
        .enumerate()
        .filter(|(_i, s)| *s != "x")
        .map(|(i, s)| (i as i64, s.parse::<i64>().unwrap()))
        .collect();

    let required_offset =
        bus_offsets
            .iter()
            .fold((0, 1), |(prev_offset, prev_period), (offset, period)| {
                let offset = *offset;
                let period = *period;

                let additional_cycles = (0..period)
                    .filter(|p| (prev_offset + p * prev_period + offset) % period == 0)
                    .next()
                    .unwrap();

                (
                    additional_cycles * prev_period + prev_offset,
                    period * prev_period,
                )
            });

    println!("Part b, moderate force = {}", required_offset.0);

    Ok(())
}

#[allow(dead_code)]
fn part_2_brute_force(lines: &Vec<String>) -> Result<(), util::Error> {
    let bus_offsets: Vec<_> = lines[1]
        .split(',')
        .enumerate()
        .filter(|(_i, s)| *s != "x")
        .map(|(i, s)| (i as i64, s.parse::<i64>().unwrap()))
        .collect();

    let first_time = (1..)
        .filter(|x| {
            bus_offsets
                .iter()
                .all(|(offset, period)| (x + offset) % period == 0)
        })
        .next()
        .unwrap();

    println!("Part b, brute force = {}", first_time);

    Ok(())
}

fn main() -> Result<(), util::Error> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let lines: Vec<String> = std::fs::read_to_string(filename)?
        .lines()
        .map(|s| s.to_owned())
        .collect();

    part_1(&lines)?;
    part_2(&lines)?;
    //part_2_brute_force(&lines)?;

    Ok(())
}
