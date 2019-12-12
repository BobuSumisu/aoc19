use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::f64::EPSILON;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn bearing(&self, other: &Self) -> f64 {
        let theta = ((other.x - self.x) as f64).atan2((other.y - self.y) as f64);
        let degrees = 180.0 - theta.to_degrees();

        if degrees < 0.0 {
            degrees + 180.0
        } else {
            degrees
        }
    }

    fn distance(&self, other: &Self) -> f64 {
        let x = (other.x - self.x) as f64;
        let y = (other.y - self.y) as f64;
        (x * x + y * y).sqrt()
    }
}

#[derive(Debug)]
struct RelPoint {
    point: Point,
    bearing: f64,
    distance: f64,
}

impl RelPoint {
    fn new(point: &Point, origin: &Point) -> Self {
        Self {
            point: *point,
            bearing: origin.bearing(point),
            distance: origin.distance(point),
        }
    }
}

fn view_count(points: &[Point], point: &Point) -> usize {
    let mut count = 0;

    let rel_points: Vec<RelPoint> = points
        .iter()
        .filter_map(|p| {
            if p == point {
                None
            } else {
                Some(RelPoint::new(p, point))
            }
        })
        .collect();

    'outer: for target in &rel_points {
        for blocker in &rel_points {
            if blocker.point == target.point {
                continue;
            }

            if target.bearing == blocker.bearing && blocker.distance < target.distance {
                continue 'outer;
            }
        }

        count += 1;
    }

    count
}

fn find_best(points: &[Point]) -> (Point, usize) {
    let mut best_count = 0;
    let mut best = points[0];

    for point in points {
        let count = view_count(points, point);
        if count > best_count {
            best_count = count;
            best = *point;
        }
    }

    (best, best_count)
}

fn vaporize(points: &[Point], point: &Point, max: usize) -> Vec<Point> {
    // Augment with bearing and distance information (and remove point).
    let mut rel_points: Vec<RelPoint> = points
        .iter()
        .filter_map(|p| {
            if p == point {
                None
            } else {
                Some(RelPoint::new(p, point))
            }
        })
        .collect();

    // Group points by bearing and sort by distance.
    // Must sort first since group_by only looks at consecutive elements.

    rel_points.sort_by(|p0, p1| {
        if p0.bearing < p1.bearing {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let mut grouped: Vec<Vec<_>> = rel_points
        .iter()
        .group_by(|p| p.bearing)
        .into_iter()
        .map(|(_, g)| g.into_iter().collect::<Vec<&RelPoint>>())
        .collect();

    // Now sort each group by distance.
    for group in grouped.iter_mut() {
        group.sort_by(|p0, p1| {
            if p0.distance < p1.distance {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
    }

    // Now we can start the show by simply looping over the groups and removing the first element.
    let mut vaporized = vec![];

    while vaporized.len() < max {
        let mut found = false;

        for group in grouped.iter_mut() {
            if !group.is_empty() {
                let rel_point = group.remove(0);
                vaporized.push(rel_point.point);
                found = true;

                if vaporized.len() == max {
                    return vaporized;
                }
            }
        }

        if !found {
            break;
        }
    }

    vaporized
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(Point::new(x as i64, y as i64))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn main() {
    let points = parse_input(include_str!("../input/input.txt"));

    let (best_point, view_count) = find_best(&points);
    println!("Part 1: {}", view_count);

    let vaporized = vaporize(&points, &best_point, 200);
    let point = vaporized[199];

    println!("Part 2: {}", point.x * 100 + point.y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_part1() {
        let points = parse_input(include_str!("../input/input.txt"));
        assert_eq!(find_best(&points).1, 286);
    }

    #[test]
    fn test_examples_part2() {
        let points = parse_input(
            ".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##",
        );
        let vaporized = vaporize(&points, &Point::new(8, 3), 10);
        assert_eq!(vaporized[0], Point::new(8, 1));
        assert_eq!(vaporized[1], Point::new(9, 0));
        assert_eq!(vaporized[2], Point::new(9, 1));
        assert_eq!(vaporized[3], Point::new(10, 0));
        assert_eq!(vaporized[4], Point::new(9, 2));
        assert_eq!(vaporized[5], Point::new(11, 1));
        assert_eq!(vaporized[6], Point::new(12, 1));
        assert_eq!(vaporized[7], Point::new(11, 2));
        assert_eq!(vaporized[8], Point::new(15, 1));
    }

    #[test]
    fn test_solution_part2() {
        let points = parse_input(include_str!("../input/input.txt"));
        let vaporized = vaporize(&points, &Point::new(22, 25), 200);
        let point = vaporized[199];
        assert_eq!(point.x * 100 + point.y, 504);
    }
}
