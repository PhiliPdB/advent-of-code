use std::str::FromStr;


#[derive(Debug)]
struct Operation {
    result: u64,
    values: Vec<u64>,
}

impl Operation {
    pub const fn result(&self) -> u64 {
        self.result
    }

    pub fn possible_solutions<const WITH_CONCAT: bool>(&self) -> u64 {
        self.possible_solutions_from::<WITH_CONCAT>(1, self.values[0])
    }

    /// Calculate the possible solutions from position `i` given the current accumulated result in `current_result`.
    fn possible_solutions_from<const WITH_CONCAT: bool>(&self, i: usize, current_result: u64) -> u64 {
        if i == self.values.len() {
            return if current_result == self.result {
                1
            } else {
                0
            };
        }

        if current_result > self.result {
            return 0;
        }

        let mut result =
            self.possible_solutions_from::<WITH_CONCAT>(
                i + 1, current_result + self.values[i]
            )
            + self.possible_solutions_from::<WITH_CONCAT>(
                i + 1, current_result * self.values[i]
            );
        if WITH_CONCAT {
            let concat_value =
                current_result * 10_u64.pow(self.values[i].ilog10() + 1)
                + self.values[i];
            result += self.possible_solutions_from::<WITH_CONCAT>(i + 1, concat_value);
        }
        result
    }
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (result, values) = s.split_once(": ")
            .ok_or("Cannot split on ': '")?;
        let result = result.parse()
            .map_err(|_| "Result could not be parsed")?;

        let values = values.split(' ')
            .map(|v| v.parse())
            .collect::<Result<Vec<_>,_>>()
            .map_err(|_| "Values could not be parsed")?;

        Ok(Self { result, values })
    }
}

fn main() {
    let operations: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|l| Operation::from_str(l).unwrap())
        .collect();


    let part1_calibration_result: u64 = operations.iter()
        .map(|o| {
            if o.possible_solutions::<false>() > 0 {
                o.result()
            } else {
                0
            }
        })
        .sum();
    println!("[Part 1] Calibration result: {part1_calibration_result:16}");


    let part2_calibration_result: u64 = operations.iter()
        .map(|o| {
            if o.possible_solutions::<true>() > 0 {
                o.result()
            } else {
                0
            }
        })
        .sum();
    println!("[Part 2] Calibration result: {part2_calibration_result:16}");
}
