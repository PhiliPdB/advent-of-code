use intcode::Program;
use itertools::Itertools;

mod intcode;

fn main() {
    let mut program = Program::new(
        include_str!("../input.txt")
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect()
    );

    let collect_commands = [
        // Hull
        "east",
        // Engineering
        "east",
        // Warp drive maintenance
        "take semiconductor",
        "north",
        // Gift wrapping center
        "take planetoid",
        "west",
        // Hot chocolate fountain
        "take food ration",
        "west",
        "west",
        // Arcade
        "take monolith",
        "east",
        "east",
        // Hot chocolate fountain
        "north",
        // Sick bay
        "take space law space brochure",
        "north",
        "north",
        // Kitchen
        "take weather machine",
        "south",
        "south",
        // Sick bay
        "east",
        // Crew quarters
        "take jam",
        "west",
        // Sick bay
        "south",
        // Hot chocolate fountain
        "east",
        // Gift wrapping center
        "north",
        // Stables
        "take antenna",
        "south",
        "south",
        // Warp drive maintenance
        // Now going to security
        "east",
        "south",
        "south",
        "east",
        // Drop all items in inventory
        "drop antenna",
        "drop food ration",
        "drop jam",
        "drop monolith",
        "drop planetoid",
        "drop semiconductor",
        "drop space law space brochure",
        "drop weather machine",
        ""
    ];

    // Walk to the security checkpoint while collecting all items
    program.run(
        collect_commands.join("\n")
            .chars()
            .map(|c| c as u8)
    );

    let all_items = [
        "antenna",
        "food ration",
        "jam",
        "monolith",
        "planetoid",
        "semiconductor",
        "space law space brochure",
        "weather machine",
    ];

    // Brute force search to find the correct item combination
    for items in all_items.into_iter().powerset() {
        let commands = items.iter()
            .map(|i| format!("take {i}"))
            .chain([String::from("east"), String::new()])
            .join("\n");

        if let Some(code) = program.run(commands.chars().map(|c| c as u8)) {
            println!("Security code: {code}");
            break;
        }

        // Drop the items
        let drop_commands = items.iter()
            .map(|i| format!("drop {i}"))
            .chain([String::new()])
            .join("\n");
        program.run(drop_commands.chars().map(|c| c as u8));
    }

}
