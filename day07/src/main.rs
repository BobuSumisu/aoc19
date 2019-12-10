use itertools::Itertools;

#[derive(Default)]
pub struct Computer {
    memory: Vec<i64>,
    inputs: Vec<i64>,
    output: i64,
    ip: usize,
    halted: bool,
}

impl Computer {
    pub fn new(program: &[i64], inputs: &[i64]) -> Self {
        Self {
            memory: program.to_vec(),
            inputs: inputs.to_vec(),
            ..Self::default()
        }
    }

    pub fn run(&mut self) -> i64 {
        while !self.halted {
            self.step();
        }
        self.output
    }

    pub fn step(&mut self) {
        loop {
            match self.memory[self.ip] % 100 {
                1 => {
                    self.put(3, self.arg(1) + self.arg(2));
                    self.ip += 4;
                }
                2 => {
                    self.put(3, self.arg(1) * self.arg(2));
                    self.ip += 4;
                }
                3 => {
                    let input = self.inputs.remove(0);
                    self.put(1, input);
                    self.ip += 2;
                }
                4 => {
                    self.output = self.arg(1);
                    self.ip += 2;
                    break;
                }
                5 => {
                    if self.arg(1) == 0 {
                        self.ip += 3;
                    } else {
                        self.ip = self.arg(2) as usize;
                    }
                }
                6 => {
                    if self.arg(1) == 0 {
                        self.ip = self.arg(2) as usize;
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
                99 => {
                    self.halted = true;
                    break;
                }
                _ => panic!("illegal opcode"),
            }
        }
    }

    fn arg(&self, i: usize) -> i64 {
        let v = self.memory[self.ip + i];
        match self.mode(i) {
            0 => self.memory[v as usize],
            1 => v,
            _ => panic!("illegal mode"),
        }
    }

    fn mode(&self, i: usize) -> i64 {
        let instr = self.memory[self.ip] / 100;
        (instr / 10_i64.pow((i - 1) as u32)) % 10
    }

    fn put(&mut self, i: usize, value: i64) {
        let dest = self.memory[self.ip + i] as usize;
        self.memory[dest] = value;
    }
}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn find_best_sequence(code: &[i64]) -> i64 {
    (0..5)
        .permutations(5)
        .map(|p| {
            p.into_iter()
                .fold(0, |i, s| Computer::new(code, &[s, i]).run())
        })
        .max()
        .unwrap()
}

fn find_best_sequence_feedback(code: &[i64]) -> i64 {
    (0..5)
        .permutations(5)
        .map(|p| {
            let mut computers = p
                .into_iter()
                .map(|x| Computer::new(code, &[x + 5]))
                .collect::<Vec<Computer>>();

            let mut input = 0;
            while !computers[4].halted {
                for computer in computers.iter_mut() {
                    computer.inputs.push(input);
                    computer.step();
                    input = computer.output;
                }
            }

            computers[4].output
        })
        .max()
        .unwrap()
}

fn main() {
    let input = parse_input(include_str!("../input/input.txt"));
    println!("Part 1: {}", find_best_sequence(&input));
    println!("Part 2: {}", find_best_sequence_feedback(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples_part1() {
        let code = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(find_best_sequence(&code), 43210);
    }

    #[test]
    fn test_solution_part1() {
        let code = parse_input(include_str!("../input/input.txt"));
        assert_eq!(find_best_sequence(&code), 298_586);
    }

    #[test]
    fn test_examples_part2() {
        let code = &[
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(139_629_729, find_best_sequence_feedback(code));
    }

    #[test]
    fn test_solution_part2() {
        let code = parse_input(include_str!("../input/input.txt"));
        assert_eq!(find_best_sequence_feedback(&code), 9_246_095);
    }
}
