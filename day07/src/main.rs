use intcode::{load_intcode, run_intcode, Computer};
use itertools::Itertools;

// .fold(0, |i, s| Computer::new(code, &[s, i]).run())
fn find_best_sequence(code: &[i64]) -> i64 {
    (0..5)
        .permutations(5)
        .map(|p| {
            p.into_iter()
                .fold(0, |i, s| run_intcode(code, &[s, i]).last_output().unwrap())
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
            while !computers[4].is_halted() {
                for computer in computers.iter_mut() {
                    computer.push_input(input);
                    computer.step().unwrap();
                    input = computer.last_output().unwrap();
                }
            }

            computers[4].last_output().unwrap()
        })
        .max()
        .unwrap()
}

fn main() {
    let intcode = load_intcode("input/input.txt");
    println!("Part 1: {}", find_best_sequence(&intcode));
    println!("Part 2: {}", find_best_sequence_feedback(&intcode));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples_part1() {
        let intcode = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(find_best_sequence(&intcode), 43210);
    }

    #[test]
    fn test_solution_part1() {
        let intcode = load_intcode("input/input.txt");
        assert_eq!(find_best_sequence(&intcode), 298_586);
    }

    #[test]
    fn test_examples_part2() {
        let intcode = &[
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(find_best_sequence_feedback(intcode), 139_629_729);
    }

    #[test]
    fn test_solution_part2() {
        let intcode = load_intcode("input/input.txt");
        assert_eq!(find_best_sequence_feedback(&intcode), 9_246_095);
    }
}
