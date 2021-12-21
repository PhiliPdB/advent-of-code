use std::collections::{HashMap, HashSet};

fn main() {
    let mut all_ingredients = HashMap::new();
    let mut all_allergens = HashSet::new();

    let mut ingredient_lookup = HashMap::new();

    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| {
            let new_s = s.replace(")", "");
            let splitted: Vec<_> = new_s.split(" (contains ").collect();

            (
                splitted[0].split(' ').map(String::from).collect::<Vec<_>>(),
                splitted[1].split(", ").map(String::from).collect::<Vec<_>>()
            )
        })
        .collect();


    // Fill all ingredients and allergens
    for (ingredients, allergens) in &input {
        for i in ingredients {
            *all_ingredients.entry(i.clone()).or_insert(0) += 1;
        }

        for a in allergens {
            all_allergens.insert(a.clone());
        }
    }

    // Create a lookup table from allergens to possible ingredients
    for (ingredients, allergens) in &input {
        let hs_ingredients: HashSet<_> = HashSet::from_iter(ingredients.iter().cloned());

        for a in allergens {
            ingredient_lookup.entry(a.clone())
                .or_insert_with(|| all_ingredients.keys().cloned().collect::<HashSet<_>>())
                .retain(|s| hs_ingredients.contains(s));
        }
    }


    // Try and reduce the possibilities for each allergen
    let mut made_changes = true;
    while made_changes {
        let mut to_remove = Vec::new();

        for (k, v) in ingredient_lookup.iter() {
            if v.len() == 1 {
                to_remove.push((k.clone(), v.iter().next().unwrap().clone()));
            }
        }

        made_changes = !to_remove.is_empty() && to_remove.len() != ingredient_lookup.len();
        for (k, v) in ingredient_lookup.iter_mut() {
            for (a, i) in &to_remove {
                if k.ne(a) {
                    v.retain(|s| s.ne(i));
                }
            }
        }
    }

    // Part 1
    let mut allergen_ingredients = HashSet::new();
    for (_, i) in ingredient_lookup.iter() {
        allergen_ingredients.extend(i.iter().cloned());
    }
    let allergen_free_ingredients: i32 = all_ingredients.iter()
        .filter_map(|(s, c)| (!allergen_ingredients.contains(s)).then(|| *c))
        .sum();

    println!("Total allergen-free ingredients: {}", allergen_free_ingredients);

    // Part 2
    let mut dangerous_ingredients: Vec<_> = ingredient_lookup.into_iter().map(|(k, v)| (k, v.into_iter().next().unwrap())).collect();
    dangerous_ingredients.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

    let dangerous_ingredients: Vec<_> = dangerous_ingredients.into_iter()
        .map(|(_, v)| v)
        .collect();

    println!("Canonical dangerous ingredient list:");
    println!("{}", dangerous_ingredients.join(","));
}
