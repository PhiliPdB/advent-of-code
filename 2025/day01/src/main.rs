fn main() {
    let rotations: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| {
            let mut num = s[1..].parse::<i32>().unwrap();
            if &s[..1] == "L" {
                num = -num;
            }
            num
        })
        .collect();

    let mut position = 50;
    let mut part1_password = 0;
    let mut part2_password = 0;
    for r in rotations {
        let prev_position = position;
        position += r % 100;

        let before_mod = position;
        position = position.rem_euclid(100);

        if position == 0 {
            part1_password += 1;
        }

        if prev_position != 0 && (before_mod != position || position == 0) {
            part2_password += 1;
        }
        part2_password += r.abs() / 100;
    }
    println!("[Part 1] Password: {part1_password}");
    println!("[Part 2] Password: {part2_password}");
}
