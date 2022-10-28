using day12;

Instruction[] instructions = File.ReadAllLines("./input.txt")
    .Select<string, Instruction>(line => {
        string[] splitted = line.Split(' ');
        switch (splitted[0]) {
            case "cpy": {
                bool xIsReg = !int.TryParse(splitted[1], out int xReg);
                int yReg = splitted[2][0] - 'a';
                if (xIsReg) {
                    xReg = splitted[1][0] - 'a';
                }
                return new Copy(xReg, yReg, xIsReg);
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
                int offset = int.Parse(splitted[2]);
                return new JumpNotZero(reg, offset, isReg);
            }
            default:
                throw new Exception("Unable to parse instruction");
        }
    })
    .ToArray();


void ExecuteProgram(int[] registers) {
    var programCounter = 0;
    while (programCounter >= 0 && programCounter < instructions.Length) {
        instructions[programCounter].Execute(registers, ref programCounter);
        programCounter++;
    }
}

// Part 1
int[] registers = { 0, 0, 0, 0 };
ExecuteProgram(registers);
Console.WriteLine($"[Part 1] Value in register 'a': {registers[0]}");

// Part 2
registers = new[] { 0, 0, 1, 0 };
ExecuteProgram(registers);
Console.WriteLine($"[Part 2] Value in register 'a': {registers[0]}");
