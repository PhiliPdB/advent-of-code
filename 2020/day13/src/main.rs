fn main() {
    let input: Vec<_> = include_str!("../input.txt").lines().collect();

    let earliest_bus = input[0].parse::<u64>().unwrap();
    let mut bus_ids: Vec<_> = input[1].split(',')
        .enumerate()
        .filter_map(|(i, s)| {
            let bus_id = s.parse::<u64>();
            if let Ok(bus_id) = bus_id {
                Some((i as u64, bus_id))
            } else {
                None
            }
        })
        .collect();

    // Part 1

    let next_bus = bus_ids.iter()
        .map(|(_, bus_id)| (*bus_id, *bus_id * ((earliest_bus / *bus_id) + 1)))
        .min_by(|(_, a1), (_, a2)| a1.cmp(a2))
        .unwrap();

    println!("Next bus: {}", next_bus.0 * (next_bus.1 - earliest_bus));

    // Part 2
    // Apply Chinese Remainder Theorem

    // Sort by biggest bus id
    bus_ids.sort_unstable_by(|(_, id1), (_, id2)| id2.cmp(id1));
    bus_ids = bus_ids.into_iter()
        // Convert the offsets into a remainder (a_i)
        .map(|(offset, id)| ((-(offset as i64)).rem_euclid(id as i64) as u64, id))
        .collect();

    let mut current_x = bus_ids[0].0;
    let mut current_factor = bus_ids[0].1;
    let mut next_index = 1;
    while next_index < bus_ids.len() {
        current_x += current_factor;

        if current_x % bus_ids[next_index].1 == bus_ids[next_index].0 {
            current_factor *= bus_ids[next_index].1;
            next_index += 1;
        }
    }

    println!("Earliest time: {}", current_x);
}
