use std::fmt;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

#[derive(Debug, Default)]
struct Layer {
    pixels: Vec<u8>,
}

impl Layer {
    fn pixel_count(&self, v: u8) -> usize {
        self.pixels.iter().filter(|&p| *p == v).count()
    }
}

impl From<&str> for Layer {
    fn from(s: &str) -> Self {
        Self {
            pixels: s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect(),
        }
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.pixels[(y * WIDTH) + x] == 1 {
                    write!(f, "*")?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

struct Image {
    layers: Vec<Layer>,
}

impl Image {
    fn find_min_pixel_count(&self, v: u8) -> Option<&Layer> {
        self.layers.iter().min_by_key(|l| l.pixel_count(v))
    }

    fn get_merged_layer(&self) -> Layer {
        let mut merged = Layer {
            pixels: vec![0; WIDTH * HEIGHT],
        };

        for layer in self.layers.iter().rev() {
            for i in 0..(WIDTH * HEIGHT) {
                if layer.pixels[i] != 2 {
                    merged.pixels[i] = layer.pixels[i];
                }
            }
        }

        merged
    }
}

fn parse_input(input: &str) -> Image {
    let layers = input
        .trim()
        .as_bytes()
        .chunks(WIDTH * HEIGHT)
        .map(|c| unsafe { std::str::from_utf8_unchecked(c) })
        .map(|s| s.into())
        .collect();
    Image { layers }
}

fn solution_part1(image: &Image) -> usize {
    let layer = image.find_min_pixel_count(0).unwrap();
    layer.pixel_count(1) * layer.pixel_count(2)
}

fn solution_part2(image: &Image) -> Layer {
    image.get_merged_layer()
}

fn main() {
    let image = parse_input(include_str!("../input/input.txt"));
    println!("Part 1: {}", solution_part1(&image));
    solution_part2(&image);
    println!("Part 2:\n{}", solution_part2(&image));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_part1() {
        let image = parse_input(include_str!("../input/input.txt"));
        assert_eq!(solution_part1(&image), 2286);
    }
}
