use std::str::FromStr;
use std::collections::HashSet;

use itertools::iproduct;


#[derive(Debug)]
struct Map {
    map: Vec<Vec<u32>>,
}

impl Map {
    pub fn trailheads<const COUNT_DISTINCT: bool>(&self) -> impl Iterator<Item = ((usize, usize), u32)> + '_ {
        iproduct!(0..self.map[0].len(), 0..self.map.len())
            .filter_map(|(x, y)| {
                let num_trails = self.num_trails::<COUNT_DISTINCT>((x, y));
                if num_trails > 0 {
                    Some(((x, y), num_trails))
                } else {
                    None
                }
            })
    }

    fn num_trails<const COUNT_DISTINCT: bool>(&self, (start_x, start_y): (usize, usize)) -> u32 {
        if self.map[start_y][start_x] != 0 {
            return 0;
        }

        let mut num_trails = 0;
        let mut queue = vec![((start_x, start_y), 0)];
        let mut visited = HashSet::new();
        while let Some(((x, y), h)) = queue.pop() {
            if h == 9 {
                if COUNT_DISTINCT || visited.insert((x, y)) {
                    num_trails += 1;
                }
                continue;
            }

            if x > 0 && self.map[y][x - 1] == h + 1 {
                queue.push(((x - 1, y), h + 1));
            }
            if x < self.map[0].len() - 1 && self.map[y][x + 1] == h + 1 {
                queue.push(((x + 1, y), h + 1));
            }
            if y > 0 && self.map[y - 1][x] == h + 1 {
                queue.push(((x, y - 1), h + 1));
            }
            if y < self.map.len() - 1 && self.map[y + 1][x] == h + 1 {
                queue.push(((x, y + 1), h + 1));
            }
        }

        num_trails
    }
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s.lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).ok_or("Invalid digit in input"))
                    .collect::<Result<Vec<_>,_>>()
            })
            .collect::<Result<Vec<_>,_>>()?;

        Ok(Map { map })
    }
}


fn main() {
    let topographic_map = Map::from_str(include_str!("../input.txt")).unwrap();


    let part1_trailhead_sum: u32 = topographic_map.trailheads::<false>()
        .map(|(_, num_trails)| num_trails)
        .sum();
    println!("[Part 1] Sum of trailheads: {part1_trailhead_sum:4}");


    let part2_trailhead_sum: u32 = topographic_map.trailheads::<true>()
        .map(|(_, num_trails)| num_trails)
        .sum();
    println!("[Part 2] Sum of trailheads: {part2_trailhead_sum:4}");
}
