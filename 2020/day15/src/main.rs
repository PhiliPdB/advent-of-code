const TOTAL_TURNS: u32 = 30_000_000;

fn main() {
    let input = [2, 20, 0, 4, 1, 17];

    let mut spoken = vec![None; TOTAL_TURNS as usize];
    for (i, n) in input.iter().enumerate() {
        spoken[*n] = Some(i as u32);
    }

    let mut next_number = 0;
    for turn in (input.len() as u32 + 1)..TOTAL_TURNS {
        if let Some(last_turn) = spoken[next_number] {
            spoken[next_number] = Some(turn);
            next_number = (turn - last_turn) as usize;
        } else {
            spoken[next_number] = Some(turn);
            next_number = 0;
        };
    }

    println!("Number: {}", next_number);
}
