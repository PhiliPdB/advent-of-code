use std::fmt::{Display, Formatter, Result};

const ENHANCEMENTS: i32 = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pixel {
    Light,
    Dark,
}

impl Pixel {
    pub const fn from_char(c: char) -> Option<Self> {
        match c {
            '#' => Some(Pixel::Light),
            '.' => Some(Pixel::Dark),
            _ => None
        }
    }

    pub const fn value(&self) -> usize {
        match self {
            Pixel::Light => 1,
            Pixel::Dark => 0,
        }
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Pixel::Light => write!(f, "#"),
            Pixel::Dark => write!(f, "."),
        }
    }
}

const fn pixel_grid((x, y): (i32, i32)) -> [(i32, i32); 9] {
    [
        (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
        (x - 1, y),     (x, y),     (x + 1, y),
        (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)
    ]
}

fn get_pixel(image: &[Vec<Pixel>], x: i32, y: i32, iteration: i32, default: Pixel) -> Pixel {
    if x < 0 || x >= image[0].len() as i32 || y < 0 || y >= image.len() as i32 {
        if iteration % 2 == 0 {
            default
        } else {
            Pixel::Dark
        }
    } else {
        image[y as usize][x as usize]
    }
}

fn calculate_pixel_index(image: &[Vec<Pixel>], (x, y): (i32, i32), iteration: i32, default: Pixel) -> usize {
    let mut index = 0;
    for (px, py) in pixel_grid((x, y)) {
        index <<= 1;
        index |= get_pixel(image, px, py, iteration, default).value();
    }
    index
}

fn enhancement_pass(enhancement_algorithm: &[Pixel], image: &[Vec<Pixel>], iteration: i32) -> Vec<Vec<Pixel>> {
    let mut new_image = vec![vec![Pixel::Dark; image[0].len() + 2]; image.len() + 2];
    for y in 0..new_image.len() {
        for x in 0..new_image[0].len() {
            new_image[y][x] = enhancement_algorithm[calculate_pixel_index(image, (x as i32 - 1, y as i32 - 1), iteration, enhancement_algorithm[0])];
        }
    }
    new_image
}


fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .split("\n\n")
        .collect();

    // Get the algorithm from the input
    let enhancement_algorithm: Vec<_> = input[0].chars()
        .filter_map(|c| Pixel::from_char(c))
        .collect();
    debug_assert_eq!(enhancement_algorithm.len(), 512);

    // Get the input image from the input
    let mut input_image: Vec<_> = input[1].lines()
        .map(|s| {
            s.chars()
                .map(|c| Pixel::from_char(c).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    // Do two enhancement passes
    for i in 0..ENHANCEMENTS {
        input_image = enhancement_pass(&enhancement_algorithm, &input_image, i + 1);
    }

    let lit_pixels: usize = input_image.iter()
        .map(|row| {
            row.iter()
                .filter(|p| **p == Pixel::Light)
                .count()
        })
        .sum();
    println!("Lit pixels: {}", lit_pixels);
}
