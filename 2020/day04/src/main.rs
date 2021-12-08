use std::collections::HashMap;


type Passport<'a> = HashMap<&'a str, &'a str>;

fn is_valid_passport(passport: &Passport) -> bool {
    const REQUIRED_FIELDS: [&str; 7] = ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];

    REQUIRED_FIELDS.iter()
        .all(|field| passport.contains_key(field))
}

fn is_strict_valid_passport(passport: &Passport) -> bool {
    return passport.contains_key("byr") && (1920..=2002).contains(&passport["byr"].parse::<i32>().unwrap())
        && passport.contains_key("iyr") && (2010..=2020).contains(&passport["iyr"].parse::<i32>().unwrap())
        && passport.contains_key("eyr") && (2020..=2030).contains(&passport["eyr"].parse::<i32>().unwrap())
        && passport.contains_key("hgt") && is_valid_height(passport["hgt"])
        && passport.contains_key("hcl") && is_valid_color(passport["hcl"])
        && passport.contains_key("ecl") && ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&passport["ecl"])
        && passport.contains_key("pid") && passport["pid"].len() == 9 && passport["pid"].parse::<i32>().is_ok();
}

fn is_valid_height(height: &str) -> bool {
    let (h, u) = height.split_at(height.len() - 2);

    if let Ok(h) = h.parse::<i32>() {
        match u {
            "cm" => (150..=193).contains(&h),
            "in" => (59..=76).contains(&h),
            _    => false
        }
    } else {
        false
    }
}

fn is_valid_color(color: &str) -> bool {
    let (h, c) = color.split_at(1);

    h == "#" && i32::from_str_radix(c, 16).is_ok()
}

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .split("\n\n")
        .map(|s| {
            Passport::from_iter(
                s.split_whitespace()
                    .map(|kv| kv.split_once(':').unwrap())
            )
        })
        .collect();


    let valid_passports = input.iter()
        .filter(|p| is_valid_passport(p))
        .count();

    println!("Valid passports: {:?}", valid_passports);


    let strict_valid_passports = input.iter()
        .filter(|p| is_strict_valid_passport(p))
        .count();

    println!("Strict valid passports: {:?}", strict_valid_passports);
}
