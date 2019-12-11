use std::collections::HashSet;
use std::fs;
use std::mem;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn quad(&self, other: &Self) -> i64 {
        if other.x >= self.x {
            if other.y >= self.y {
                1
            } else {
                4
            }
        } else {
            if other.y >= self.y {
                2
            } else {
                3
            }
        }
    }

    fn slope(&self, other: &Self) -> f64 {
        (other.y - self.y) / (other.x - self.x)
    }

    fn dist(&self, other: &Self) -> f64 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < 0.0001 && (self.y - other.y).abs() < 0.0001
    }
}

#[derive(Debug)]
struct Space {
    asteroids: Vec<Point>,
}

impl Space {
    fn can_see(&self, a: &Point, b: &Point) -> bool {
        if a == b {
            return false;
        }

        // Yeaa, so the idea here is iterating through every asteroid and check
        // if they are in front of `b`. This is done by simply checking if `b` and `c` are in
        // the same quadrant relative to `a` and that they have the same slope.
        // My thought was if so, and if `c` is closer than `b`, then `c` is blocking the view from
        // `a` to `b`.
        //
        // This is not very effective: for every asteroid `a` you have to check if it can see every
        // asteroid `b` which required checking if any asteroid `c` is in between.
        //
        // So I guess worst runtime would be `n^3` :P
        let quad = a.quad(b);
        let slope = a.slope(b);
        let dist = a.dist(b);

        for c in &self.asteroids {
            if c == a || c == b {
                continue;
            }

            if a.quad(c) == quad && a.slope(c) == slope && a.dist(c) < dist {
                return false;
            }
        }

        true
    }

    fn find_best(&self) -> usize {
        let mut best = 0;

        for a in &self.asteroids {
            let mut can_see = 0;

            for b in &self.asteroids {
                if self.can_see(a, b) {
                    can_see += 1;
                }
            }

            if can_see > best {
                best = can_see
            }
        }

        best
    }
}

fn parse_input(input: &str) -> Space {
    let asteroids = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == '#' {
                        Some(Point::new(x as f64, y as f64))
                    } else {
                        None
                    }
                })
                .collect::<Vec<Point>>()
        })
        .collect();
    Space { asteroids }
}

fn get_input() -> Space {
    parse_input(&fs::read_to_string("input/input.txt").unwrap())
}

fn main() {
    let space = get_input();
    println!("Part 1: {}", space.find_best());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_part1() {
        let space = get_input();
        assert_eq!(space.find_best(), 286);
    }
}
