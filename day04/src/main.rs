struct DigitIter(u64);

impl Iterator for DigitIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let d = self.0 % 10;
            self.0 /= 10;
            Some(d)
        }
    }
}

fn is_valid_password(password: u64) -> bool {
    let mut has_double = false;
    let mut prev = None;

    for n in DigitIter(password) {
        if let Some(prev) = prev {
            if n > prev {
                return false;
            }
            if n == prev {
                has_double = true;
            }
        }
        prev = Some(n);
    }

    has_double
}

fn is_valid_password2(password: u64) -> bool {
    let mut has_double = false;
    let mut prev = None;
    let mut seen = 0;

    for n in DigitIter(password) {
        if let Some(prev) = prev {
            if n > prev {
                return false;
            }

            if n == prev {
                seen += 1
            } else {
                if seen == 2 {
                    has_double = true;
                }
                seen = 1
            }
        } else {
            seen = 1
        }

        prev = Some(n);
    }

    if seen == 2 {
        has_double = true;
    }

    has_double
}

fn get_input() -> impl Iterator<Item = u64> {
    (356_261..846_303)
}

fn part1() -> usize {
    get_input().filter(|&x| is_valid_password(x)).count()
}

fn part2() -> usize {
    get_input().filter(|&x| is_valid_password2(x)).count()
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples_part1() {
        assert!(is_valid_password(111111));
        assert!(!is_valid_password(223450));
        assert!(!is_valid_password(123789));
    }

    #[test]
    fn test_examples_part2() {
        assert!(is_valid_password2(112233));
        assert!(!is_valid_password2(123444));
        assert!(is_valid_password2(111122));
        assert!(!is_valid_password2(211122));
        assert!(!is_valid_password2(111222));
        assert!(is_valid_password2(111199));
        assert!(!is_valid_password2(123456));
        assert!(is_valid_password2(113456));
        assert!(is_valid_password2(111224));
        assert!(!is_valid_password2(122234));
    }

    #[test]
    fn test_solution_part1() {
        assert_eq!(544, part1());
    }

    #[test]
    fn test_solution_part2() {
        assert_eq!(334, part2());
    }
}
