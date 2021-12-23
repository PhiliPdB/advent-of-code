use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Node(Map, u32);

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Space {
    Open,
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Space {
    pub const fn multiplier(&self) -> u32 {
        match self {
            Space::Open   => panic!("Can't move an open space"),
            Space::Amber  =>    1,
            Space::Bronze =>   10,
            Space::Copper =>  100,
            Space::Desert => 1000,
        }
    }

    pub const fn room_index(&self) -> usize {
        match self {
            Space::Open   => panic!("No room"),
            Space::Amber  => 0,
            Space::Bronze => 1,
            Space::Copper => 2,
            Space::Desert => 3,
        }
    }

    pub fn from_char(c: char) -> Result<Self, &'static str> {
        match c {
            'A' => Ok(Space::Amber),
            'B' => Ok(Space::Bronze),
            'C' => Ok(Space::Copper),
            'D' => Ok(Space::Desert),
            '.' => Ok(Space::Open),
            _   => Err("Invalid space character"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Map {
    hallway: [Space; 11],
    rooms: [[Space; 2]; 4],
}

impl Map {
    pub const fn distance_to_room(hallway_index: usize, room_index: usize) -> u32 {
        debug_assert!(room_index < 4);

        (hallway_index as i32 - ((room_index + 1) * 2) as i32).abs() as u32
    }

    pub fn is_blocking(&self, hallway_index: usize, room_index: usize) -> bool {
        debug_assert!(room_index < 4);

        let room_index = (room_index + 1) * 2;
        if hallway_index < room_index {
            self.hallway[hallway_index..(room_index + 1)].iter().any(|s| *s != Space::Open)
        } else {
            self.hallway[room_index..(hallway_index + 1)].iter().any(|s| *s != Space::Open)
        }
    }

    pub fn is_finished(&self) -> bool {
        self.rooms[0][0] == Space::Amber && self.rooms[0][1] == Space::Amber
            && self.rooms[1][0] == Space::Bronze && self.rooms[1][1] == Space::Bronze
            && self.rooms[2][0] == Space::Copper && self.rooms[2][1] == Space::Copper
            && self.rooms[3][0] == Space::Desert && self.rooms[3][1] == Space::Desert
    }

    pub fn generate_moves(&self) -> Vec<Node> {
        let mut moves = Vec::new();

        // Can we move from the hallway back to a room?
        for (i, space) in self.hallway.iter().enumerate() {
            match space {
                Space::Open => continue,
                _ => {
                    let room = &self.rooms[space.room_index()];
                    // Check bottom room
                    if room[0] == Space::Open {
                        debug_assert_eq!(room[1], Space::Open);

                        let mut new_map = self.clone();
                        new_map.hallway[i] = Space::Open;
                        new_map.rooms[space.room_index()][0] = *space;

                        moves.push(Node(new_map, (Map::distance_to_room(i, space.room_index()) + 2) * space.multiplier()));
                    } else if room[1] == Space::Open && room[0] == *space {
                        // Can only move to the top room if the bottom room is filled with the correct item

                        let mut new_map = self.clone();
                        new_map.hallway[i] = Space::Open;
                        new_map.rooms[space.room_index()][1] = *space;

                        moves.push(Node(new_map, (Map::distance_to_room(i, space.room_index()) + 1) * space.multiplier()));
                    }
                }
            }
        }

        // Check if we can move directly from room to room
        // for room in 0..4 {
        //     let (space, dist, i) =
        //         if self.rooms[room][1] == Space::Open && self.rooms[room][0] != Space::Open {
        //             // Move from room 0
        //             (self.rooms[room][0], 2, 0)
        //         } else if self.rooms[room][1] != Space::Open {
        //             // Move from room 1
        //             (self.rooms[room][1], 1, 1)
        //         } else {
        //             continue;
        //         };

        //     let other_room = space.room_index();
        //     if other_room == room || self.is_blocking((room + 1) * 2, other_room) {
        //         continue;
        //     }

        //     let room_distance = Map::distance_to_room((room + 1) * 2, other_room);

        //     if self.rooms[other_room][0] == Space::Open {
        //         debug_assert_eq!(self.rooms[other_room][1], Space::Open);

        //         let mut new_map = self.clone();
        //         new_map.rooms[room][i] = Space::Open;
        //         new_map.rooms[other_room][0] = space;

        //         moves.push(Node(new_map, (room_distance + 2 + dist) * space.multiplier()));
        //     } else if self.rooms[other_room][1] == Space::Open && self.rooms[other_room][0] == space {
        //         // Can only move to the top room if the bottom room is filled with the correct item
        //         let mut new_map = self.clone();
        //         new_map.rooms[room][i] = Space::Open;
        //         new_map.rooms[other_room][1] = space;

        //         moves.push(Node(new_map, (room_distance + 1 + dist) * space.multiplier()));
        //     }
        // }

        // Generate hallway moves
        for hallway_index in [0, 1, 3, 5, 7, 9, 10] {
            if self.hallway[hallway_index] != Space::Open {
                continue;
            }

            for room in 0..4 {
                if self.is_blocking(hallway_index, room) {
                    continue;
                }

                if self.rooms[room][1] == Space::Open && self.rooms[room][0] != Space::Open {
                    // Move from room 0
                    let mut new_map = self.clone();
                    new_map.hallway[hallway_index] = self.rooms[room][0];
                    new_map.rooms[room][0] = Space::Open;

                    moves.push(Node(new_map, (Map::distance_to_room(hallway_index, room) + 2) * self.rooms[room][0].multiplier()));
                } else if self.rooms[room][1] != Space::Open {
                    // Move from room 1
                    let mut new_map = self.clone();
                    new_map.hallway[hallway_index] = self.rooms[room][1];
                    new_map.rooms[room][1] = Space::Open;

                    moves.push(Node(new_map, (Map::distance_to_room(hallway_index, room) + 1) * self.rooms[room][1].multiplier()));
                }
            }
        }

        moves
    }
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hallway = [Space::Open; 11];
        let mut rooms = [[Space::Open; 2]; 4];

        let l: Vec<_> = s.lines().collect();
        let first_room_line: Vec<_> = l[2].chars().collect();
        let second_room_line: Vec<_> = l[3].chars().collect();

        for (i, r) in [3, 5, 7, 9].into_iter().enumerate() {
            rooms[i][0] = Space::from_char(second_room_line[r])?;
            rooms[i][1] = Space::from_char(first_room_line[r])?;
        }

        Ok(Self { hallway, rooms })
    }
}

fn main() {
    let input = Map::from_str(include_str!("../input.txt")).unwrap();
    println!("{:?}", input);

    let mut visited = HashSet::new();
    let mut heap = BinaryHeap::new();
    heap.push(Node(input, 0));

    let mut score = None;

    while let Some(current_item) = heap.pop() {
        if current_item.0.is_finished() {
            score = Some(current_item.1);
            break;
        }

        if !visited.insert(current_item.0) {
            continue;
        }

        // Add new moves
        for mut new_move in current_item.0.generate_moves() {
            new_move.1 += current_item.1;
            heap.push(new_move);
        }
    }

    println!("{:?}", score);
}
