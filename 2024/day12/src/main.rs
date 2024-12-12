use std::str::FromStr;
use std::collections::HashSet;



#[derive(Debug)]
struct Garden {
    map: Vec<Vec<char>>,
}

impl Garden {
    pub fn regions(&self) -> Vec<(u32, u32, u32)> {
        let mut regions = Vec::new();

        let mut visited = vec![vec![false; self.map[0].len()]; self.map.len()];
        for y in 0..self.map.len() {
            for x in 0..self.map[0].len() {
                if visited[y][x] {
                    continue;
                }

                let region_size = self.region_size((x, y), &mut visited);
                regions.push(region_size);
            }
        }

        regions
    }

    fn region_size(&self, (start_x, start_y): (usize, usize), visited: &mut Vec<Vec<bool>>) -> (u32, u32, u32) {
        let mut queue = vec![(start_x, start_y)];
        let mut area = 0;
        let mut perimeter = 0;
        let mut sides = HashSet::new();

        let plot_char = self.map[start_y][start_x];
        while let Some((x, y)) = queue.pop() {
            if visited[y][x] {
                continue;
            }

            visited[y][x] = true;
            area += 1;

            if x > 0 && self.map[y][x - 1] == plot_char {
                queue.push((x - 1, y));
            } else {
                perimeter += 1;

                self.update_sides(&mut sides, (x, y), true, -1);
            }
            if x < self.map[0].len() - 1 && self.map[y][x + 1] == plot_char {
                queue.push((x + 1, y));
            } else {
                perimeter += 1;

                self.update_sides(&mut sides, (x, y), true, 1);
            }

            if y > 0 && self.map[y - 1][x] == plot_char {
                queue.push((x, y - 1));
            } else {
                perimeter += 1;

                self.update_sides(&mut sides, (x, y), false, -1);
            }
            if y < self.map.len() - 1 && self.map[y + 1][x] == plot_char {
                queue.push((x, y + 1));
            } else {
                perimeter += 1;

                self.update_sides(&mut sides, (x, y), false, 1);
            }
        }

        (area, perimeter, sides.len() as u32)
    }

    /// Find the side a boundary belongs to and insert it to the HashSet of sides
    fn update_sides(&self,
        sides: &mut HashSet<((i32, i32), (i32, i32))>,
        (x, y): (usize, usize),
        vertical: bool, boundary_step: i32,
    ) {
        let plot_char = self.map[y][x];

        let (mut x, mut y) = (x, y);
        let boundary_coord;
        if vertical {
            let boundary_x = x as i32 + boundary_step;
            let width = self.map[0].len() as i32;

            while y > 0 && self.map[y - 1][x] == plot_char
                && (boundary_x < 0 || boundary_x >= width || self.map[y - 1][boundary_x as usize] != plot_char)
            {
                y -= 1;
            }

            boundary_coord = (boundary_x, y as i32);
        } else {
            let boundary_y = y as i32 + boundary_step;
            let height = self.map.len() as i32;

            while x > 0 && self.map[y][x - 1] == plot_char
                && (boundary_y < 0 || boundary_y >= height || self.map[boundary_y as usize][x - 1] != plot_char)
            {
                x -= 1;
            }

            boundary_coord = (x as i32, boundary_y);
        }

        sides.insert(((x as i32, y as i32), boundary_coord));
    }
}

impl FromStr for Garden {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Garden {
            map: s.lines().map(|l| l.chars().collect()).collect(),
        })
    }
}


fn main() {
    let garden = Garden::from_str(include_str!("../input.txt")).unwrap();
    let regions = garden.regions();


    let fencing_cost: u32 = regions.iter()
        .map(|&(area, perimeter, _)| area * perimeter)
        .sum();
    println!("[Part 1] Total fencing cost: {fencing_cost}");


    let discounted_fencing_cost: u32 = regions.iter()
        .map(|&(area, _, sides)| area * sides)
        .sum();
    println!("[Part 2] Discounted fencing cost: {discounted_fencing_cost}");
}
