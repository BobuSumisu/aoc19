use std::convert::TryFrom;
use std::io::{self, Read};

const RESULT_INDEX: usize = 0;
const NOUN_INDEX: usize = 1;
const VERB_INDEX: usize = 2;

enum IntcodeInstruction {
    Add,
    Multiply,
    Halt,
}

impl TryFrom<u64> for IntcodeInstruction {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(IntcodeInstruction::Add),
            2 => Ok(IntcodeInstruction::Multiply),
            99 => Ok(IntcodeInstruction::Halt),
            _ => Err("illegal opcode"),
        }
    }
}

struct IntcodeProgram {
    memory: Vec<u64>,
    ip: usize,
}

impl IntcodeProgram {
    fn new(memory: Vec<u64>) -> IntcodeProgram {
        IntcodeProgram { memory, ip: 0 }
    }

    fn find_noun_and_verb(memory: Vec<u64>, target: u64) -> Option<(u64, u64)> {
        for noun in 0..100 {
            for verb in 0..100 {
                let mut program = IntcodeProgram::new(memory.clone());
                program.set_noun_and_verb(noun, verb);
                if program.run() == target {
                    return Some((noun, verb));
                }
            }
        }
        None
    }

    fn run(&mut self) -> u64 {
        loop {
            let opcode = self.memory[self.ip];
            let instruction = IntcodeInstruction::try_from(opcode).unwrap();

            match instruction {
                IntcodeInstruction::Add => {
                    let indices = self.get_indices();
                    let values = self.get_values(indices);
                    self.memory[indices.2] = values.0 + values.1;
                }
                IntcodeInstruction::Multiply => {
                    let indices = self.get_indices();
                    let values = self.get_values(indices);
                    self.memory[indices.2] = values.0 * values.1;
                }
                IntcodeInstruction::Halt => break,
            }

            self.ip += 4;
        }

        self.memory[RESULT_INDEX]
    }

    fn get_indices(&self) -> (usize, usize, usize) {
        (
            self.memory[self.ip + 1] as usize,
            self.memory[self.ip + 2] as usize,
            self.memory[self.ip + 3] as usize,
        )
    }

    fn get_values(&self, indices: (usize, usize, usize)) -> (u64, u64, u64) {
        (
            self.memory[indices.0],
            self.memory[indices.1],
            self.memory[indices.2],
        )
    }

    fn set_noun_and_verb(&mut self, noun: u64, verb: u64) {
        self.memory[NOUN_INDEX] = noun;
        self.memory[VERB_INDEX] = verb;
    }

    fn get_noun_and_verb(&self) -> (u64, u64) {
        (self.memory[NOUN_INDEX], self.memory[VERB_INDEX])
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let code: Vec<u64> = input
        .split(',')
        .filter_map(|num| num.parse().ok())
        .collect();

    let mut program = IntcodeProgram::new(code.clone());
    program.set_noun_and_verb(12, 2);
    let result = program.run();
    println!("Part 1: {}", result);

    let (noun, verb) = IntcodeProgram::find_noun_and_verb(code, 19690720).unwrap();
    println!("Part 2: {}", 100 * noun + verb);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_examples_part1() {
        assert_eq!(2, IntcodeProgram::new(vec![1, 0, 0, 0, 99]).run());
        assert_eq!(2, IntcodeProgram::new(vec![2, 3, 0, 3, 99]).run());
        assert_eq!(2, IntcodeProgram::new(vec![2, 4, 4, 5, 99, 0]).run());
        assert_eq!(
            30,
            IntcodeProgram::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]).run()
        );
        assert_eq!(
            3500,
            IntcodeProgram::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]).run()
        );
    }

    #[test]
    fn test_examples_part2() {
        let memory: Vec<u64> = fs::read_to_string("input/input.txt")
            .unwrap()
            .split(',')
            .filter_map(|v| v.parse().ok())
            .collect();
        let mut program = IntcodeProgram::new(memory);
        program.set_noun_and_verb(12, 2);
        program.run();
        let (noun, verb) = program.get_noun_and_verb();
        assert_eq!(1202, 100 * noun + verb);
    }

    #[test]
    fn test_part1() {
        let memory: Vec<u64> = fs::read_to_string("input/input.txt")
            .unwrap()
            .split(',')
            .filter_map(|v| v.parse().ok())
            .collect();
        let mut program = IntcodeProgram::new(memory);
        program.set_noun_and_verb(12, 2);
        let value = program.run();
        assert_eq!(3654868, value);
    }

    #[test]
    fn test_part2() {
        let memory: Vec<u64> = fs::read_to_string("input/input.txt")
            .unwrap()
            .split(',')
            .filter_map(|v| v.parse().ok())
            .collect();

        let (noun, verb) = IntcodeProgram::find_noun_and_verb(memory, 19690720).unwrap();
        assert_eq!(7014, 100 * noun + verb);
    }
}
