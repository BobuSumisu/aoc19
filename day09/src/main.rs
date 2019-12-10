use std::convert::TryInto;
use std::fs;

#[derive(Default)]
pub struct Computer {
    memory: Vec<i128>,
    inputs: Vec<i128>,
    outputs: Vec<i128>,
    ip: i128,
    halted: bool,
    base: i128,
}

impl Computer {
    pub fn new(program: &[i128], inputs: &[i128]) -> Self {
        Self {
            memory: program.to_vec(),
            inputs: inputs.to_vec(),
            ..Self::default()
        }
    }

    pub fn run(&mut self) -> Vec<i128> {
        while !self.halted {
            self.step();
        }
        self.outputs.clone()
    }

    pub fn step(&mut self) {
        loop {
            match self.memory[self.ip as usize] % 100 {
                1 => {
                    let x = self.arg(1);
                    let y = self.arg(2);
                    self.put(3, x + y);
                    self.ip += 4;
                }
                2 => {
                    let x = self.arg(1);
                    let y = self.arg(2);
                    self.put(3, x * y);
                    self.ip += 4;
                }
                3 => {
                    let input = self.inputs.remove(0);
                    self.put(1, input);
                    self.ip += 2;
                }
                4 => {
                    let x = self.arg(1);
                    self.outputs.push(x);
                    self.ip += 2;
                    break;
                }
                5 => {
                    if self.arg(1) == 0 {
                        self.ip += 3;
                    } else {
                        self.ip = self.arg(2);
                    }
                }
                6 => {
                    if self.arg(1) == 0 {
                        self.ip = self.arg(2);
                    } else {
                        self.ip += 3;
                    }
                }
                7 => {
                    if self.arg(1) < self.arg(2) {
                        self.put(3, 1);
                    } else {
                        self.put(3, 0);
                    }
                    self.ip += 4;
                }
                8 => {
                    if self.arg(1) == self.arg(2) {
                        self.put(3, 1);
                    } else {
                        self.put(3, 0);
                    }
                    self.ip += 4;
                }
                9 => {
                    self.base += self.arg(1);
                    self.ip += 2;
                }
                99 => {
                    self.halted = true;
                    break;
                }
                _ => panic!("illegal opcode"),
            }
        }
    }

    fn arg(&mut self, i: i128) -> i128 {
        let v = self.memory_get(self.ip + i);
        match self.mode(i) {
            0 => self.memory_get(v),
            1 => v,
            2 => self.memory_get(self.base + v),
            _ => panic!("illegal mode"),
        }
    }

    fn mode(&mut self, i: i128) -> i128 {
        let instr = self.memory_get(self.ip) / 100;
        (instr / 10_i128.pow((i - 1) as u32)) % 10
    }

    fn put(&mut self, i: i128, value: i128) {
        let dest = self.memory_get(self.ip + i);
        match self.mode(i) {
            0 => self.memory_set(dest, value),
            2 => self.memory_set(self.base + dest, value),
            _ => panic!("illegal mode"),
        };
    }

    fn memory_get(&mut self, i: i128) -> i128 {
        let index: usize = i.try_into().unwrap();
        if self.memory.len() < index + 1 {
            self.memory.resize(index + 1, 0);
        }
        self.memory[index]
    }

    fn memory_set(&mut self, i: i128, v: i128) {
        let index: usize = i.try_into().unwrap();
        if self.memory.len() < index + 1 {
            self.memory.resize(index + 1, 0);
        }
        self.memory[index] = v;
    }
}

fn get_input() -> Vec<i128> {
    fs::read_to_string("input/input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn main() {
    let code = get_input();
    println!("Part 1: {}", Computer::new(&code, &[1]).run()[0]);
    println!("Part 2: {}", Computer::new(&code, &[2]).run()[0]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples_part1() {
        let code = &[
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let output = Computer::new(code, &[]).run();
        assert_eq!(output, code);

        let code = &[1102, 34_915_192, 34_915_192, 7, 4, 7, 99, 0];
        let output = Computer::new(code, &[]).run();
        assert_eq!(output[0].to_string().len(), 16);

        let code = &[104, 1_125_899_906_842_624, 99];
        let output = Computer::new(code, &[]).run();
        assert_eq!(output[0], code[1]);
    }

    #[test]
    fn test_solution_part1() {
        let code = get_input();
        assert_eq!(Computer::new(&code, &[1]).run()[0], 2_316_632_620);
    }

    #[test]
    fn test_solution_part2() {
        let code = get_input();
        assert_eq!(Computer::new(&code, &[2]).run()[0], 78869);
    }
}
