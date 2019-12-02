use std::io::{self, Read};

struct Patch(u64, u64);

fn run_program(program: &[u64], patch: Option<Patch>) -> Vec<u64> {
    let mut memory = program.to_owned();
    if let Some(Patch(x, y)) = patch {
        memory[1] = x;
        memory[2] = y;
    }

    let mut ip = 0;

    loop {
        match memory[ip] {
            1 | 2 => {
                let x = memory[memory[ip + 1] as usize];
                let y = memory[memory[ip + 2] as usize];
                let dst = memory[ip + 3] as usize;
                memory[dst] = if memory[ip] == 1 { x + y } else { x * y };
                ip += 4;
            }
            99 => break,
            _ => panic!("unknown opcode"),
        }
    }

    memory
}

fn find_correct_patch(program: &[u64], target: u64) -> Option<Patch> {
    for x in 0..100 {
        for y in 0..100 {
            let memory = run_program(program, Some(Patch(x, y)));
            if memory[0] == target {
                return Some(Patch(memory[1], memory[2]));
            }
        }
    }
    None
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let code: Vec<u64> = input.split(',').filter_map(|v| v.parse().ok()).collect();

    let memory = run_program(&code, Some(Patch(12, 2)));
    println!("Part 1: {}", memory[0]);

    let patch = find_correct_patch(&code, 19690720).expect("correct patch not found");
    println!("Part 2: {}", 100 * patch.0 + patch.1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn read_input() -> Vec<u64> {
        fs::read_to_string("input/input.txt")
            .unwrap()
            .split(',')
            .filter_map(|v| v.parse().ok())
            .collect()
    }

    #[test]
    fn test_examples_part1() {
        assert_eq!(2, run_program(&vec![1, 0, 0, 0, 99], None)[0]);
        assert_eq!(2, run_program(&vec![2, 3, 0, 3, 99], None)[0]);
        assert_eq!(2, run_program(&vec![2, 4, 4, 5, 99, 0], None)[0]);
        assert_eq!(30, run_program(&vec![1, 1, 1, 4, 99, 5, 6, 0, 99], None)[0]);
        assert_eq!(
            3500,
            run_program(&vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], None)[0]
        );
    }

    #[test]
    fn test_examples_part2() {
        let program = read_input();
        let memory = run_program(&program, Some(Patch(12, 2)));
        assert_eq!(1202, 100 * memory[1] + memory[2]);
    }

    #[test]
    fn test_part1() {
        let program = read_input();
        let memory = run_program(&program, Some(Patch(12, 2)));
        assert_eq!(3654868, memory[0]);
    }

    #[test]
    fn test_part2() {
        let program = read_input();
        let patch = find_correct_patch(&program, 19690720).expect("correct patch not found");
        assert_eq!(7014, 100 * patch.0 + patch.1);
    }
}
