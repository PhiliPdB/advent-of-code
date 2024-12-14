use std::{collections::VecDeque, str::FromStr};


#[derive(Debug, Clone)]
struct FileSystem {
    free_spaces: VecDeque<(usize, u32)>, // (index, length)
    files: Vec<(usize, u32, u32)>, // (index, length, id)
}

impl FileSystem {
    fn compact(&mut self) {
        let mut queue = Vec::from_iter(0..self.files.len());
        while let Some(i) = queue.pop() {
            if self.free_spaces.is_empty() {
                break;
            }

            let file = &mut self.files[i];
            let mut first_free_space = self.free_spaces.pop_front().unwrap();
            if file.0 < first_free_space.0 {
                break;
            }

            // Check if the file can fit in the free space
            if file.1 <= first_free_space.1 {
                // Update file
                file.0 = first_free_space.0;

                // Update free space
                first_free_space.0 += file.1 as usize;
                first_free_space.1 -= file.1;

                // Reinsert free space if there is remaining space
                if first_free_space.1 > 0 {
                    self.free_spaces.push_front(first_free_space);
                }
            } else {
                let file_index = file.0;
                let remaining_size = file.1 - first_free_space.1;
                let file_id = file.2;

                // Update file
                file.0 = first_free_space.0;
                file.1 = first_free_space.1;

                // Add a new file
                let file = (file_index, remaining_size, file_id);
                self.files.push(file);
                // Push index tot the queue
                queue.push(self.files.len() - 1);
            }
        }
    }

    fn compact_whole_file(&mut self) {
        for i in (0..self.files.len()).rev() {
            if self.free_spaces.is_empty() {
                break;
            }

            let file = &mut self.files[i];
            let Some(free_space_index) = self.free_spaces.iter()
                .position(|(index, length)| {
                    *index < file.0 && *length >= file.1
                }) else {
                    continue;
                };
            let free_space = self.free_spaces[free_space_index];

            // Update file
            file.0 = free_space.0;
            if file.1 < free_space.1 {
                // Update free space
                let mut_free_space = &mut self.free_spaces[free_space_index];
                mut_free_space.0 += file.1 as usize;
                mut_free_space.1 -= file.1;
            } else { // File size equals the free space size
                // So remove the free space
                self.free_spaces.remove(free_space_index);
            }
        }
    }


    fn checksum(&self) -> u64 {
        self.files.iter()
            .map(|&(index, length, id)| {
                (0..length).map(|i| (index as u64 + i as u64) * id as u64).sum::<u64>()
            })
            .sum()
    }
}

impl FromStr for FileSystem {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let compact_representation: Vec<_> = s
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        let mut free_spaces = VecDeque::new();
        let mut files = Vec::new();

        let mut i = 0;
        for (id, c) in compact_representation.chunks(2).enumerate() {
            files.push((i, c[0], id as u32));
            i += c[0] as usize;

            if c.len() > 1 {
                free_spaces.push_back((i, c[1]));
                i += c[1] as usize;
            }
        }

        Ok(FileSystem {
            free_spaces,
            files,
        })
    }
}


fn main() {
    let filesystem = FileSystem::from_str(include_str!("../input.txt")).unwrap();


    let mut part1_filesystem = filesystem.clone();
    part1_filesystem.compact();
    println!("[Part 1] Checksum: {}", part1_filesystem.checksum());


    let mut part2_filesystem = filesystem;
    part2_filesystem.compact_whole_file();
    println!("[Part 2] Checksum: {}", part2_filesystem.checksum());
}
