use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| {
            let operation = &l[..3];
            let value = l[4..].parse().unwrap();
            match operation {
                "nop" => Instruction::NoOperation(value),
                "acc" => Instruction::Accumulate(value),
                "jmp" => Instruction::Jump(value),
                _ => unreachable!(),
            }
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(instructions: &[Instruction]) -> isize {
    match run_program(instructions) {
        RunResult::InfiniteLoop(acc) => acc,
        _ => unreachable!(),
    }
}

#[aoc(day8, part2)]
pub fn solve_part2(instructions: &[Instruction]) -> isize {
    let fix_locations: Vec<usize> = instructions
        .iter()
        .enumerate()
        .filter(|(_, ins)| matches!(ins, Instruction::Jump(_) | Instruction::NoOperation(_)))
        .map(|(i, _)| i)
        .collect();

    for fix in fix_locations {
        let mut program = instructions.to_vec();
        program[fix].flip();
        match run_program(&program) {
            RunResult::Finished(acc) => return acc,
            RunResult::InfiniteLoop(_) => {}
        }
    }
    unreachable!()
}

#[derive(Clone)]
pub enum Instruction {
    Jump(isize),
    Accumulate(isize),
    NoOperation(isize),
}

#[derive(Debug)]
enum RunResult {
    InfiniteLoop(isize),
    Finished(isize),
}

impl Instruction {
    fn execute(&self, acc: &mut isize, index: &mut isize) {
        match self {
            Instruction::Accumulate(value) => {
                *acc += value;
                *index += 1;
            }
            Instruction::Jump(value) => *index += value,
            Instruction::NoOperation(_) => *index += 1,
        }
    }

    fn flip(&mut self) {
        match self {
            Instruction::Jump(value) => *self = Instruction::NoOperation(*value),
            Instruction::NoOperation(value) => *self = Instruction::Jump(*value),
            _ => {}
        }
    }
}

fn run_program(program: &[Instruction]) -> RunResult {
    let mut visited: Vec<bool> = vec![false; program.len()];
    let mut acc = 0;
    let mut index = 0;
    loop {
        if index as usize == program.len() {
            return RunResult::Finished(acc);
        } else if visited[index as usize] {
            return RunResult::InfiniteLoop(acc);
        } else {
            visited[index as usize] = true;
            let instr = &program[index as usize];
            instr.execute(&mut acc, &mut index);
        }
    }
}

#[cfg(test)]
mod test {
    use itertools::join;

    use super::*;

    static EXAMPLE_1: &[&str] = &[
        "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4", "acc +6",
    ];

    #[test]
    fn solve_part1_test() {
        let input = input_generator(join(EXAMPLE_1, "\n").as_str());
        assert_eq!(solve_part1(&input), 5);
    }

    #[test]
    fn solve_part2_test() {
        let input = input_generator(join(EXAMPLE_1, "\n").as_str());
        assert_eq!(solve_part2(&input), 8);
    }
}
