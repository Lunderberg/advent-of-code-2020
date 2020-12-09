use std::collections::HashSet;
use itertools_num::ItertoolsNum;

use util;

fn last_is_sum_of_previous(seq: &[i64]) -> bool{
    let (last,elements) = seq.split_last().unwrap();
    let elements : HashSet<_> = elements.iter().collect();

    elements
        .iter()
        .any(|val| elements.contains(&(last - *val)))
}

fn contiguous_sum_indices(seq: &[i64], target_val: i64) -> Option<(usize,usize)> {
    let cumsum : Vec<i64> = seq
        .iter()
        .cumsum()
        .collect();

    cumsum
        .iter()
        .enumerate()
        .map(|(i,first)| {
            let target_last = first + target_val;
            let res = cumsum.binary_search(&target_last);
            match res {
                Ok(last_i) => Some( (i,last_i) ),
                Err(_) => None,
            }
        })
        .filter(|res| res.is_some())
        .map(|res| res.unwrap())
        .next()
}

fn first_out_of_sequence(sequence: &[i64], preamble_len: usize) -> Option<i64> {
    sequence
        .windows(preamble_len+1)
        .filter(|seq| !last_is_sum_of_previous(seq))
        .next()
        .map(|seq| *seq.last().unwrap())
}

fn main() -> Result<(), util::Error> {
    let args : Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let preamble_len = args[2].parse::<usize>()?;

    let sequence =
        util::file_lines(filename).unwrap()
        .map(|line| -> Result<_,util::Error> {Ok(line?.parse::<i64>()?)} )
        .collect::<Result<Vec<_>,_>>()?;

    let invalid_number = first_out_of_sequence(&sequence[..], preamble_len).unwrap();
    println!("Part a, first out of sequence: {:?}", invalid_number);

    let contiguous_range = contiguous_sum_indices(&sequence[..], invalid_number).unwrap();
    let (ia,ib) = contiguous_range;
    println!("Part b, continguous range {} - {}", sequence[ia], sequence[ib]);
    let rmin = sequence[ia..=ib].iter().min().unwrap();
    let rmax = sequence[ia..=ib].iter().max().unwrap();
    println!("Part b, max={}, min={}, total={}", rmin, rmax, rmin+rmax);



    Ok(())
}
