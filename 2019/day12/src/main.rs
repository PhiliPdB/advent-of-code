#![allow(clippy::comparison_chain)]

use std::slice;

use num::integer::lcm;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Moon {
    x: i32, y: i32, z: i32,
    vx: i32, vy: i32, vz: i32,
}

impl Moon {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            x, y, z,
            vx: 0, vy: 0, vz: 0,
        }
    }

    pub fn update_position(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
    }

    pub fn get_energy(&self) -> i32 {
        let potential = self.x.abs() + self.y.abs() + self.z.abs();
        let kinetic = self.vx.abs() + self.vy.abs() + self.vz.abs();

        potential * kinetic
    }
}

fn energy_after_steps(mut moons: [Moon; 4], steps: u32) -> i32 {
    let mut iteration = 0;
    let ptr = moons.as_mut_ptr();

    while iteration < steps {
        // Update velocities
        for i in 1..moons.len() {
            for j in 0..i {
                // SAFETY: i != j
                let (m1, m2) = unsafe {
                    (
                        &mut slice::from_raw_parts_mut(ptr.add(i), 1)[0],
                        &mut slice::from_raw_parts_mut(ptr.add(j), 1)[0]
                    )
                };

                if m1.x > m2.x {
                    m1.vx -= 1;
                    m2.vx += 1;
                } else if m1.x < m2.x {
                    m1.vx += 1;
                    m2.vx -= 1;
                }

                if m1.y > m2.y {
                    m1.vy -= 1;
                    m2.vy += 1;
                } else if m1.y < m2.y {
                    m1.vy += 1;
                    m2.vy -= 1;
                }

                if m1.z > m2.z {
                    m1.vz -= 1;
                    m2.vz += 1;
                } else if m1.z < m2.z {
                    m1.vz += 1;
                    m2.vz -= 1;
                }
            }
        }

        // Update positions
        for m in moons.iter_mut() {
            m.update_position();
        }

        iteration += 1;
    }


    moons.iter()
        .map(|m| m.get_energy())
        .sum()
}

fn iterations_for_axis<const AXIS: usize>(mut moons: [Moon; 4]) -> u64 {
    let orig_x = moons.map(|m| m.x);
    let orig_y = moons.map(|m| m.y);
    let orig_z = moons.map(|m| m.z);

    let mut iteration = 0;
    let ptr = moons.as_mut_ptr();

    loop {
        // Update velocities
        for i in 1..moons.len() {
            for j in 0..i {
                // SAFETY: i != j
                let (m1, m2) = unsafe {
                    (
                        &mut slice::from_raw_parts_mut(ptr.add(i), 1)[0],
                        &mut slice::from_raw_parts_mut(ptr.add(j), 1)[0]
                    )
                };

                match AXIS {
                    0 => {
                        if m1.x > m2.x {
                            m1.vx -= 1;
                            m2.vx += 1;
                        } else if m1.x < m2.x {
                            m1.vx += 1;
                            m2.vx -= 1;
                        }
                    },
                    1 => {
                        if m1.y > m2.y {
                            m1.vy -= 1;
                            m2.vy += 1;
                        } else if m1.y < m2.y {
                            m1.vy += 1;
                            m2.vy -= 1;
                        }
                    },
                    2 => {
                        if m1.z > m2.z {
                            m1.vz -= 1;
                            m2.vz += 1;
                        } else if m1.z < m2.z {
                            m1.vz += 1;
                            m2.vz -= 1;
                        }
                    },
                    _ => panic!("Invalid axis"),
                }
            }
        }

        // Update positions
        for m in moons.iter_mut() {
            m.update_position();
        }

        iteration += 1;

        match AXIS {
            0 => {
                if moons.iter().all(|m| m.vx == 0) && moons.map(|m| m.x) == orig_x {
                    break;
                }
            },
            1 => {
                if moons.iter().all(|m| m.vy == 0) && moons.map(|m| m.y) == orig_y {
                    break;
                }
            },
            2 => {
                if moons.iter().all(|m| m.vz == 0) && moons.map(|m| m.z) == orig_z {
                    break;
                }
            },
            _ => panic!("Invalid axis"),
        }
    }

    iteration
}

fn main() {
    let moons = [
        Moon::new(19, -10, 7),
        Moon::new(1, 2, -3),
        Moon::new(14, -4, 1),
        Moon::new(8, 7, -6),
    ];

    // Part 1
    println!("Sum of total energy: {}", energy_after_steps(moons, 1000));

    let iterations_x = iterations_for_axis::<0>(moons);
    let iterations_y = iterations_for_axis::<1>(moons);
    let iterations_z = iterations_for_axis::<2>(moons);

    let total_iterations = lcm(lcm(iterations_x, iterations_y), iterations_z);
    println!("Total iterations: {}", total_iterations);
}
