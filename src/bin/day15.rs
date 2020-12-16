use std::collections::HashMap;

#[derive(Debug)]
struct State {
    turn_num: i32,
    current_num: i32,
    when_last_used: HashMap<i32, i32>,
}

impl State {
    fn new() -> Self {
        Self {
            turn_num: 0,
            current_num: 0,
            when_last_used: HashMap::new(),
        }
    }

    fn add_num(&mut self, x: i32) {
        self.when_last_used.insert(self.current_num, self.turn_num);
        self.current_num = x;
        self.turn_num += 1;
    }

    fn next_num(&self) -> i32 {
        let prev_turn_used = self.when_last_used.get(&self.current_num);
        match prev_turn_used {
            Some(x) => self.turn_num - x,
            None => 0,
        }
    }

    fn iter(&mut self) {
        self.add_num(self.next_num());
    }

    fn loop_until(&mut self, last_iter: i32) {
        while self.turn_num < last_iter {
            self.iter();
        }
    }
}

fn main() -> Result<(), util::Error> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let starter = std::fs::read_to_string(filename)?
        .split(',')
        .map(|s| s.trim().parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut state = State::new();
    starter.iter().for_each(|&x| state.add_num(x));

    state.loop_until(2020);
    println!("Iter 2020 = {}", state.current_num);

    state.loop_until(30000000);
    println!("Iter 30000000 = {}", state.current_num);

    Ok(())
}
