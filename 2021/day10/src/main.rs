
fn bracket_matches(opening: char, closing: char) -> bool {
    match opening {
        '(' => closing == ')',
        '[' => closing == ']',
        '{' => closing == '}',
        '<' => closing == '>',
        _   => unreachable!(),
    }
}

fn incorrect_bracket_score(bracket: char) -> i32 {
    match bracket {
        ')' =>     3,
        ']' =>    57,
        '}' =>  1197,
        '>' => 25137,
        _   => unreachable!(),
    }
}

fn insert_bracket_score(bracket: char) -> u64 {
    match bracket {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _   => unreachable!(),
    }
}


fn parse_line(line: &str) -> (i32, Vec<char>) {
    let mut stack = Vec::with_capacity(line.len());

    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                let last_item = stack.pop().unwrap();
                if !bracket_matches(last_item, c) {
                    return (incorrect_bracket_score(c), stack);
                }
            },
            _ => panic!("Invalid bracket"),
        }
    }

    (0, stack)
}

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(parse_line)
        .collect();

    // Part 1
    let score: i32 = input.iter()
        .map(|(s, _)| s)
        .sum();
    println!("Invalid syntax error score: {:#10}", score);


    // Part 2
    let mut fix_missing_scores: Vec<_> = input.iter()
        // Only fix incomplete sequences and ignore the invalid ones.
        .filter_map(|(s, remaining)| (*s == 0).then(|| {
            // Make sure to go in reverse order through the stack,'
            // as the last item needs to be closed first.
            remaining.iter().rev().fold(0, |acc, c| {
                acc * 5 + insert_bracket_score(*c)
            })
        }))
        .collect();
    fix_missing_scores.sort_unstable();

    println!("Fix missing brackets score: {:#10}", fix_missing_scores[fix_missing_scores.len() / 2]);
}
