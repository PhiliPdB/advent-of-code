use itertools::Itertools;

type Point = (i64, i64);

// Check if point is inside by counting edge encounters
fn is_point_inside(red_tiles: &[Point], point: Point) -> bool {
    let mut inside = false;

    for (from, to) in red_tiles
        .iter()
        .circular_tuple_windows()
    {
        let x_from = i64::min(from.0, to.0);
        let x_to = i64::max(from.0, to.0);

        let y_from = i64::min(from.1, to.1);
        let y_to = i64::max(from.1, to.1);

        // Check if point is on the edge (this is seen as inside)
        if (x_from..=x_to).contains(&point.0) && (y_from..=y_to).contains(&point.1) {
            return true;
        }

        if y_from < point.1 && point.1 <= y_to && point.0 < x_to {
            let dy = from.1 - to.1;
            if dy != 0 {
                let x_intersection = (point.1 - from.1) * (from.0 - to.0) / dy + from.0;

                if point.0 <= x_intersection {
                    inside = !inside;
                }
            }
        }
    }

    inside
}

fn orientation(p: Point, q: Point, r: Point) -> i64 {
    let val = (q.1 - p.1) * (r.0 - q.0) - (q.0 - p.0) * (r.1 - q.1);
    val.signum()
}

fn line_intersects(l1_from: Point, l1_to: Point, l2_from: Point, l2_to: Point) -> bool {
    let o1 = orientation(l1_from, l1_to, l2_from);
    let o2 = orientation(l1_from, l1_to, l2_to);
    let o3 = orientation(l2_from, l2_to, l1_from);
    let o4 = orientation(l2_from, l2_to, l1_to);

    o1 * o2 < 0 && o3 * o4 < 0
}

fn is_rectangle_inside(red_tiles: &[Point], rectangle: [Point; 4]) -> bool {
    if rectangle
        .iter()
        .any(|p| !is_point_inside(red_tiles, *p))
    {
        // All corners must lie inside
        return false;
    }

    // None of the rectangle edges must intersect with any polygon edge (i.e. the red tiles)
    red_tiles
        .iter()
        .circular_tuple_windows()
        .all(|(e1, e2)| {
            rectangle
                .iter()
                .circular_tuple_windows()
                .all(|(r1, r2)| !line_intersects(*r1, *r2, *e1, *e2))
        })
}

fn main() {
    let red_tiles: Vec<Point> = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();

            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let mut part1_largest_rectangle = 0;
    let mut part2_largest_rectangle = 0;
    for (t1, t2) in red_tiles.iter().tuple_combinations() {
        let area = (t1.0.abs_diff(t2.0) + 1) * (t1.1.abs_diff(t2.1) + 1);

        if area > part1_largest_rectangle {
            part1_largest_rectangle = area;
        }

        if area > part2_largest_rectangle
            // Check if we are inside the polygon
            && is_rectangle_inside(&red_tiles, [*t1, (t2.0, t1.1), *t2, (t1.0, t2.1)])
        {
            part2_largest_rectangle = area;
        }
    }
    println!("[Part 1] Largest rectangle: {part1_largest_rectangle}");
    println!("[Part 2] Largest rectangle: {part2_largest_rectangle}");
}
