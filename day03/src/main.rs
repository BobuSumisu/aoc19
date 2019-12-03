use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::io::{self, Read};
use std::iter::FromIterator;
use std::str::FromStr;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {:?}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let data = parse_input(&input);

    let points_a = points_from_instructions(&data[0]);
    let points_b = points_from_instructions(&data[1]);

    points_a
        .intersection(&points_b)
        .clone()
        .map(|p| p.distance_from_origin())
        .min()
        .unwrap_or(0)
}

fn part2(input: &str) -> u64 {
    let data = parse_input(&input);

    let points_a = points_from_instructions(&data[0]);
    let points_b = points_from_instructions(&data[1]);

    points_a
        .intersection(&points_b)
        .clone()
        .map(|p| {
            let a = points_a.get(p).unwrap();
            let b = points_b.get(p).unwrap();
            a.steps + b.steps
        })
        .min()
        .unwrap()
}

fn parse_input(input: &str) -> Vec<Vec<Instruction>> {
    input
        .lines()
        .map(|line| line.split(',').filter_map(|v| v.parse().ok()).collect())
        .collect()
}

fn points_from_instructions(instrs: &[Instruction]) -> HashSet<Point> {
    let mut points = vec![];
    let mut point = Point {
        x: 0,
        y: 0,
        steps: 0,
    };

    for instr in instrs {
        let mut new_points = instr.points(point);
        point = new_points.last().expect("no last point").clone();
        points.append(&mut new_points);
    }

    HashSet::from_iter(points.into_iter())
}

#[derive(Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err("illegal direction"),
        }
    }
}

struct Instruction {
    dir: Direction,
    amount: usize,
}

impl Instruction {
    fn points(&self, origin: Point) -> Vec<Point> {
        let mut points = vec![];
        let dir: Point = self.dir.clone().into();
        let mut point = origin.clone();

        for _ in 0..self.amount {
            point = Point {
                x: point.x + dir.x,
                y: point.y + dir.y,
                steps: point.steps + 1,
            };
            points.push(point.clone());
        }

        points
    }
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Instruction {
            dir: s[0..1].parse().expect("error parsing direction"),
            amount: s[1..].parse().expect("error parsing amount"),
        })
    }
}

#[derive(Debug, Clone, Eq)]
struct Point {
    x: i64,
    y: i64,
    steps: u64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y, steps: 0 }
    }

    fn distance_from_origin(&self) -> u64 {
        (self.x.abs() + self.y.abs()) as u64
    }
}

impl From<Direction> for Point {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up => Point::new(0, 1),
            Direction::Down => Point::new(0, -1),
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0),
        }
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_examples_part1() {
        assert_eq!(6, part1("R8,U5,L5,D3\nU7,R6,D4,L4"));
        assert_eq!(
            159,
            part1("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83")
        );
        assert_eq!(
            135,
            part1(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )
        );
    }

    #[test]
    fn test_examples_part2() {
        assert_eq!(
            610,
            part2("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83")
        );
        assert_eq!(
            410,
            part2(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )
        );
    }

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(273, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("input/input.txt").unwrap();
        assert_eq!(15622, part2(&input));
    }
}
