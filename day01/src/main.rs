use std::io::{self, Read};

fn mass_to_fuel(mass: u64) -> u64 {
    (mass / 3).saturating_sub(2)
}

struct Module {
    mass: u64,
}

impl Module {
    fn new(mass: u64) -> Module {
        Module { mass }
    }

    fn fuel_required(&self) -> u64 {
        mass_to_fuel(self.mass)
    }

    fn fuel_required_corrected(&self) -> u64 {
        std::iter::successors(Some(self.fuel_required()), |&fuel| Some(mass_to_fuel(fuel)))
            .take_while(|&fuel| fuel != 0)
            .sum()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let modules: Vec<Module> = input
        .lines()
        .filter_map(|line| line.parse().ok())
        .map(|mass| Module::new(mass))
        .collect();

    let solution_part1: u64 = modules.iter().map(|module| module.fuel_required()).sum();
    println!("Solution part 1: {}", solution_part1);

    let solution_part2: u64 = modules
        .iter()
        .map(|module| module.fuel_required_corrected())
        .sum();
    println!("Solution part 2: {}", solution_part2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_fuel_required() {
        assert_eq!(Module::new(12).fuel_required(), 2);
        assert_eq!(Module::new(14).fuel_required(), 2);
        assert_eq!(Module::new(1969).fuel_required(), 654);
        assert_eq!(Module::new(100756).fuel_required(), 33583);
    }

    #[test]
    fn test_extra_fuel_required() {
        assert_eq!(Module::new(14).fuel_required_corrected(), 2);
        assert_eq!(Module::new(1969).fuel_required_corrected(), 966);
        assert_eq!(Module::new(100756).fuel_required_corrected(), 50346);
    }

    #[test]
    fn test_part1() {
        let sum: u64 = fs::read_to_string("input/input.txt")
            .unwrap()
            .lines()
            .filter_map(|line| line.parse().ok())
            .map(|mass| Module::new(mass).fuel_required())
            .sum();
        assert_eq!(sum, 3267890);
    }

    #[test]
    fn test_part2() {
        let sum: u64 = fs::read_to_string("input/input.txt")
            .unwrap()
            .lines()
            .filter_map(|line| line.parse().ok())
            .map(|mass| Module::new(mass).fuel_required_corrected())
            .sum();
        assert_eq!(sum, 4898972);
    }
}
