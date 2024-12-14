use std::str::FromStr;

use crate::Node;
use crate::space::Space;

pub trait Map {
    fn distance_to_room(hallway_index: usize, room_index: usize) -> u32 {
        debug_assert!(room_index < 4);

        (hallway_index as i32 - ((room_index + 1) * 2) as i32).unsigned_abs()
    }

    fn is_blocking(&self, hallway_index: usize, room_index: usize) -> bool;

    fn is_finished(&self) -> bool;

    fn generate_moves(&self) -> Vec<Node<Self>> where Self: Sized;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Map2 {
    hallway: [Space; 11],
    rooms: [[Space; 2]; 4],
}

impl Map for Map2 {

    fn is_blocking(&self, hallway_index: usize, room_index: usize) -> bool {
        debug_assert!(room_index < 4);

        let room_index = (room_index + 1) * 2;
        if hallway_index < room_index {
            self.hallway[(hallway_index + 1)..(room_index + 1)].iter().any(|s| *s != Space::Open)
        } else {
            self.hallway[room_index..hallway_index].iter().any(|s| *s != Space::Open)
        }
    }

    fn is_finished(&self) -> bool {
        self.rooms[0][0] == Space::Amber && self.rooms[0][1] == Space::Amber
            && self.rooms[1][0] == Space::Bronze && self.rooms[1][1] == Space::Bronze
            && self.rooms[2][0] == Space::Copper && self.rooms[2][1] == Space::Copper
            && self.rooms[3][0] == Space::Desert && self.rooms[3][1] == Space::Desert
    }

    fn generate_moves(&self) -> Vec<Node<Self>> {
        let mut moves = Vec::new();

        // Can we move from the hallway back to a room?
        for (i, space) in self.hallway.iter().enumerate() {
            match space {
                Space::Open => continue,
                _ => {
                    // Check if there is nothing blocking us
                    if self.is_blocking(i, space.room_index()) {
                        continue;
                    }

                    let room = &self.rooms[space.room_index()];

                    // Check which slot we should move to
                    let room_slot =
                        if room[0] == Space::Open {
                            debug_assert_eq!(room[1], Space::Open);

                            0
                        } else if room[1] == Space::Open && room[0] == *space {
                            1
                        } else {
                            continue;
                        };

                    let mut new_map = *self;
                    new_map.hallway[i] = Space::Open;
                    new_map.rooms[space.room_index()][room_slot] = *space;

                    moves.push(Node(new_map, (Self::distance_to_room(i, space.room_index()) + (2 - room_slot) as u32) * space.multiplier()));
                }
            }
        }

        // Generate hallway moves
        for hallway_index in [0, 1, 3, 5, 7, 9, 10] {
            if self.hallway[hallway_index] != Space::Open {
                continue;
            }

            for room in 0..4 {
                if self.is_blocking(hallway_index, room) {
                    continue;
                }

                // Check from which slot in the room we should move
                let room_slot =
                    if self.rooms[room][1] == Space::Open && self.rooms[room][0] != Space::Open {
                        0
                    } else if self.rooms[room][1] != Space::Open {
                        1
                    } else {
                        continue;
                    };

                let mut new_map = *self;
                new_map.hallway[hallway_index] = self.rooms[room][room_slot];
                new_map.rooms[room][room_slot] = Space::Open;

                moves.push(Node(new_map, (Self::distance_to_room(hallway_index, room) + (2 - room_slot) as u32) * self.rooms[room][room_slot].multiplier()));
            }
        }

        moves
    }
}

impl FromStr for Map2 {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Map4 {
    hallway: [Space; 11],
    rooms: [[Space; 4]; 4],
}

impl Map for Map4 {

    fn is_blocking(&self, hallway_index: usize, room_index: usize) -> bool {
        debug_assert!(room_index < 4);

        let room_index = (room_index + 1) * 2;
        if hallway_index < room_index {
            self.hallway[(hallway_index + 1)..(room_index + 1)].iter().any(|s| *s != Space::Open)
        } else {
            self.hallway[room_index..hallway_index].iter().any(|s| *s != Space::Open)
        }
    }

    fn is_finished(&self) -> bool {
        self.rooms[0][0] == Space::Amber && self.rooms[0][1] == Space::Amber && self.rooms[0][2] == Space::Amber && self.rooms[0][3] == Space::Amber
            && self.rooms[1][0] == Space::Bronze && self.rooms[1][1] == Space::Bronze && self.rooms[1][2] == Space::Bronze && self.rooms[1][3] == Space::Bronze
            && self.rooms[2][0] == Space::Copper && self.rooms[2][1] == Space::Copper && self.rooms[2][2] == Space::Copper && self.rooms[2][3] == Space::Copper
            && self.rooms[3][0] == Space::Desert && self.rooms[3][1] == Space::Desert && self.rooms[3][2] == Space::Desert && self.rooms[3][3] == Space::Desert
    }

    fn generate_moves(&self) -> Vec<Node<Self>> {
        let mut moves = Vec::new();

        // Can we move from the hallway back to a room?
        for (i, space) in self.hallway.iter().enumerate() {
            match space {
                Space::Open => continue,
                _ => {
                    // Check if there is nothing blocking us
                    if self.is_blocking(i, space.room_index()) {
                        continue;
                    }

                    let room = &self.rooms[space.room_index()];
                    // Check which slot we can move to
                    let room_slot =
                        if room[0] == Space::Open {
                            debug_assert_eq!(room[1], Space::Open);
                            debug_assert_eq!(room[2], Space::Open);
                            debug_assert_eq!(room[3], Space::Open);

                            0
                        } else if room[1] == Space::Open && room[0] == *space {
                            debug_assert_eq!(room[2], Space::Open);
                            debug_assert_eq!(room[3], Space::Open);

                            1
                        } else if room[2] == Space::Open && room[1] == *space && room[0] == *space {
                            debug_assert_eq!(room[3], Space::Open);

                            2
                        } else if room[3] == Space::Open && room[2] == *space && room[1] == *space && room[0] == *space {
                            3
                        } else {
                            continue;
                        };

                        let mut new_map = *self;
                        new_map.hallway[i] = Space::Open;
                        new_map.rooms[space.room_index()][room_slot] = *space;

                        moves.push(Node(new_map, (Self::distance_to_room(i, space.room_index()) + (4 - room_slot) as u32) * space.multiplier()));
                }
            }
        }

        // Generate hallway moves
        for hallway_index in [0, 1, 3, 5, 7, 9, 10] {
            if self.hallway[hallway_index] != Space::Open {
                continue;
            }

            for room in 0..4 {
                if self.is_blocking(hallway_index, room) {
                    continue;
                }

                // Check from which slot in the room we should move
                let room_slot =
                    if self.rooms[room][3] == Space::Open && self.rooms[room][2] == Space::Open && self.rooms[room][1] == Space::Open && self.rooms[room][0] != Space::Open {
                        0
                    } else if self.rooms[room][3] == Space::Open && self.rooms[room][2] == Space::Open && self.rooms[room][1] != Space::Open {
                        1
                    } else if self.rooms[room][3] == Space::Open && self.rooms[room][2] != Space::Open {
                        2
                    } else if self.rooms[room][3] != Space::Open {
                        3
                    } else {
                        continue;
                    };

                let mut new_map = *self;
                new_map.hallway[hallway_index] = self.rooms[room][room_slot];
                new_map.rooms[room][room_slot] = Space::Open;

                moves.push(Node(new_map, (Self::distance_to_room(hallway_index, room) + (4 - room_slot) as u32) * self.rooms[room][room_slot].multiplier()));
            }
        }

        moves
    }
}

impl FromStr for Map4 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hallway = [Space::Open; 11];
        let mut rooms = [[Space::Open; 4]; 4];

        let l: Vec<_> = s.lines().collect();
        let first_room_line: Vec<_> = l[2].chars().collect();
        let second_room_line: Vec<_> = l[3].chars().collect();
        let third_room_line: Vec<_> = l[4].chars().collect();
        let fourth_room_line: Vec<_> = l[5].chars().collect();

        for (i, r) in [3, 5, 7, 9].into_iter().enumerate() {
            rooms[i][0] = Space::from_char(fourth_room_line[r])?;
            rooms[i][1] = Space::from_char(third_room_line[r])?;
            rooms[i][2] = Space::from_char(second_room_line[r])?;
            rooms[i][3] = Space::from_char(first_room_line[r])?;
        }

        Ok(Self { hallway, rooms })
    }
}
