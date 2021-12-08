
pub struct Note<'a> {
    pattern: Vec<&'a str>,
    output: Vec<&'a str>,
}

impl<'a> Note<'a> {

    pub fn decode(&self) -> i32 {
        // Get digit codes with unique lengths
        let digit_1 = self.pattern.iter().find(|p| p.len() == 2).unwrap();
        let digit_4 = self.pattern.iter().find(|p| p.len() == 4).unwrap();
        let digit_7 = self.pattern.iter().find(|p| p.len() == 3).unwrap();
        let digit_8 = self.pattern.iter().find(|p| p.len() == 7).unwrap();

        // Digits of length 6
        let digit_0 = self.pattern.iter().find(|p| {
            p.len() == 6 && contains_digit(p, digit_7) && !contains_digit(p, digit_4)
        }).unwrap();
        let digit_9 = self.pattern.iter().find(|p| {
            p.len() == 6 && contains_digit(p, digit_7) && contains_digit(p, digit_4)
        }).unwrap();
        let digit_6 = self.pattern.iter().find(|p| {
            p.len() == 6 && !contains_digit(p, digit_0) && !contains_digit(p, digit_9)
        }).unwrap();
        let segment_e = ['a', 'b', 'c', 'd', 'e', 'f', 'g'].into_iter()
            .filter(|&c| !digit_9.contains(c))
            .collect::<Vec<_>>()[0];

        // Digits of length 5
        let digit_3 = self.pattern.iter().find(|p| {
            p.len() == 5 && contains_digit(p, digit_7)
        }).unwrap();
        let digit_2 = self.pattern.iter().find(|p| {
            p.len() == 5 && p.contains(segment_e)
        }).unwrap();
        let digit_5 = self.pattern.iter().find(|p| {
            p.len() == 5 && !p.contains(segment_e) && !contains_digit(p, digit_3)
        }).unwrap();

        let digits = vec![
            digit_0, digit_1, digit_2, digit_3, digit_4, digit_5, digit_6, digit_7, digit_8, digit_9,
        ];

        self.output.iter()
            .fold(0, |acc, p| acc * 10 + find_digit(&digits, p))
    }

    pub fn from_str(s: &'a str) -> Self {
        let parts: Vec<_> = s.split(" | ").collect();

        Self {
            pattern: parts[0].split(' ').collect(),
            output: parts[1].split(' ').collect(),
        }
    }
}

fn contains_digit(x: &str, y: &str) -> bool {
    y.chars().all(|c| x.contains(c))
}

fn find_digit(digits: &Vec<&&str>, pattern: &str) -> i32 {
    digits.iter()
        .position(|d| d.len() == pattern.len() && contains_digit(d, pattern))
        .unwrap() as i32
}

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| Note::from_str(s))
        .collect();

    // Part 1
    let unique_output_count = input.iter()
        .flat_map(|n| &n.output)
        .filter(|o| [2, 3, 4, 7].contains(&o.len()))
        .count();

    println!("Outputs of 1, 4, 7, or 8: {}", unique_output_count);


    // Part 2
    let digit_sum: i32 = input.iter()
        .map(|n| n.decode())
        .sum();

    println!("Sum of outputs: {}", digit_sum);
}
