
const CARD_SUBJECT_NUMBER: i64 = 7;
const DOOR_SUBJECT_NUMBER: i64 = 7;

fn loop_size(public_key: i64, subject_number: i64) -> i64 {
    let mut loops = 0;
    let mut value = 1;
    loop {
        value *= subject_number;
        value %= 20201227;
        loops += 1;

        if value == public_key {
            break;
        }
    }

    loops
}

fn transform_subject_number(subject_number: i64, loops: i64) -> i64 {
    let mut value = 1;
    for _ in 0..loops {
        value *= subject_number;
        value %= 20201227;
    }
    value
}


fn main() {
    let public_keys: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let card_loops = loop_size(public_keys[0], CARD_SUBJECT_NUMBER);
    // println!("Card loops: {}", card_loops);
    // println!("Door loops: {}", loop_size(public_keys[1], DOOR_SUBJECT_NUMBER));

    let encryption_key = transform_subject_number(public_keys[1], card_loops);
    println!("Encryption key: {}", encryption_key);
}
