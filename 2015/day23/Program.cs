
var instructions = File.ReadAllLines("./input.txt")
    .Select(line => {
        var split = line.Split(' ');
        return split[0] switch {
            "hlf" => new Instruction(OpCode.Hlf, split[1] == "a" ? Register.RegisterA : Register.RegisterB),
            "tpl" => new Instruction(OpCode.Tpl, split[1] == "a" ? Register.RegisterA : Register.RegisterB),
            "inc" => new Instruction(OpCode.Inc, split[1] == "a" ? Register.RegisterA : Register.RegisterB),
            "jmp" => new Instruction(OpCode.Jmp, 0, int.Parse(split[1])),
            "jie" => new Instruction(OpCode.Jie, split[1][0] == 'a' ? Register.RegisterA : Register.RegisterB, int.Parse(split[2])),
            "jio" => new Instruction(OpCode.Jio, split[1][0] == 'a' ? Register.RegisterA : Register.RegisterB, int.Parse(split[2])),
            _ => throw new Exception("Could not parse instruction")
        };
    })
    .ToArray();

void ExecuteProgram(int[] registers) {
    var instructionPointer = 0;
    while (instructionPointer >= 0 && instructionPointer < instructions.Length) {
        instructions[instructionPointer].Execute(registers, ref instructionPointer);
        instructionPointer++;
    }
}

var part1Registers = new[] { 0, 0 };
ExecuteProgram(part1Registers);
Console.WriteLine($"[Part 1] Register B: {part1Registers[(int) Register.RegisterB]}");

var part2Registers = new[] { 1, 0 };
ExecuteProgram(part2Registers);
Console.WriteLine($"[Part 2] Register B: {part2Registers[(int)Register.RegisterB]}");


internal enum OpCode {
    Hlf, Tpl, Inc, Jmp, Jie, Jio
}

internal enum Register {
    RegisterA, RegisterB,
}

internal struct Instruction {
    public OpCode opCode;
    public Register register;
    public int offset;

    public Instruction(OpCode opCode, Register register = 0, int offset = 0) {
        this.opCode = opCode;
        this.register = register;
        this.offset = offset;
    }

    public void Execute(int[] registers, ref int instructionPointer) {
        switch (this.opCode) {
            case OpCode.Hlf:
                registers[(int) this.register] /= 2;
                break;
            case OpCode.Tpl:
                registers[(int) this.register] *= 3;
                break;
            case OpCode.Inc:
                registers[(int) this.register]++;
                break;
            case OpCode.Jmp:
                instructionPointer += this.offset - 1;
                break;
            case OpCode.Jie:
                if (registers[(int) this.register] % 2 == 0) {
                    instructionPointer += this.offset  - 1;
                }
                break;
            case OpCode.Jio:
                if (registers[(int) this.register] == 1) {
                    instructionPointer += this.offset - 1;
                }
                break;
            default:
                throw new ArgumentOutOfRangeException();
        }
    }
}
