use std::thread;
use std::time::{Duration, Instant};

use intcode::Program;

mod intcode;


fn main() {
    let program = Program::new(
        include_str!("../input.txt")
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect()
    );

    const NETWORK_SIZE: usize = 50;
    let mut inputs = Vec::new();
    let (program_output, global_output) = crossbeam_channel::unbounded();
    for i in 0..NETWORK_SIZE {
        let (input, program_input) = crossbeam_channel::unbounded();
        // Send program address
        input.send(i as i64).unwrap();
        inputs.push(input);

        let mut p = program.clone();
        let program_output = program_output.clone();
        thread::spawn(move || {
            p.run(program_input, program_output);
        });
    }
    let inputs = inputs;

    // Read global output to navigate messages
    let mut part1_output = true;
    let mut last_nat_received = (-1, -1);
    let mut last_y_send = -1;

    let mut last_received = Instant::now();

    loop {
        let mut output_iter = global_output.try_iter();
        if let Some(address) = output_iter.next() {
            last_received = Instant::now();

            let mut output_iter = global_output.iter();
            let x = output_iter.next().unwrap();
            let y = output_iter.next().unwrap();

            if address == 255 {
                if part1_output {
                    println!("[Part 1] Y = {y}");
                    part1_output = false;
                }

                last_nat_received = (x, y);
            } else {
                inputs[address as usize].send(x).unwrap();
                inputs[address as usize].send(y).unwrap();
            }

            continue;
        }

        if last_received.elapsed().as_millis() >= 500 && inputs.iter().all(|i| i.is_empty()) {
            inputs[0].send(last_nat_received.0).unwrap();
            inputs[0].send(last_nat_received.1).unwrap();

            if last_nat_received.1 == last_y_send {
                println!("[Part 2] Y = {last_y_send}");
                break;
            }

            last_y_send = last_nat_received.1;
        }

        thread::sleep(Duration::from_millis(100));
    }
}
