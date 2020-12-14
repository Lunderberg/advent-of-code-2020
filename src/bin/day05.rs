#[derive(Debug)]
struct BoardingPass {
    seat_id: u32,
}

#[derive(Debug)]
enum BoardingPassError {
    IncorrectChar,
}

impl std::str::FromStr for BoardingPass {
    type Err = BoardingPassError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BoardingPass {
            seat_id: s
                .chars()
                .map(|c| match c {
                    'F' => Ok(0),
                    'B' => Ok(1),
                    'L' => Ok(0),
                    'R' => Ok(1),
                    _ => Err(BoardingPassError::IncorrectChar),
                })
                .collect::<Result<Vec<_>, _>>()?
                .iter()
                .fold(0, |acc, x| 2 * acc + x),
        })
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let mut passes: Vec<_> = util::file_lines(filename)
        .unwrap()
        .map(|line| line.unwrap().parse::<BoardingPass>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    passes.sort_by_key(|p| p.seat_id);

    let max_val = passes.last().unwrap().seat_id;
    println!("Part 1, max boarding pass = {:?}", max_val);

    let missing_val = passes
        .as_slice()
        .windows(2)
        .filter(|s| s[0].seat_id + 1 != s[1].seat_id)
        .map(|s| BoardingPass {
            seat_id: s[0].seat_id + 1,
        })
        .next();
    println!("Part 2, missing boarding pass = {:?}", missing_val);
}
