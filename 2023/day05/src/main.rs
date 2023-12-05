use std::str::FromStr;


struct Mapping {
    from_start: u64,
    to_start: u64,
    length: u64,
}

impl Mapping {
    fn convert(&self, from: u64) -> Option<u64> {
        if self.from_start <= from && from < self.from_start + self.length {
            Some(self.to_start + (from - self.from_start))
        } else {
            None
        }
    }

    fn convert_back(&self, to: u64) -> Option<u64> {
        if self.to_start <= to && to < self.to_start + self.length {
            Some(self.from_start + (to - self.to_start))
        } else {
            None
        }
    }
}

impl FromStr for Mapping {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [to, from, length] = s.split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect::<Vec<_>>().try_into()
            .map_err(|_| "Could not convert mapping")?;

        Ok(Self { from_start: from, to_start: to, length })
    }
}

fn have_seed(location: u64, mappings: &[Vec<Mapping>], seeds: &[u64]) -> bool {
    let mut seed = location;
    for m in mappings.iter().rev() {
        let mapping = m.iter().find(|m| m.convert_back(seed).is_some());
        if let Some(mapping) = mapping {
            seed = mapping.convert_back(seed).unwrap();
        }
    }

    seeds.chunks(2)
        .any(|c| c[0] <= seed && seed < c[0] + c[1])
}

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .split("\n\n")
        .collect();

    let seeds: Vec<u64> = input[0][7..].split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mappings: Vec<_> = input.into_iter().skip(1)
        .map(|s| {
            s.lines().skip(1)
                .map(|l| Mapping::from_str(l).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    // Part 1
    let mut locations = seeds.clone();
    for m in &mappings {
        for location in locations.iter_mut() {
            let mapping = m.iter().find(|m| m.convert(*location).is_some());
            if let Some(mapping) = mapping {
                *location = mapping.convert(*location).unwrap();
            }
        }
    }
    println!("[Part 1] Lowest location number: {:9}", locations.iter().min().unwrap());

    // Binary search for min location value
    let mut lower =  10_000_000;
    let mut upper = 100_000_000;
    while lower < upper {
        let m = (upper + lower) / 2;

        if have_seed(m, &mappings, &seeds) {
            upper = m;
        } else {
            lower = m + 1;
        }
    }
    debug_assert!(have_seed(upper, &mappings, &seeds));
    println!("[Part 2] Lowest location number: {upper:9}");
}
