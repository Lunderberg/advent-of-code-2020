use std::collections::HashSet;
use std::convert::From;

#[derive(Debug, Clone)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl std::str::FromStr for Instruction {
    type Err = util::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let word_iter: Vec<_> = s.split(' ').collect();
        let command = word_iter[0];
        let argument = word_iter[1].parse::<i32>()?;

        match command {
            "acc" => Ok(Instruction::Acc(argument)),
            "jmp" => Ok(Instruction::Jmp(argument)),
            "nop" => Ok(Instruction::Nop(argument)),
            _ => Err(util::Error::InvalidValue(command.to_owned())),
        }
    }
}

#[derive(Debug)]
struct VirtualMachine {
    program: Vec<Instruction>,
    ip: i32,
    acc: i32,
}

impl From<Vec<Instruction>> for VirtualMachine {
    fn from(program: Vec<Instruction>) -> Self {
        Self {
            program,
            ip: 0,
            acc: 0,
        }
    }
}

impl VirtualMachine {
    fn iter(&mut self) {
        match self.program[self.ip as usize] {
            Instruction::Acc(val) => {
                self.acc += val;
                self.ip += 1;
            }
            Instruction::Jmp(val) => {
                self.ip += val;
            }
            Instruction::Nop(_) => {
                self.ip += 1;
            }
        }
    }

    fn terminated(&self) -> bool {
        (self.ip as usize) == self.program.len()
    }
}

#[derive(Debug)]
struct InfiniteLoop;

fn test_swap(
    mut program: Vec<Instruction>,
    loc: usize,
) -> Result<i32, InfiniteLoop> {
    match program[loc] {
        Instruction::Acc(_) => {}
        Instruction::Jmp(val) => program[loc] = Instruction::Nop(val),
        Instruction::Nop(val) => program[loc] = Instruction::Jmp(val),
    }

    let mut vm = VirtualMachine::from(program);
    let mut visited_instructions = HashSet::<i32>::new();
    loop {
        if visited_instructions.contains(&vm.ip) || vm.terminated() {
            break;
        }
        visited_instructions.insert(vm.ip);
        vm.iter();
    }

    if vm.terminated() {
        Ok(vm.acc)
    } else {
        Err(InfiniteLoop)
    }
}

fn main() -> Result<(), util::Error> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let program = util::file_lines(filename)
        .unwrap()
        .map(|line| line?.parse::<Instruction>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut vm = VirtualMachine::from(program.clone());

    let mut visited_instructions = HashSet::<i32>::new();
    while !visited_instructions.contains(&vm.ip) {
        visited_instructions.insert(vm.ip);
        vm.iter();
    }
    println!("First repeated instruction {} with acc = {}", vm.ip, vm.acc);

    let terminal_values = program
        .iter()
        .enumerate()
        .map(|(i, _instruction)| test_swap(program.clone(), i))
        .filter(|res| res.is_ok())
        .map(|res| res.unwrap())
        .collect::<Vec<_>>();
    println!("Terminal values = {:?}", terminal_values);

    Ok(())
}
