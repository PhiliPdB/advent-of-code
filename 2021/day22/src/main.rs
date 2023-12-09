use ndarray::Array3;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Status {
    On,
    #[default]
    Off
}

#[derive(Debug, Clone, Copy)]
pub struct Command {
    command: Status,
    x_start: i32, x_end: i32,
    y_start: i32, y_end: i32,
    z_start: i32, z_end: i32,
}

fn prepare_intersection_points(points: Vec<i32>) -> Vec<i32> {
    let mut prepared_points = points;
    // Make sure the points are sorted
    prepared_points.sort_unstable();
    // And no duplicates exist
    prepared_points.dedup();
    prepared_points
}


fn get_on_cubes(commands: &[Command]) -> u64 {
    // Combine all 'intersection' points, which are the points at
    // which a cube can be split up.
    let intersection_x = prepare_intersection_points(commands.iter().flat_map(|c| [c.x_start, c.x_end]).collect());
    let intersection_y = prepare_intersection_points(commands.iter().flat_map(|c| [c.y_start, c.y_end]).collect());
    let intersection_z = prepare_intersection_points(commands.iter().flat_map(|c| [c.z_start, c.z_end]).collect());

    // Map each command to a intersection start, end index
    let intersect_commands: Vec<_> = commands.iter()
        .map(|c| Command {
            command: c.command,
            x_start: intersection_x.binary_search(&c.x_start).unwrap() as i32,
            x_end: intersection_x.binary_search(&c.x_end).unwrap() as i32,
            y_start: intersection_y.binary_search(&c.y_start).unwrap() as i32,
            y_end: intersection_y.binary_search(&c.y_end).unwrap() as i32,
            z_start: intersection_z.binary_search(&c.z_start).unwrap() as i32,
            z_end: intersection_z.binary_search(&c.z_end).unwrap() as i32,
        })
        .collect();

    // Create 3D-matrix to save the status of each cube
    let mut cube_status = Array3::default((intersection_x.len() - 1, intersection_y.len() - 1, intersection_z.len() - 1));

    for command in &intersect_commands {
        for x in command.x_start..command.x_end {
            for y in command.y_start..command.y_end {
                for z in command.z_start..command.z_end {
                    cube_status[[x as usize, y as usize, z as usize]] = command.command;
                }
            }
        }
    }

    // Count the volume of the 'on'-cubes
    cube_status.outer_iter()
        .enumerate()
        .map(|(x, square)| {
            let dx = (intersection_x[x + 1] - intersection_x[x]) as u64;

            square.outer_iter().enumerate()
                .map(|(y, row)| {
                    let dy = (intersection_y[y + 1] - intersection_y[y]) as u64;

                    let inter_prod = dx * dy;
                    row.iter().enumerate()
                        .map(|(z, status)| if *status == Status::On {
                            let dz = (intersection_z[z + 1] - intersection_z[z]) as u64;

                            inter_prod * dz
                        } else {
                            0
                        })
                        .sum::<u64>()
                })
                .sum::<u64>()
        })
        .sum()
}


fn main() {
    let input: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| {
            let (command, coords) = s.split_once(' ').unwrap();
            let command = match command {
                "on" => Status::On,
                "off" => Status::Off,
                _ => panic!("Invalid command"),
            };
            let coords: Vec<_> = coords.split(',')
                .flat_map(|c| {
                    c[2..].split("..").map(|n| n.parse::<i32>().unwrap())
                })
                .collect();

            Command {
                command,
                x_start: coords[0], x_end: coords[1] + 1,
                y_start: coords[2], y_end: coords[3] + 1,
                z_start: coords[4], z_end: coords[5] + 1,
            }
        })
        .collect();

    let initialization_input: Vec<_> = input.iter()
        .map(|c| Command {
            command: c.command,
            x_start: c.x_start.max(-50), x_end: c.x_end.min(51),
            y_start: c.y_start.max(-50), y_end: c.y_end.min(51),
            z_start: c.z_start.max(-50), z_end: c.z_end.min(51),
        })
        .collect();

    println!("[Part 1] On cubes: {:#16}", get_on_cubes(&initialization_input));
    println!("[Part 2] On cubes: {:#16}", get_on_cubes(&input));
}
