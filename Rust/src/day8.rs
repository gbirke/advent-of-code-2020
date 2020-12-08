use std::collections::HashSet;

#[derive(Eq, PartialEq, Debug)]
pub enum Instruction {
    Noop,
    Acc(i32),
    Jump(i32),
}

#[aoc_generator(day8)]
pub fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(" ");
            match parts.next() {
                Some("nop") => Instruction::Noop,
                Some("acc") => Instruction::Acc(parts.next().unwrap().parse::<i32>().unwrap()),
                Some("jmp") => Instruction::Jump(parts.next().unwrap().parse::<i32>().unwrap()),
                _ => panic!("Unknown instruction"),
            }
        })
        .collect()
}

#[derive(Eq, PartialEq, Debug)]
enum RuntimeError {
    IllegalJump(usize),
    LoopDetected(usize, usize, i32),
}

type RuntimeResult = Result<i32, RuntimeError>;

struct VirtualMachine {
    instruction_counter: usize,
    accumulator: i32,
}

impl VirtualMachine {
    fn new() -> VirtualMachine {
        VirtualMachine {
            instruction_counter: 0,
            accumulator: 0,
        }
    }

    fn run_with_loop_detection(&mut self, instructions: &Vec<Instruction>) -> RuntimeResult {
        let mut executed_instructions: HashSet<usize> = HashSet::new();
        let mut previous_instruction: usize = 0;
        loop {
            if executed_instructions.contains(&self.instruction_counter) {
                return Err(RuntimeError::LoopDetected(
                    previous_instruction,
                    self.instruction_counter,
                    self.accumulator,
                ));
            }
            previous_instruction = self.instruction_counter.clone();
            executed_instructions.insert(previous_instruction);
            match instructions[self.instruction_counter] {
                Instruction::Noop => self.instruction_counter += 1,
                Instruction::Acc(acc) => {
                    self.accumulator += acc;
                    self.instruction_counter += 1;
                }
                Instruction::Jump(count) => {
                    let mut current: i32 = self.instruction_counter as i32;
                    if current + count < 0 {
                        return Err(RuntimeError::IllegalJump(self.instruction_counter));
                    }
                    current += count;
                    self.instruction_counter = current as usize;
                }
            }
            if self.instruction_counter > instructions.len() {
                return Ok(self.accumulator);
            }
        }
    }
}

#[aoc(day8, part1)]
pub fn part_one(instructions: &Vec<Instruction>) -> i32 {
    let mut vm = VirtualMachine::new();
    match vm.run_with_loop_detection(&instructions) {
        Err(RuntimeError::LoopDetected(_, _, acc)) => acc,
        result => panic!("Unexpected runtime result: {:?}", result),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_parses_instructions() {
        let input = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";
        let expected = vec![
            Instruction::Noop,
            Instruction::Acc(1),
            Instruction::Jump(4),
            Instruction::Acc(3),
            Instruction::Jump(-3),
            Instruction::Acc(-99),
            Instruction::Acc(1),
            Instruction::Jump(-4),
            Instruction::Acc(6),
        ];

        assert_eq!(parse_input(input), expected);
    }

    fn it_runs_until_loop_detected() {
        let mut vm = VirtualMachine::new();
        let instructions = vec![
            Instruction::Noop,
            Instruction::Acc(1),
            Instruction::Jump(4),
            Instruction::Acc(3),
            Instruction::Jump(-3),
            Instruction::Acc(-99),
            Instruction::Acc(1),
            Instruction::Jump(-4),
            Instruction::Acc(6),
        ];

        assert_eq!(&vm.run_with_loop_detection(&instructions), &5)
    }
}
