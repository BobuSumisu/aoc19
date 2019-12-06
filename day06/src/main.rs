use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("input/input.txt").unwrap();
    let space = parse_input(&input);
    println!("Part 1: {}", space.num_orbits());
    println!("Part 2: {}", space.distance_between("YOU", "SAN"));
}

fn parse_input(input: &str) -> Space {
    let orbits: Vec<Orbit> = input
        .lines()
        .map(|l| {
            let mut i = l.split(')');
            Orbit(i.next().unwrap(), i.next().unwrap())
        })
        .collect();

    orbits.into()
}

#[derive(Debug)]
struct Orbit<'a>(&'a str, &'a str);

#[derive(Debug)]
struct Space<'a> {
    objects: HashSet<&'a str>,
    orbits: HashMap<&'a str, &'a str>,
}

impl<'a> Space<'a> {
    fn num_orbits(&self) -> usize {
        let mut num_orbits = 0;

        for object in self.objects.iter() {
            let mut parent = self.orbits.get(object);
            while let Some(p) = parent {
                num_orbits += 1;
                parent = self.orbits.get(p);
            }
        }

        num_orbits
    }

    fn distance_between(&self, a: &str, b: &str) -> usize {
        let a_parents = self.all_parents(a);
        let b_parents = self.all_parents(b);

        let mut dist = 0;

        for (i, p) in a_parents.iter().enumerate() {
            if let Some(j) = b_parents.iter().position(|x| x == p) {
                dist = i + j;
                break;
            }
        }

        dist
    }

    fn all_parents(&self, name: &str) -> Vec<&str> {
        let mut parents = vec![];
        let mut parent = self.orbits.get(name);

        while let Some(p) = parent {
            parents.push(*p);
            parent = self.orbits.get(p);
        }

        parents
    }
}

impl<'a> From<Vec<Orbit<'a>>> for Space<'a> {
    fn from(v: Vec<Orbit<'a>>) -> Self {
        let mut objects: HashSet<&str> = HashSet::new();
        let mut orbits: HashMap<&str, &str> = HashMap::new();

        for orbit in v {
            objects.insert(orbit.0);
            objects.insert(orbit.1);
            orbits.insert(orbit.1, orbit.0);
        }

        Space { objects, orbits }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples_part1() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
        let space = parse_input(&input);
        assert_eq!(42, space.num_orbits());
    }

    #[test]
    fn test_examples_part2() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";
        let space = parse_input(&input);
        assert_eq!(4, space.distance_between("YOU", "SAN"));
    }

    #[test]
    fn test_part1() {
        let space = parse_input(include_str!("../input/input.txt"));
        assert_eq!(300598, space.num_orbits());
    }

    #[test]
    fn test_part2() {
        let space = parse_input(include_str!("../input/input.txt"));
        assert_eq!(520, space.distance_between("YOU", "SAN"));
    }
}
