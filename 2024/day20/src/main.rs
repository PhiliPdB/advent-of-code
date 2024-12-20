use std::str::FromStr;
use std::collections::VecDeque;

use hashbrown::HashSet;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
}

impl Tile {
    pub const fn from_char(c: char) -> Result<Self, &'static str> {
        match c {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Wall),
            _ => Err("Invalid tile"),
        }
    }
}

#[derive(Debug)]
struct RaceTrack {
    map: Vec<Vec<Tile>>,
    start: (usize, usize),
    finish: (usize, usize),
}

impl RaceTrack {
    fn shortest_path(&self) -> Vec<(usize, usize)> {
        let mut queue = VecDeque::new();
        queue.push_back((self.start, 0, (0, 0)));

        let mut visited = vec![vec![false; self.map[0].len()]; self.map.len()];

        let mut predecessor = vec![vec![(0, 0); self.map[0].len()]; self.map.len()];
        while let Some(((x, y), steps, previous)) = queue.pop_front() {
            if visited[y][x] {
                continue;
            }
            visited[y][x] = true;

            predecessor[y][x] = previous;
            if (x, y) == self.finish {
                break;
            }

            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let new_x = x as i32 + dx;
                let new_y = y as i32 + dy;

                if new_x < 0 || new_x >= self.map[0].len() as i32 {
                    continue;
                }
                if new_y < 0 || new_y >= self.map.len() as i32 {
                    continue;
                }

                let new_x = new_x as usize;
                let new_y = new_y as usize;

                if visited[new_y][new_x] {
                    continue;
                }

                if self.map[new_y][new_x] == Tile::Wall {
                    continue;
                }

                queue.push_back(((new_x, new_y), steps + 1, (x, y)));
            }
        }

        let mut path = Vec::new();
        let mut current = self.finish;
        while current != self.start {
            path.push(current);
            current = predecessor[current.1][current.0];
        }
        path.push(self.start);
        path.reverse();

        path
    }

    fn cheating_options(&self, shortest_path: &[(usize, usize)], cheat_length: usize, min_time_save: usize) -> u64 {
        let path_length = shortest_path.len() - 1;

        let mut distance_to_end = vec![vec![path_length + 10; self.map[0].len()]; self.map.len()];
        for (i, (x, y)) in shortest_path.iter().enumerate() {
            distance_to_end[*y][*x] = path_length - i;
        }

        let mut cheats = 0;
        // Find cheating options
        for (step, &(x, y)) in shortest_path.iter().enumerate() {
            let current_distance = path_length - step;
            if current_distance < min_time_save {
                break;
            }

            let mut queue = VecDeque::new();
            queue.push_back(((x, y), 0));
            let mut visited = HashSet::new();

            while let Some(((x, y), distance)) = queue.pop_front() {
                if distance > cheat_length {
                    continue;
                }

                if !visited.insert((x, y)) {
                    continue;
                }

                // Don't compare lengths when not back at a path
                if self.map[y][x] == Tile::Empty {
                    let new_distance = distance_to_end[y][x] + distance;
                    if current_distance > new_distance && current_distance - new_distance >= min_time_save {
                        cheats += 1;
                    }
                }

                for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    let new_x = x as i32 + dx;
                    let new_y = y as i32 + dy;

                    if new_x < 0 || new_x >= self.map[0].len() as i32 {
                        continue;
                    }
                    if new_y < 0 || new_y >= self.map.len() as i32 {
                        continue;
                    }

                    let new_x = new_x as usize;
                    let new_y = new_y as usize;

                    // Add to the queue
                    queue.push_back(((new_x, new_y), distance + 1));
                }
            }
        }

        cheats
    }
}

impl FromStr for RaceTrack {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Vec::new();
        let mut start = None;
        let mut finish = None;

        for (y, line) in s.lines().enumerate() {
            let mut row = Vec::new();

            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    start = Some((x, y));
                    row.push(Tile::Empty);
                    continue;
                } else if c == 'E' {
                    finish = Some((x, y));
                    row.push(Tile::Empty);
                    continue;
                }

                let tile = Tile::from_char(c)?;
                row.push(tile);
            }

            map.push(row);
        }

        Ok(Self {
            map,
            start: start.ok_or("No start found")?,
            finish: finish.ok_or("No finish found")?,
        })
    }
}


fn main() {
    let race_track = RaceTrack::from_str(include_str!("../input.txt")).unwrap();
    let shortest_path = race_track.shortest_path();


    // Part 1
    let part1_cheats = race_track.cheating_options(&shortest_path, 2, 100);
    println!("[Part 1]: Cheats saving at least 100ps: {part1_cheats:7}");


    // Part 2
    let part2_cheats = race_track.cheating_options(&shortest_path, 20, 100);
    println!("[Part 2]: Cheats saving at least 100ps: {part2_cheats:7}");
}
