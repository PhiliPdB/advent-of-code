use std::ops::Range;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use itertools::Itertools;


fn get_value(program: &[i32], mode: i32, location: usize) -> i32 {
    if mode == 0 {
        program[program[location] as usize]
    } else {
        program[location]
    }
}


fn run_program_thread(mut program: Vec<i32>, input: Receiver<i32>, output: Sender<i32>) {
    // Set instruction pointer
    let mut ip = 0;
    loop {
        let instruction = ((program[ip] / 10) % 10) * 10 + program[ip] % 10;

        match instruction {
            1 => {
                let val1 = get_value(&program, (program[ip] / 100) % 10, ip + 1);
                let val2 = get_value(&program, (program[ip] / 1_000) % 10, ip + 2);
                let loc3 = program[ip + 3];

                program[loc3 as usize] = val1 + val2;
                ip += 4;
            },
            2 => {
                let val1 = get_value(&program, (program[ip] / 100) % 10, ip + 1);
                let val2 = get_value(&program, (program[ip] / 1_000) % 10, ip + 2);
                let loc3 = program[ip + 3];

                program[loc3 as usize] = val1 * val2;
                ip += 4;
            },
            3 => {
                let loc1 = program[ip + 1];
                // Receive input from channel
                program[loc1 as usize] = input.recv().unwrap();
                ip += 2;
            },
            4 => {
                let loc1 = program[ip + 1];
                let val1 =
                    if (program[ip] / 100) % 10 == 0 {
                        program[loc1 as usize]
                    } else {
                        loc1
                    };

                // Send output on channel
                output.send(val1).unwrap();
                ip += 2;
            },
            5 => {
                let val1 = get_value(&program, (program[ip] / 100) % 10, ip + 1);
                let val2 = get_value(&program, (program[ip] / 1_000) % 10, ip + 2);

                if val1 != 0 {
                    ip = val2 as usize;
                } else {
                    ip += 3;
                }
            },
            6 => {
                let val1 = get_value(&program, (program[ip] / 100) % 10, ip + 1);
                let val2 = get_value(&program, (program[ip] / 1_000) % 10, ip + 2);

                if val1 == 0 {
                    ip = val2 as usize;
                } else {
                    ip += 3;
                }
            },
            7 => {
                let val1 = get_value(&program, (program[ip] / 100) % 10, ip + 1);
                let val2 = get_value(&program, (program[ip] / 1_000) % 10, ip + 2);
                let loc3 = program[ip + 3];

                if val1 < val2 {
                    program[loc3 as usize] = 1;
                } else {
                    program[loc3 as usize] = 0;
                }
                ip += 4;
            },
            8 => {
                let val1 = get_value(&program, (program[ip] / 100) % 10, ip + 1);
                let val2 = get_value(&program, (program[ip] / 1_000) % 10, ip + 2);
                let loc3 = program[ip + 3];

                if val1 == val2 {
                    program[loc3 as usize] = 1;
                } else {
                    program[loc3 as usize] = 0;
                }
                ip += 4;
            },
            99 => break,
            _ => panic!("Invalid program"),
        }
    }
}

fn run_amps(program: &[i32], phase_range: Range<i32>) -> i32 {
    let mut max_output = 0;

    for phase_setting in phase_range.permutations(5) {
        // Create channels for communication between the amps.
        let (input, ia) = mpsc::channel();
        let (oa, ib) = mpsc::channel();
        let (ob, ic) = mpsc::channel();
        let (oc, id) = mpsc::channel();
        let (od, ie) = mpsc::channel();
        let (oe, output) = mpsc::channel();

        // Send phase settings
        input.send(phase_setting[0]).unwrap();
        oa.send(phase_setting[1]).unwrap();
        ob.send(phase_setting[2]).unwrap();
        oc.send(phase_setting[3]).unwrap();
        od.send(phase_setting[4]).unwrap();

        // Create threads for each amp

        let amp_a_program = program.to_owned();
        let _amp_a = thread::spawn(move || {
            run_program_thread(amp_a_program, ia, oa);
        });

        let amp_b_program = program.to_owned();
        let _amp_b = thread::spawn(move || {
            run_program_thread(amp_b_program, ib, ob);
        });

        let amp_c_program = program.to_owned();
        let _amp_c = thread::spawn(move || {
            run_program_thread(amp_c_program, ic, oc);
        });

        let amp_d_program = program.to_owned();
        let _amp_d = thread::spawn(move || {
            run_program_thread(amp_d_program, id, od);
        });

        let amp_e_program = program.to_owned();
        let _amp_e = thread::spawn(move || {
            run_program_thread(amp_e_program, ie, oe);
        });

        // Start the program by sending the initial input
        input.send(0).unwrap();

        // Listen for outputs of amp e.
        for o in output {
            // Try to send the output back to amp a
            if input.send(o).is_err() {
                // If sending results in an error, the receiver is probably
                // gone, which means that the program has halted.
                // Thus, we are finished.

                // Check the final output
                if o > max_output {
                    max_output = o;
                }

                break;
            };
        }
    }

    max_output
}

fn main() {
    let program: Vec<_> = include_str!("../input.txt")
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // Part 1

    let part1_max_output = run_amps(&program, 0..5);
    println!("[Part 1] Max output: {:#8}", part1_max_output);

    // Part 2

    let part2_max_output = run_amps(&program, 5..10);
    println!("[Part 2] Max output: {:#8}", part2_max_output);
}
