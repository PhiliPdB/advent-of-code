use std::str::FromStr;

#[derive(Debug)]
struct Game {
    id: u32,
    subsets: Vec<Color>,
}

#[derive(Debug)]
struct Color {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [game_id, subsets] = s.split(':').collect::<Vec<_>>().try_into()
            .map_err(|_| "Invalid game string")?;
        let game_id: u32 = game_id[5..].parse()
            .map_err(|_| "Unable to parse game id")?;

        let subsets: Vec<_> = subsets.split(';').collect();
        let mut parsed_subsets = Vec::with_capacity(subsets.len());

        for subset in subsets {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for color in subset.split(',') {
                let items: Vec<_> = color
                    .trim()
                    .split(' ')
                    .collect();

                let amount: u32 = items[0].parse()
                    .map_err(|_| "Unable to parse color amount")?;
                match items[1] {
                    "red" => red += amount,
                    "green" => green += amount,
                    "blue" => blue += amount,
                    _ => return Err("Unable to parse subset")
                }
            }

            parsed_subsets.push(Color { red, green, blue });
        }

        Ok(Game { id: game_id, subsets: parsed_subsets })
    }
}

fn main() {
    let games: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| Game::from_str(s).unwrap())
        .collect();

    let part1_sum: u32 = games.iter()
        .filter_map(|g| {
            if g.subsets.iter().all(|c| c.red <= 12 && c.green <= 13 && c.blue <= 14) {
                Some(g.id)
            } else {
                None
            }
        })
        .sum();
    println!("[Part 1] Sum of game ids: {part1_sum}");

    let power_sum: u32 = games.iter()
        .map(|g| {
            let max_red = g.subsets.iter().map(|c| c.red).max().unwrap();
            let max_green = g.subsets.iter().map(|c| c.green).max().unwrap();
            let max_blue = g.subsets.iter().map(|c| c.blue).max().unwrap();

            max_red * max_green * max_blue
        })
        .sum();
    println!("[Part 2] Sum of powers: {power_sum}");
}
