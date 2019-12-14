use intcode::{load_intcode, run_intcode};

fn main() {
    let intcode = load_intcode("input/input.txt");
    println!(
        "Part 1: {}",
        run_intcode(&intcode, &[1]).last_output().unwrap()
    );
    println!(
        "Part 2: {}",
        run_intcode(&intcode, &[2]).last_output().unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples_part1() {
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
    }

    #[test]
    fn test_solution_part1() {
        let intcode = load_intcode("input/input.txt");
        assert_eq!(
            run_intcode(&intcode, &[1]).last_output(),
            Some(2_316_632_620)
        );
    }

    #[test]
    fn test_solution_part2() {
        let intcode = load_intcode("input/input.txt");
        assert_eq!(run_intcode(&intcode, &[2]).last_output(), Some(78869));
    }
}
