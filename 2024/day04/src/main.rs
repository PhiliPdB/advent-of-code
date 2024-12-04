
fn xmas_count((x, y): (usize, usize), search_area: &Vec<Vec<char>>) -> u32 {
    let height = search_area.len();
    let width = search_area[0].len();

    const NEEDLE: [char; 4] = ['X', 'M', 'A', 'S'];

    let mut valid_directions = [true; 8];
    for (i, c) in NEEDLE.into_iter().enumerate().skip(1) {
        // Horizontal left
        valid_directions[0] = valid_directions[0] && x >= i && search_area[y][x-i] == c;
        // Horizontal right
        valid_directions[1] = valid_directions[1] && x + i < width && search_area[y][x+i] == c;

        // Vertical up
        valid_directions[2] = valid_directions[2] && y >= i && search_area[y-i][x] == c;
        // Vertical down
        valid_directions[3] = valid_directions[3] && y + i < height && search_area[y+i][x] == c;

        // Diagonal up left
        valid_directions[4] = valid_directions[4] && y >= i && x >= i && search_area[y-i][x-i] == c;
        // Diagonal up right
        valid_directions[5] = valid_directions[5] && y >= i && x + i < width && search_area[y-i][x+i] == c;
        // Diagonal up left
        valid_directions[6] = valid_directions[6] && y + i < height && x >= i && search_area[y+i][x-i] == c;
        // Diagonal up right
        valid_directions[7] = valid_directions[7] && y + i < height && x + i < width && search_area[y+i][x+i] == c;
    }

    valid_directions.into_iter()
        .filter(|d| *d)
        .count() as u32
}

fn x_mas_count((x, y): (i32, i32), search_area: &Vec<Vec<char>>) -> u32 {
    let height = search_area.len() as i32;
    let width = search_area[0].len() as i32;

    let mut count = 0;
    let directions = [
        ((-1, -1), [(-2, 0), (0, -2)]),
        (( 1, -1), [( 2, 0), (0, -2)]),
        ((-1,  1), [(-2, 0), (0,  2)]),
        (( 1,  1), [( 2, 0), (0,  2)]),
    ];
    for ((dx, dy), p) in directions {
        let mut is_valid = true;
        for (i, c) in [(1, 'A'), (2, 'S')] {
            is_valid = is_valid
                && x + i*dx >= 0 && x + i*dx < width
                && y + i*dy >= 0 && y + i*dy < height
                && search_area[(y+i*dy) as usize][(x+i*dx) as usize] == c;
        }

        if is_valid
            && ( // Check the cross
                // Either p[0] is M and p[1] is S
                (search_area[(y+p[0].1) as usize][(x+p[0].0) as usize] == 'M' && search_area[(y+p[1].1) as usize][(x+p[1].0) as usize] == 'S')
                // Or the other way around
                || (search_area[(y+p[0].1) as usize][(x+p[0].0) as usize] == 'S' && search_area[(y+p[1].1) as usize][(x+p[1].0) as usize] == 'M')
            )
        {
            count += 1;
        }
    }

    count
}

fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();

    let mut total_xmas_count = 0;
    let mut total_x_mas_count = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            match input[y][x] {
                'X' => total_xmas_count += xmas_count((x, y), &input),
                'M' => total_x_mas_count += x_mas_count((x as i32, y as i32), &input),
                _ => (),
            }
        }
    }
    // Found x-mas for both the masses in the x
    // Thus have to divide by 2
    total_x_mas_count /= 2;

    println!("[Part 1] XMAS appears {total_xmas_count} times");
    println!("[Part 2] X-MAS appears {total_x_mas_count} times");
}
