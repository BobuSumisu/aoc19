use std::fs;
use std::iter;

fn mass_to_fuel(mass: u64) -> u64 {
    (mass / 3).saturating_sub(2)
}

fn total_mass_to_fuel(mass: u64) -> u64 {
    iter::successors(Some(mass_to_fuel(mass)), |&fuel| Some(mass_to_fuel(fuel)))
        .take_while(|&fuel| fuel != 0)
        .sum()
}

fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn load_input() -> Vec<u64> {
    parse_input(&fs::read_to_string("input/input.txt").unwrap())
}

fn part1(input: &[u64]) -> u64 {
    input.iter().map(|&x| mass_to_fuel(x)).sum()
}

fn part2(input: &[u64]) -> u64 {
    input.iter().map(|&x| total_mass_to_fuel(x)).sum()
}

fn main() {
    let input = load_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mass_to_fuel() {
        assert_eq!(mass_to_fuel(12), 2);
        assert_eq!(mass_to_fuel(14), 2);
        assert_eq!(mass_to_fuel(1969), 654);
        assert_eq!(mass_to_fuel(100_756), 33583);
    }

    #[test]
    fn test_total_mass_to_fuel() {
        assert_eq!(total_mass_to_fuel(14), 2);
        assert_eq!(total_mass_to_fuel(1969), 966);
        assert_eq!(total_mass_to_fuel(100_756), 50346);
    }

    #[test]
    fn test_solution_part1() {
        let input = load_input();
        assert_eq!(part1(&input), 3_267_890);
    }

    #[test]
    fn test_solution_part2() {
        let input = load_input();
        assert_eq!(part2(&input), 4_898_972);
    }
}
