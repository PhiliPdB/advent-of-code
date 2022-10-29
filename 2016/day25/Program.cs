using day25;

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
                case "out": {
                    bool isReg = !int.TryParse(splitted[1], out int reg);
                    if (isReg) {
                        reg = splitted[1][0] - 'a';
                    }

                    return new Out(reg, isReg);
                }
                default:
                    throw new Exception("Unable to parse instruction");
            }
        })
        .ToArray();
}

Instruction[] instructions = ReadInstructions();

bool ExecuteProgram(int aValue) {
    int[] registers = { aValue, 0, 0, 0 };
    var programCounter = 0;

    const int maxOutputs = 100;
    var currentOutputs = 0;
    var nextOutput = 0;

    while (programCounter >= 0 && programCounter < instructions.Length && currentOutputs < maxOutputs) {
        var result = instructions[programCounter].Execute(registers, ref programCounter);
        if (result != null) {
            if (result == nextOutput) {
                nextOutput = nextOutput == 0 ? 1 : 0;
                currentOutputs++;
            } else {
                return false;
            }
        }

        programCounter++;
    }

    return true;
}


var aValue = 0;
while (true) {
    if (ExecuteProgram(aValue)) {
        Console.WriteLine($"Found value: {aValue}");
        return;
    }
    aValue++;
}
