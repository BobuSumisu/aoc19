use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let data = parse_input_string(&input);

    let (min_dist, min_steps) = find_minimum_distance_and_steps(&data[0], &data[1]);
    println!("Part 1: {}", min_dist);
    println!("Part 2: {}", min_steps);
}

enum Move {
    Up(i64),
    Down(i64),
    Left(i64),
    Right(i64),
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        let mut chars = s.chars();
        let direction = chars.next().unwrap();
        let amount = chars.collect::<String>().parse().unwrap();
        match direction {
            'U' => Move::Up(amount),
            'D' => Move::Down(amount),
            'L' => Move::Left(amount),
            'R' => Move::Right(amount),
            c => panic!("unexpected char: {}", c),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }

    fn distance_to_origin(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

fn moves_to_points(start: Point, moves: &[Move]) -> Vec<Point> {
    let mut points = vec![];
    let mut pos = start;
    for mov in moves {
        match mov {
            Move::Up(n) => {
                points.append(&mut (0..*n).map(|x| Point::new(pos.x, pos.y + x)).collect());
                pos.y += n;
            }
            Move::Down(n) => {
                points.append(&mut (0..*n).map(|x| Point::new(pos.x, pos.y - x)).collect());
                pos.y -= n;
            }
            Move::Left(n) => {
                points.append(&mut (0..*n).map(|x| Point::new(pos.x - x, pos.y)).collect());
                pos.x -= n;
            }
            Move::Right(n) => {
                points.append(&mut (0..*n).map(|x| Point::new(pos.x + x, pos.y)).collect());
                pos.x += n;
            }
        }
    }
    points
}

fn find_minimum_distance_and_steps(moves_a: &[Move], moves_b: &[Move]) -> (i64, i64) {
    let origin = Point::new(0, 0);
    let points_a = moves_to_points(origin, &moves_a);
    let points_b = moves_to_points(origin, &moves_b);

    let mut min_dist = std::i64::MAX;
    let mut min_steps = std::i64::MAX;

    for (i, a) in points_a.iter().enumerate() {
        for (j, b) in points_b.iter().enumerate() {
            if a != &origin && a == b {
                min_dist = std::cmp::min(min_dist, a.distance_to_origin());
                min_steps = std::cmp::min(min_steps, (i + j) as i64);
            }
        }
    }

    (min_dist, min_steps)
}

fn parse_input_string(input: &str) -> Vec<Vec<Move>> {
    input
        .lines()
        .map(|line| line.split(',').map(|x| Move::from(x)).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_examples() {
        let input = "R8,U5,L5,D3\nU7,R6,D4,L4";
        let data = parse_input_string(input);
        assert_eq!(6, find_minimum_distance_and_steps(&data[0], &data[1]).0);

        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        let data = parse_input_string(input);
        assert_eq!(159, find_minimum_distance_and_steps(&data[0], &data[1]).0);

        let input =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let data = parse_input_string(input);
        assert_eq!(135, find_minimum_distance_and_steps(&data[0], &data[1]).0);

        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        let data = parse_input_string(input);
        assert_eq!(610, find_minimum_distance_and_steps(&data[0], &data[1]).1);

        let input =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let data = parse_input_string(input);
        assert_eq!(410, find_minimum_distance_and_steps(&data[0], &data[1]).1);
    }

    #[test]
    fn test_solutions() {
        let input = fs::read_to_string("input/input.txt").unwrap();
        let data = parse_input_string(&input);
        let (min_dist, min_steps) = find_minimum_distance_and_steps(&data[0], &data[1]);
        assert_eq!(273, min_dist);
        assert_eq!(15622, min_steps);
    }
}
