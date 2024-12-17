use computer::Computer;

mod computer;


fn run_compiled_input(registers: [i64; 3]) -> Vec<u8> {
    let mut output = Vec::new();
    let mut registers = registers;

    loop {
        // 2 4
        registers[1] = registers[0] % 8;
        // 1 5
        registers[1] ^= 5;
        // 7 5
        registers[2] = registers[0] / 2_i64.pow(registers[1] as u32);
        // 4 5
        registers[1] ^= registers[2];
        // 0 3
        registers[0] /= 8;
        // 1 6
        registers[1] ^= 6;
        // 5 5
        output.push((registers[1] % 8) as u8);

        if registers[0] == 0 {
            break;
        }
    }

    output
}

fn find_a(program: &[u8], index: usize, a: i64) -> Option<i64> {
    for a_bit in 0..8 {
        let test_a = a | (a_bit << (3*index));
        let output = run_compiled_input([test_a, 0, 0]);

        if output.len() != program.len() {
            continue;
        }
        if output[index] == program[index] {
            if index == 0 {
                return Some(test_a);
            } else if let Some(answer) = find_a(program, index - 1, test_a) {
                return Some(answer);
            }
        }
    }

    None
}

fn main() {
    // Test input
    // let program = vec![0,1,5,4,3,0];
    // let mut computer = Computer::new(
    //     [729, 0, 0],
    //     program.clone(),
    // );
    // Real input
    let program = vec![2,4,1,5,7,5,4,5,0,3,1,6,5,5,3,0];
    let mut computer = Computer::new(
        [63281501, 0, 0],
        program.clone(),
    );

    let string_output = computer.run()
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");
    println!("[Part 1] Output: {string_output}");


    // Part 2
    let a = find_a(&program, program.len() - 1, 0).unwrap();
    println!("[Part 2] Minimal 'a' value: {a}");
}
