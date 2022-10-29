using day23;

static Instruction[] ReadInstructions() {
    return File.ReadAllLines("./input.txt")
        .Select<string, Instruction>(line => {
            string[] splitted = line.Split(' ');
            switch (splitted[0]) {
                case "cpy": {
                    bool xIsReg = !int.TryParse(splitted[1], out int xReg);
                    int yReg = splitted[2][0] - 'a';
                    if (xIsReg) {
                        xReg = splitted[1][0] - 'a';
                    }

                    return new Copy(xReg, yReg, xIsReg, true);
                }
                case "inc": {
                    int register = splitted[1][0] - 'a';
                    return new Increase(register);
                }
                case "dec": {
                    int register = splitted[1][0] - 'a';
                    return new Decrease(register);
                }
                case "jnz": {
                    bool isReg = !int.TryParse(splitted[1], out int reg);
                    if (isReg) {
                        reg = splitted[1][0] - 'a';
                    }

                    bool offsetIsReg = !int.TryParse(splitted[2], out int offset);
                    if (offsetIsReg) {
                        offset = splitted[2][0] - 'a';
                    }

                    return new JumpNotZero(reg, offset, isReg, offsetIsReg);
                }
                case "tgl": {
                    int reg = splitted[1][0] - 'a';
                    return new Toggle(reg);
                }
                default:
                    throw new Exception("Unable to parse instruction");
            }
        })
        .ToArray();
}

Instruction[] instructions = ReadInstructions();

void ExecuteProgram(int[] registers, int programCounter = 0) {
    while (programCounter >= 0 && programCounter < instructions.Length) {
        instructions[programCounter].Execute(registers, ref programCounter, instructions);
        programCounter++;
    }
}

int[] registers = { 7, 0, 0, 0 };
ExecuteProgram(registers);
Console.WriteLine($"[Part 1] Value for safe: {registers[0]}");

// Part 2
instructions = ReadInstructions();
registers = new [] { 12 * 11, 11, 0, 0 };
ExecuteProgram(registers, 10);
Console.WriteLine($"[Part 2] Value for safe: {registers[0]}");
