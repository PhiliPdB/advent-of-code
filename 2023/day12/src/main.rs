use std::str::FromStr;

use hashbrown::HashMap;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Status {
    Operational, Damaged, Unknown,
}

impl Status {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!("Invalid status char: {c}"),
        }
    }
}

#[derive(Debug)]
struct Record {
    status: Vec<Status>,
    groups: Vec<u32>,
}

impl Record {
    pub fn arrangements(&self, cache: &mut HashMap<(Vec<Status>, Vec<u32>), u64>) -> u64 {
        Self::calculate_arrangements(cache, &self.status, &self.groups)
    }

    pub fn unfolded_arrangements(&self, cache: &mut HashMap<(Vec<Status>, Vec<u32>), u64>) -> u64 {
        let mut status = self.status.clone();
        status.push(Status::Unknown);
        let status = status.repeat(5);
        let status = &status[..status.len() - 1];

        Self::calculate_arrangements(cache, status, &self.groups.repeat(5))
    }

    fn calculate_arrangements(cache: &mut HashMap<(Vec<Status>, Vec<u32>), u64>, status: &[Status], groups: &[u32]) -> u64 {
        let s = status.to_owned();
        let g = groups.to_owned();
        if let Some(a) = cache.get(&(s, g)) {
            return *a;
        }

        if groups.is_empty() {
            if status.iter().all(|s| matches!(s, Status::Operational|Status::Unknown)) {
                return 1;
            }

            return 0;
        }
        if status.is_empty() {
            return 0;
        }
        if status.len() < groups.iter().sum::<u32>() as usize + groups.len() - 1 {
            return 0;
        }

        match status[0] {
            Status::Operational => {
                Self::calculate_arrangements(cache, &status[1..], &groups)
            },
            Status::Damaged => {
                if status.len() < groups[0] as usize || status[..(groups[0] as usize)].iter().any(|s| *s == Status::Operational)
                    || (status.len() > groups[0] as usize && status[groups[0] as usize] == Status::Damaged)
                {
                    0
                } else {
                    let new_status = if status.len() == groups[0] as usize {
                        &status[(groups[0] as usize)..]
                    } else {
                        &status[(groups[0] as usize) + 1..]
                    };

                    Self::calculate_arrangements(cache, new_status, &groups[1..])
                }
            },
            Status::Unknown => {
                let do_nothing = Self::calculate_arrangements(cache, &status[1..], &groups);
                let do_something = if status.len() >= groups[0] as usize && status[..(groups[0] as usize)].iter().all(|s| *s != Status::Operational)
                    && (status.len() == groups[0] as usize || status[groups[0] as usize] != Status::Damaged)
                {
                    let new_status = if status.len() == groups[0] as usize {
                        &status[(groups[0] as usize)..]
                    } else {
                        &status[(groups[0] as usize) + 1..]
                    };

                    Self::calculate_arrangements(cache, new_status, &groups[1..])
                } else {
                    0
                };

                let s = status.to_owned();
                let g = groups.to_owned();
                cache.insert((s, g), do_nothing + do_something);
                do_nothing + do_something
            },
        }
    }
}

impl FromStr for Record {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [status, groups] = s.split(' ')
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| "Invalid format")?;

        let status = status.chars()
            .map(Status::from_char)
            .collect();

        let groups = groups.split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        Ok(Self { status, groups })
    }
}


fn main() {
    let records: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|l| Record::from_str(l).unwrap())
        .collect();

    let mut cache = HashMap::new();


    let part1_arrangements: u64 = records.iter()
        .map(|r| r.arrangements(&mut cache))
        .sum();
    println!("[Part 1] Total arrangements: {part1_arrangements:13}");


    let part2_arrangements: u64 = records.iter()
        .map(|r| r.unfolded_arrangements(&mut cache))
        .sum();
    println!("[Part 2] Total arrangements: {part2_arrangements:13}");
}
