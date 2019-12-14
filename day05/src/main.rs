use intcode::{load_intcode, run_intcode};

fn main() {
    let intcode = load_intcode("input/input.txt");
    println!(
        "Part 1: {}",
        run_intcode(&intcode, &[1]).last_output().unwrap()
    );
    println!(
        "Part 2: {}",
        run_intcode(&intcode, &[5]).last_output().unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_part1() {
        let intcode = load_intcode("input/input.txt");
        assert_eq!(run_intcode(&intcode, &[1]).last_output(), Some(12_440_243));
    }

    #[test]
    fn test_solution_part2() {
        let intcode = load_intcode("input/input.txt");
        assert_eq!(run_intcode(&intcode, &[5]).last_output(), Some(15_486_302));
    }
}
