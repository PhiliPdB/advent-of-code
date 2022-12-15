use std::collections::HashSet;


#[derive(Debug)]
struct Sensor {
    location: (i64, i64),
    closest_beacon: (i64, i64),
}

impl Sensor {
    fn new(location: (i64, i64), closest_beacon: (i64, i64)) -> Self {
        Self { location, closest_beacon }
    }

    fn distance_to_closest(&self) -> i64 {
        (self.location.0 - self.closest_beacon.0).abs() + (self.location.1 - self.closest_beacon.1).abs()
    }

    fn distance(&self, loc: (i64, i64)) -> i64 {
        (self.location.0 - loc.0).abs() + (self.location.1 - loc.1).abs()
    }
}

fn main() {
    let sensors: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let (s, b) = l.split_once(": ").unwrap();

            let s = s.split_once(", ").unwrap();
            let s = (
                s.0.split_once('=').unwrap().1.parse::<i64>().unwrap(),
                s.1.split_once('=').unwrap().1.parse::<i64>().unwrap()
            );

            let b = b.split_once(", ").unwrap();
            let b = (
                b.0.split_once('=').unwrap().1.parse::<i64>().unwrap(),
                b.1.split_once('=').unwrap().1.parse::<i64>().unwrap()
            );

            Sensor::new(s, b)
        })
        .collect();

    // Part 1
    const ROW: i64 = 2_000_000;
    let mut no_beacon = HashSet::new();
    for sensor in &sensors {
        let d = sensor.distance_to_closest();

        let y_dist = (sensor.location.1 - ROW).abs();
        let x_min = sensor.location.0 - (d - y_dist);
        let x_max = sensor.location.0 + (d - y_dist);

        for x in x_min..=x_max {
            if (x, ROW) != sensor.closest_beacon {
                no_beacon.insert(x);
            }
        }
    }
    println!("[Part 1] Locations without beacon on y={ROW}: {}", no_beacon.len());

    // Part 2
    const MIN: i64 = 0;
    const MAX: i64 = 4_000_000;
    'y_loop: for y in MIN..(MAX+1) {
        let mut x = MIN;
        'x_loop: while x < MAX {
            for s in &sensors {
                let d = s.distance((x, y));
                let closest = s.distance_to_closest();

                if d <= closest {
                    let y_dist = (s.location.1 - y).abs();
                    if x < s.location.0 {
                        x += 2*(d - y_dist) + (closest - d) + 1;
                    } else {
                        x += (closest - d) + 1;
                    }
                    continue 'x_loop;
                }
            }

            println!("[Part 2] Tuning frequency: {}", x * 4_000_000 + y);
            break 'y_loop;
        }
    }
}
