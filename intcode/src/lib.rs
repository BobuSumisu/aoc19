use std::convert::{TryFrom, TryInto};
use std::fs;

enum Op {
    Add,
    Multiply,
    Read,
    Write,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    AdjustBase,
    Halt,
}

impl TryFrom<i64> for Op {
    type Error = String;

    fn try_from(n: i64) -> Result<Self, Self::Error> {
        match n {
            1 => Ok(Self::Add),
            2 => Ok(Self::Multiply),
            3 => Ok(Self::Read),
            4 => Ok(Self::Write),
            5 => Ok(Self::JumpIfTrue),
            6 => Ok(Self::JumpIfFalse),
            7 => Ok(Self::LessThan),
            8 => Ok(Self::Equals),
            9 => Ok(Self::AdjustBase),
            99 => Ok(Self::Halt),
            _ => Err(format!("illegal operation code: {}", n)),
        }
    }
}

#[derive(Clone, Copy)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

impl TryFrom<i64> for Mode {
    type Error = String;

    fn try_from(n: i64) -> Result<Self, Self::Error> {
        match n {
            0 => Ok(Self::Position),
            1 => Ok(Self::Immediate),
            2 => Ok(Self::Relative),
            _ => Err(format!("illegal argument mode: {}", n)),
        }
    }
}

struct Instruction {
    op: Op,
    modes: [Mode; 3],
}

impl TryFrom<i64> for Instruction {
    type Error = String;

    fn try_from(n: i64) -> Result<Self, Self::Error> {
        let op = (n % 100).try_into()?;
        let mut n = n / 100;
        let mut modes = [Mode::Position; 3];

        for i in 0..3 {
            modes[i] = (n % 10).try_into()?;
            n /= 10;
        }

        Ok(Self { op, modes })
    }
}

#[derive(Default)]
pub struct Computer {
    memory: Vec<i64>,
    inputs: Vec<i64>,
    outputs: Vec<i64>,
    ip: i64,
    halted: bool,
    base: i64,
}

impl Computer {
    pub fn new(intcode: &[i64], inputs: &[i64]) -> Self {
        Self {
            memory: intcode.to_vec(),
            inputs: inputs.to_vec(),
            ..Self::default()
        }
    }

    pub const fn is_halted(&self) -> bool {
        self.halted
    }

    pub fn push_input(&mut self, input: i64) {
        self.inputs.push(input);
    }

    pub fn memory(&self) -> &[i64] {
        &self.memory
    }

    pub fn outputs(&self) -> &[i64] {
        &self.outputs
    }

    pub fn last_output(&self) -> Option<i64> {
        self.outputs.last().cloned()
    }

    pub fn patch(&mut self, patch: (i64, i64)) {
        self.memory[1] = patch.0;
        self.memory[2] = patch.1;
    }

    pub fn get_patch(&self) -> (i64, i64) {
        (self.memory[1], self.memory[2])
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while !self.halted {
            self.step()?;
        }
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let instr = Instruction::try_from(self.memory[usize::try_from(self.ip)?])?;
            match instr.op {
                Op::Add => {
                    let x = self.arg(1, instr.modes[0]);
                    let y = self.arg(2, instr.modes[1]);
                    self.put(3, x + y, instr.modes[2]);
                    self.ip += 4;
                }
                Op::Multiply => {
                    let x = self.arg(1, instr.modes[0]);
                    let y = self.arg(2, instr.modes[1]);
                    self.put(3, x * y, instr.modes[2]);
                    self.ip += 4;
                }
                Op::Read => {
                    let input = self.inputs.remove(0);
                    self.put(1, input, instr.modes[0]);
                    self.ip += 2;
                }
                Op::Write => {
                    let x = self.arg(1, instr.modes[0]);
                    self.outputs.push(x);
                    self.ip += 2;
                    break;
                }
                Op::JumpIfTrue => {
                    if self.arg(1, instr.modes[0]) == 0 {
                        self.ip += 3;
                    } else {
                        self.ip = self.arg(2, instr.modes[1]);
                    }
                }
                Op::JumpIfFalse => {
                    if self.arg(1, instr.modes[0]) == 0 {
                        self.ip = self.arg(2, instr.modes[1]);
                    } else {
                        self.ip += 3;
                    }
                }
                Op::LessThan => {
                    if self.arg(1, instr.modes[0]) < self.arg(2, instr.modes[1]) {
                        self.put(3, 1, instr.modes[2]);
                    } else {
                        self.put(3, 0, instr.modes[2]);
                    }
                    self.ip += 4;
                }
                Op::Equals => {
                    if self.arg(1, instr.modes[0]) == self.arg(2, instr.modes[1]) {
                        self.put(3, 1, instr.modes[2]);
                    } else {
                        self.put(3, 0, instr.modes[2]);
                    }
                    self.ip += 4;
                }
                Op::AdjustBase => {
                    self.base += self.arg(1, instr.modes[0]);
                    self.ip += 2;
                }
                Op::Halt => {
                    self.halted = true;
                    break;
                }
            }
        }

        Ok(())
    }

    fn arg(&mut self, arg_index: i64, mode: Mode) -> i64 {
        let value = self.memory_get(self.ip + arg_index);
        match mode {
            Mode::Position => self.memory_get(value),
            Mode::Immediate => value,
            Mode::Relative => self.memory_get(self.base + value),
        }
    }

    fn put(&mut self, index: i64, value: i64, mode: Mode) {
        let dest = self.memory_get(self.ip + index);
        match mode {
            Mode::Position => self.memory_set(dest, value),
            Mode::Immediate => panic!("immediate mode illegal for storing values"),
            Mode::Relative => self.memory_set(self.base + dest, value),
        };
    }

    fn memory_get(&mut self, index: i64) -> i64 {
        let index: usize = index.try_into().unwrap();
        if self.memory.len() < index + 1 {
            self.memory.resize(index + 1, 0);
        }
        self.memory[index]
    }

    fn memory_set(&mut self, index: i64, value: i64) {
        let index: usize = index.try_into().unwrap();
        if self.memory.len() < index + 1 {
            self.memory.resize(index + 1, 0);
        }
        self.memory[index] = value;
    }
}

pub fn run_intcode(intcode: &[i64], inputs: &[i64]) -> Computer {
    let mut computer = Computer::new(intcode, inputs);
    computer.run().unwrap();
    computer
}

pub fn load_intcode(path: &str) -> Vec<i64> {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day05() {
        let intcode = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(run_intcode(&intcode, &[7]).last_output(), Some(0));
        assert_eq!(run_intcode(&intcode, &[8]).last_output(), Some(1));

        let intcode = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(run_intcode(&intcode, &[7]).last_output(), Some(1));
        assert_eq!(run_intcode(&intcode, &[9]).last_output(), Some(0));

        let intcode = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        assert_eq!(run_intcode(&intcode, &[7]).last_output(), Some(0));
        assert_eq!(run_intcode(&intcode, &[8]).last_output(), Some(1));

        let intcode = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        assert_eq!(run_intcode(&intcode, &[7]).last_output(), Some(1));
        assert_eq!(run_intcode(&intcode, &[9]).last_output(), Some(0));

        let intcode = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_eq!(run_intcode(&intcode, &[0]).last_output(), Some(0));
        assert_eq!(run_intcode(&intcode, &[1]).last_output(), Some(1));

        let intcode = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_eq!(run_intcode(&intcode, &[0]).last_output(), Some(0));
        assert_eq!(run_intcode(&intcode, &[1]).last_output(), Some(1));

        let intcode = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        assert_eq!(run_intcode(&intcode, &[7]).last_output(), Some(999));
        assert_eq!(run_intcode(&intcode, &[8]).last_output(), Some(1000));
        assert_eq!(run_intcode(&intcode, &[9]).last_output(), Some(1001));

        let intcode = load_intcode("../day05/input/input.txt");
        assert_eq!(run_intcode(&intcode, &[1]).last_output(), Some(12_440_243));
        assert_eq!(run_intcode(&intcode, &[5]).last_output(), Some(15_486_302));
    }

    #[test]
    fn test_day09() {
        let intcode = &[
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let computer = run_intcode(intcode, &[]);
        assert_eq!(computer.outputs(), intcode);

        let intcode = &[1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0];
        let last_output = run_intcode(intcode, &[]).last_output().unwrap();
        assert_eq!(last_output.to_string().len(), 16);

        let intcode = &[104, 1_125_899_906_842_624, 99];
        let last_output = run_intcode(intcode, &[]).last_output().unwrap();
        assert_eq!(last_output, intcode[1]);

        let intcode = load_intcode("../day09/input/input.txt");
        assert_eq!(
            run_intcode(&intcode, &[1]).last_output(),
            Some(2_316_632_620)
        );
        assert_eq!(run_intcode(&intcode, &[2]).last_output(), Some(78869));
    }
}
