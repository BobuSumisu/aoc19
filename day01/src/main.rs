use std::io::{self, Read};

type Result<T> = std::result::Result<T, std::boxed::Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let values = parse_input(input)?;

    let ans1 = part1(&values)?;
    println!("part1: {}", ans1);

    let ans2 = part2(&values)?;
    println!("part2: {}", ans2);

    Ok(())
}

fn parse_input(input: String) -> Result<Vec<u32>> {
    match input.lines().map(|line| line.parse::<u32>()).collect() {
        Ok(values) => Ok(values),
        Err(err) => Err(format!("failed to parse integer: {}", err))?,
    }
}

fn fuel_required(mass: u32, extra_fuel: bool) -> u32 {
    let fuel = (mass as f32 / 3.0).floor() as i32 - 2;
    let fuel = if fuel < 0 { 0 } else { fuel as u32 };

    if fuel == 0 {
        0
    } else if extra_fuel {
        fuel + fuel_required(fuel, true)
    } else {
        fuel
    }
}

fn part1(values: &Vec<u32>) -> Result<u32> {
    let v = values.iter().map(|v| fuel_required(*v, false)).sum();
    Ok(v)
}

fn part2(values: &Vec<u32>) -> Result<u32> {
    let v = values.iter().map(|v| fuel_required(*v, true)).sum();
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_fuel_required() {
        assert_eq!(fuel_required(12, false), 2);
        assert_eq!(fuel_required(14, false), 2);
        assert_eq!(fuel_required(1969, false), 654);
        assert_eq!(fuel_required(100756, false), 33583);
    }

    #[test]
    fn test_extra_fuel_required() {
        assert_eq!(fuel_required(14, true), 2);
        assert_eq!(fuel_required(1969, true), 966);
        assert_eq!(fuel_required(100756, true), 50346);
    }

    #[test]
    fn test_part1() -> Result<()> {
        let values = parse_input(fs::read_to_string("input/input.txt")?)?;
        assert_eq!(part1(&values)?, 3267890);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let values = parse_input(fs::read_to_string("input/input.txt")?)?;
        assert_eq!(part2(&values)?, 4898972);
        Ok(())
    }
}
