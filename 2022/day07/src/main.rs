use std::collections::{HashMap, hash_map::Entry};

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .collect();

    let mut current_folder = Vec::new();
    let mut folders = HashMap::new();
    folders.insert(String::from(""), Vec::new());

    for line in input {
        if line == "$ cd /" {
            current_folder.clear();
        } else if line == "$ cd .." {
            current_folder.pop();
        } else if line.starts_with("$ cd") {
            current_folder.push(line.split(' ').last().unwrap());
            folders.insert(current_folder.join("/"), Vec::new());
        } else if line == "$ ls" {
            continue;
        } else {
            let (t, name) = line.split_once(' ').unwrap();
            if let Ok(n) = t.parse::<usize>() {
                folders.get_mut(&current_folder.join("/")).unwrap().push((n, name));
            }
        }
    }

    let mut folder_sizes = HashMap::new();
    for (k, v) in &folders {
        let size = v.iter().map(|n| n.0).sum::<usize>();

        let splitted: Vec<_> = k.split('/').collect();
        for i in 0..=splitted.len() {
            let key = splitted[0..i].join("/");
            match folder_sizes.entry(key) {
                Entry::Occupied(mut e) => {
                    *e.get_mut() += size;
                },
                Entry::Vacant(e) => {
                    e.insert(size);
                },
            };
        }
    }

    let part1_result: usize = folder_sizes.iter()
        .filter_map(|(_k, v)| {
            if *v <= 100_000 {
                Some(v)
            } else {
                None
            }
        })
        .sum();
    println!("[Part 1] Sum: {part1_result}");


    let free_space = 70_000_000 - folder_sizes[""];
    let required_size = 30_000_000 - free_space;

    let mut smallest = usize::MAX;
    for (_k, v) in folder_sizes {
        if v > required_size && v < smallest {
            smallest = v;
        }
    }
    println!("[Part 2] Size: {smallest}");
}
