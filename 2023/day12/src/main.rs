use std::str::FromStr;


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
    pub fn arrangements(&self) -> u64 {
        Self::arrangements_dp(&self.status, &self.groups)
    }

    pub fn unfolded_arrangements(&self) -> u64 {
        let mut status = self.status.clone();
        status.push(Status::Unknown);
        let status = status.repeat(5);
        let status = &status[..status.len() - 1];

        Self::arrangements_dp(status, &self.groups.repeat(5))
    }

    fn arrangements_dp(status: &[Status], groups: &[u32]) -> u64 {
        let mut dp_table = Vec::with_capacity(groups.len() + 1);
        for _ in 0..(groups.len() + 1) {
            dp_table.push(vec![0; status.len() + 1]);
        }
        
        dp_table[groups.len()][status.len()] = 1;
        for i in (0..status.len()).rev() {
            dp_table[groups.len()][i] = 
                if status[i] == Status::Damaged {
                    0
                } else {
                    dp_table[groups.len()][i + 1]
                };
        }

        for group_index in (0..groups.len()).rev() {
            for status_index in (0..status.len()).rev() {
                if groups[group_index..].iter().sum::<u32>() as usize + (groups[group_index..].len() - 1) > status[status_index..].len() {
                    // Note table was initialized on 0
                    continue;
                }

                dp_table[group_index][status_index] =
                    match status[status_index] {
                        Status::Operational => dp_table[group_index][status_index + 1],
                        Status::Damaged => {
                            let next_group_size = groups[group_index] as usize;
                            let fits = status[status_index..(status_index+next_group_size)].iter()
                                .all(|s| *s != Status::Operational);

                            // Next fits
                            if fits && next_group_size == status[status_index..].len() {
                                dp_table[group_index + 1][status_index + next_group_size]
                            } else if fits && status[status_index + next_group_size] != Status::Damaged {
                                dp_table[group_index + 1][status_index + next_group_size + 1]
                            } else {
                                0
                            }
                        },
                        Status::Unknown => {
                            let do_nothing = dp_table[group_index][status_index + 1];

                            let next_group_size = groups[group_index] as usize;
                            let fits = status[status_index..(status_index+next_group_size)].iter()
                                .all(|s| *s != Status::Operational);
                            let do_something =
                                if fits && next_group_size == status[status_index..].len() {
                                    dp_table[group_index + 1][status_index + next_group_size]
                                } else if fits && status[status_index + next_group_size] != Status::Damaged {
                                    dp_table[group_index + 1][status_index + next_group_size + 1]
                                } else {
                                    0
                                };


                            do_nothing + do_something
                        },
                    }
            }
        }

        dp_table[0][0]
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


    let part1_arrangements: u64 = records.iter()
        .map(|r| r.arrangements())
        .sum();
    println!("[Part 1] Total arrangements: {part1_arrangements:13}");


    let part2_arrangements: u64 = records.iter()
        .map(|r| r.unfolded_arrangements())
        .sum();
    println!("[Part 2] Total arrangements: {part2_arrangements:13}");
}
