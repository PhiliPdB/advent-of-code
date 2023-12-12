use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    pub fn arrangements(&self) -> u32 {
        Self::calculate_arrangements(&self.status, &self.groups)
    }

    pub fn unfolded_arrangements(&self) -> u32 {
        let mut status = self.status.clone();
        status.push(Status::Unknown);
        let status = status.repeat(5);
        let status = &status[..status.len() - 1];

        Self::calculate_arrangements(status, &self.groups.repeat(5))
    }

    fn calculate_arrangements(status: &[Status], groups: &[u32]) -> u32 {
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
                Self::calculate_arrangements(&status[1..], &groups)
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

                    Self::calculate_arrangements(new_status, &groups[1..])
                }
            },
            Status::Unknown => {
                let do_nothing = Self::calculate_arrangements(&status[1..], &groups);
                let do_something = if status.len() >= groups[0] as usize && status[..(groups[0] as usize)].iter().all(|s| *s != Status::Operational)
                    && (status.len() == groups[0] as usize || status[groups[0] as usize] != Status::Damaged)
                {
                    let new_status = if status.len() == groups[0] as usize {
                        &status[(groups[0] as usize)..]
                    } else {
                        &status[(groups[0] as usize) + 1..]
                    };

                    Self::calculate_arrangements(new_status, &groups[1..])
                } else {
                    0
                };

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
    // println!("{records:?}");

    println!("{}", records[1].unfolded_arrangements());

    let part1_arrangements: u32 = records.iter()
        .map(|r| r.arrangements())
        .sum();
    println!("[Part 1] Total arrangements: {part1_arrangements}");


    let part2_arrangements: u32 = records.iter()
        .map(|r| {
            let a = r.unfolded_arrangements();
            println!("{r:?}: {a}");
            a
        })
        .sum();
    println!("[Part 1] Total arrangements: {part2_arrangements}");
}
