use intcode::{load_intcode, Computer};

fn run_intcode_patched(intcode: &[i64], patch: (i64, i64)) -> Computer {
    let mut computer = Computer::new(intcode, &[]);
    computer.patch(patch);
    computer.run().unwrap();
    computer
}

fn find_correct_patch(intcode: &[i64], target: i64) -> Option<(i64, i64)> {
    for x in 0..100 {
        for y in 0..100 {
            let patch = (x, y);
            let computer = run_intcode_patched(intcode, patch);
            if computer.memory()[0] == target {
                return Some(patch);
            }
        }
    }
    None
}

fn part1(intcode: &[i64]) -> i64 {
    let computer = run_intcode_patched(intcode, (12, 2));
    computer.memory()[0]
}

fn part2(intcode: &[i64]) -> i64 {
    match find_correct_patch(intcode, 19_690_720) {
        Some((x, y)) => 100 * x + y,
        None => 0,
    }
}

fn main() {
    let intcode = load_intcode("input/input.txt");
    println!("Part 1: {}", part1(&intcode));
    println!("Part 2: {}", part2(&intcode));
}

#[cfg(test)]
mod tests {
    use super::*;
    use intcode::run_intcode;

    #[test]
    fn test_examples_part1() {
        assert_eq!(run_intcode(&vec![1, 0, 0, 0, 99], &[]).memory()[0], 2);
        assert_eq!(run_intcode(&vec![1, 0, 0, 0, 99], &[]).memory()[0], 2);
        assert_eq!(run_intcode(&vec![2, 3, 0, 3, 99], &[]).memory()[0], 2);
        assert_eq!(run_intcode(&vec![2, 4, 4, 5, 99, 0], &[]).memory()[0], 2);
        assert_eq!(
            run_intcode(&vec![1, 1, 1, 4, 99, 5, 6, 0, 99], &[]).memory()[0],
            30
        );
        assert_eq!(
            run_intcode(&vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], &[]).memory()[0],
            3500
        );
    }

    #[test]
    fn test_examples_part2() {
        let intcode = load_intcode("input/input.txt");
        let computer = run_intcode_patched(&intcode, (12, 2));
        let (x, y) = computer.get_patch();
        assert_eq!(100 * x + y, 1202);
    }

    #[test]
    fn test_part1() {
        let intcode = load_intcode("input/input.txt");
        assert_eq!(
            run_intcode_patched(&intcode, (12, 2)).memory()[0],
            3_654_868
        );
    }

    #[test]
    fn test_part2() {
        let intcode = load_intcode("input/input.txt");
        let (x, y) = find_correct_patch(&intcode, 19_690_720).unwrap();
        assert_eq!(100 * x + y, 7014);
    }
}
