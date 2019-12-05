#![allow(dead_code)]
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let program = parse_input(&input);

    println!("Part 1: {}", run_program(&program, 1).unwrap());
    println!("Part 2: {}", run_program(&program, 5).unwrap());
}

const OP_ADD: i64 = 1;
const OP_MULTIPLY: i64 = 2;
const OP_READ: i64 = 3;
const OP_WRITE: i64 = 4;
const OP_JUMP_IF_TRUE: i64 = 5;
const OP_JUMP_IF_FALSE: i64 = 6;
const OP_LESS_THAN: i64 = 7;
const OP_EQUALS: i64 = 8;
const OP_HALT: i64 = 99;

const MODE_POSITION: i64 = 0;
const MODE_IMMEDIATE: i64 = 1;

fn parse_input(s: &str) -> Vec<i64> {
    s.trim().split(',').map(|x| x.parse().unwrap()).collect()
}

const fn parse_instruction(n: i64) -> (i64, i64, i64, i64) {
    let opcode = n % 100;
    let mut params = n / 100;
    let a = params % 10;
    params /= 10;
    let b = params % 10;
    params /= 10;
    let c = params % 10;

    (opcode, a, b, c)
}

fn read_param(memory: &[i64], ip: usize, num: usize, mode: i64) -> i64 {
    let x = memory[ip + num];
    if mode == MODE_POSITION {
        memory[x as usize]
    } else {
        x
    }
}

fn run_program(program: &[i64], input: i64) -> Option<i64> {
    let mut output = vec![];
    let mut memory = program.to_owned();
    let mut ip = 0;

    loop {
        let (opcode, a, b, _) = parse_instruction(memory[ip]);

        match opcode {
            OP_ADD | OP_MULTIPLY => {
                let x = read_param(&memory, ip, 1, a);
                let y = read_param(&memory, ip, 2, b);
                let z = read_param(&memory, ip, 3, MODE_IMMEDIATE);

                memory[z as usize] = if opcode == OP_ADD { x + y } else { x * y };

                ip += 4;
            }
            OP_READ => {
                let x = read_param(&memory, ip, 1, MODE_IMMEDIATE);

                memory[x as usize] = input;

                ip += 2;
            }
            OP_WRITE => {
                let x = read_param(&memory, ip, 1, a);

                output.push(x);

                ip += 2;
            }
            OP_JUMP_IF_TRUE | OP_JUMP_IF_FALSE => {
                let x = read_param(&memory, ip, 1, a);
                let y = read_param(&memory, ip, 2, b);

                if (opcode == OP_JUMP_IF_TRUE && x != 0) || (opcode == OP_JUMP_IF_FALSE && x == 0) {
                    ip = y as usize;
                } else {
                    ip += 3;
                }
            }
            OP_LESS_THAN | OP_EQUALS => {
                let x = read_param(&memory, ip, 1, a);
                let y = read_param(&memory, ip, 2, b);
                let z = read_param(&memory, ip, 3, MODE_IMMEDIATE);

                if (opcode == OP_LESS_THAN && x < y) || (opcode == OP_EQUALS && x == y) {
                    memory[z as usize] = 1;
                } else {
                    memory[z as usize] = 0;
                }

                ip += 4;
            }
            OP_HALT => {
                break;
            }
            _ => {
                panic!("illegal opcode");
            }
        }
    }

    output.last().cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_examples_part1() {
        let input = vec![1002, 4, 3, 4, 33];
        assert_eq!(None, run_program(&input, 1));
    }

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("input/input.txt").unwrap();
        let program = parse_input(&input);
        assert_eq!(Some(12440243), run_program(&program, 1));
    }

    #[test]
    fn test_examples_part2() {
        let program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(Some(0), run_program(&program, 7));
        assert_eq!(Some(1), run_program(&program, 8));

        let program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(Some(1), run_program(&program, 7));
        assert_eq!(Some(0), run_program(&program, 9));

        let program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        assert_eq!(Some(0), run_program(&program, 7));
        assert_eq!(Some(1), run_program(&program, 8));

        let program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        assert_eq!(Some(1), run_program(&program, 7));
        assert_eq!(Some(0), run_program(&program, 9));

        let program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_eq!(Some(0), run_program(&program, 0));
        assert_eq!(Some(1), run_program(&program, 1));

        let program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_eq!(Some(0), run_program(&program, 0));
        assert_eq!(Some(1), run_program(&program, 1));

        let program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        assert_eq!(Some(999), run_program(&program, 7));
        assert_eq!(Some(1000), run_program(&program, 8));
        assert_eq!(Some(1001), run_program(&program, 9));
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("input/input.txt").unwrap();
        let program = parse_input(&input);
        assert_eq!(Some(15486302), run_program(&program, 5));
    }
}
