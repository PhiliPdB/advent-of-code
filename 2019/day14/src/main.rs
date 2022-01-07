use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Chemical<'a> {
    amount: u64,
    name: &'a str,
}

impl<'a> Chemical<'a> {
    pub fn new(amount: u64, name: &'a str) -> Self {
        Self { amount, name }
    }

    // Can't implement the trait because of the lifetime requirements
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &'a str) -> Result<Self, &'static str> {
        let (amount, name) = s.split_once(' ').ok_or("Invalid chemical format")?;
        let amount = amount.parse().map_err(|_| "Can't parse chemical amount")?;

        Ok(Self::new(amount, name))
    }
}

fn get_ore_requirement<'a>(
    reactions: &HashMap<&'a str, (Chemical<'a>, Vec<Chemical<'a>>)>, mut left_over: HashMap<&'a str, u64>,
    chemical: &'a str, amount: u64
) -> (u64, HashMap<&'a str, u64>) {
    if chemical == "ORE" {
        return (amount, left_over);
    }

    let (out, ingredients) = &reactions[&chemical];
    let required_reactions = (amount as f64 / out.amount as f64).ceil() as u64;
    left_over.insert(chemical, out.amount * required_reactions - amount);

    ingredients.iter()
        .fold((0, left_over), |(acc, mut left_over), c| {
            let l = left_over.remove(c.name).unwrap_or(0);
            let required_amount = c.amount * required_reactions;
            if l > required_amount {
                // Already have everything
                left_over.insert(c.name, l - required_amount);
                (acc, left_over)
            } else {
                // Need to produce more
                let (c, new_left_over) = get_ore_requirement(reactions, left_over, c.name,  required_amount - l);
                (c + acc, new_left_over)
            }
        })
}

fn main() {
    let input: HashMap<_, _> = include_str!("../input.txt")
        .lines()
        .map(|s| {
            let (ingredients, output) = s.split_once(" => ").unwrap();
            let output = Chemical::from_str(output).unwrap();
            let ingredients: Vec<_> = ingredients.split(", ")
                .map(|i| Chemical::from_str(i).unwrap())
                .collect();

            (output.name, (output, ingredients))
        })
        .collect();

    // Part 1

    let (ore_requirement, _) = get_ore_requirement(&input, HashMap::new(), "FUEL", 1);
    println!("ORE requirement: {}", ore_requirement);

    // Part 2
    const ORE_STOCK: u64 = 1_000_000_000_000;

    // Binary search
    let mut start = ORE_STOCK / ore_requirement;
    let mut end = ORE_STOCK;
    while start < end - 1 {
        let m = (start + end) / 2;

        let (ore_requirement, _) = get_ore_requirement(&input, HashMap::new(), "FUEL", m);
        if ore_requirement > ORE_STOCK {
            end = m - 1;
        } else {
            start = m;
        }
    }

    println!("Can make {} FUEL", start);
}
