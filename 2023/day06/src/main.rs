
#[derive(Debug, Copy, Clone)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    fn number_of_ways_to_beat(&self) -> u32 {
        // let t is time holding the button,
        // then distance = t*(T - t) = -t^2 + T*t
        // Thus we need to solve -t^2 + T*t - D > 0
        // Where T is time and D is the record distance

        let a = -1.0;
        let b = self.time as f64;
        let c = -(self.distance as f64);
        let d = b * b - 4.0 * a * c;

        let t1 = (-b + d.sqrt()) / (2.0 * a);
        let t2 = (-b - d.sqrt()) / (2.0 * a);

        // Use a small epsilon to move away from the edges where
        // the equality holds.
        // We need to be strictly greater than the record distance.
        let epsilon = 0.001;
        ((t2 - epsilon).floor() - (t1 + epsilon).ceil() + 1.0) as u32
    }
}

fn main() {
    let races = [
        Race::new(38,  241),
        Race::new(94, 1549),
        Race::new(79, 1074),
        Race::new(70, 1091),
    ];

    let part1_prod: u32 = races.iter()
        .map(|r| r.number_of_ways_to_beat())
        .product();
    println!("[Part 1] Answer: {part1_prod}");


    let part2_race = Race::new(38947970, 241154910741091);
    println!(
        "[Part 2] Number of ways to beat: {}",
        part2_race.number_of_ways_to_beat()
    );
}
