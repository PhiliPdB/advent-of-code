
type Policy = (i32, i32, char);

fn get_policy(policy: &str) -> Policy {
    let policy_char = policy.chars().collect::<Vec<_>>()[policy.len() - 1];
    let counts: Vec<_> = policy[0..policy.len() - 2]
        .split('-')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    (counts[0], counts[1], policy_char)
}

fn test_password_part1((min, max, policy_char): Policy, password: &str) -> bool {
    let policy_char_count = password
        .chars()
        .filter(|&c| c == policy_char)
        .count() as i32;

    min <= policy_char_count && policy_char_count <= max
}

fn test_password_part2((pos1, pos2, policy_char): Policy, password: &str) -> bool {
    let pos1 = pos1 as usize - 1;
    let pos2 = pos2 as usize - 1;

    let chars: Vec<_> = password
        .chars()
        .collect();

    if pos1 >= chars.len() {
        false
    } else if pos2 >= chars.len() {
        chars[pos1] == policy_char
    } else {
        (chars[pos1] == policy_char) ^ (chars[pos2] == policy_char)
    }
}


fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| {
            let pw: Vec<_> = s.split(": ").collect();
            (pw[0], pw[1])
        })
        .collect();

    let valid_passwords_part1 = input.iter()
        .filter(|(policy, password)| {
            let policy = get_policy(policy);

            test_password_part1(policy, password)
        })
        .count();
    let valid_passwords_part2 = input.iter()
        .filter(|(policy, password)| {
            let policy = get_policy(policy);

            test_password_part2(policy, password)
        })
        .count();


    println!("[Part 1] Valid passwords: {}", valid_passwords_part1);
    println!("[Part 2] Valid passwords: {}", valid_passwords_part2);
}
