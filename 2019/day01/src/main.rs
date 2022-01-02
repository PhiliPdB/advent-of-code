fn main() {
    let module_mass: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // Part 1

    let simple_fuel_requirement: i32 = module_mass.iter()
        .map(|m| (m / 3) - 2)
        .sum();

    println!("[Part 1] Fuel requirement: {}", simple_fuel_requirement);

    // Part 2

    let fuel_requirement: i32 = module_mass.iter()
        .map(|m| {
            let mut m = *m;
            let mut fuel = 0;
            while m > 0 {
                let new_m = (m / 3) - 2;
                if new_m > 0 {
                    fuel += new_m;
                }
                m = new_m;
            }

            fuel
        })
        .sum();

    println!("[Part 2] Fuel requirement: {}", fuel_requirement);
}
