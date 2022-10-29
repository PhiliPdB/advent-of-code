namespace day23;

internal abstract class Instruction {
    
    public abstract void Execute(int[] registers, ref int programCounter, Instruction[] instructions);

}

internal class Copy : Instruction {
    public int valX { get; init; }
    public int valY { get; init; }
    public bool xIsRegister { get; init; }
    public bool yIsRegister { get; init; }

    public Copy(int valX, int valY, bool xIsRegister, bool yIsRegister) {
        this.valX = valX;
        this.valY = valY;
        this.xIsRegister = xIsRegister;
        this.yIsRegister = yIsRegister;
    }

    public override void Execute(int[] registers, ref int programCounter, Instruction[] instructions) {
        if (!this.yIsRegister) {
            return;
        }

        registers[this.valY] = this.xIsRegister
            ? registers[this.valX]
            : this.valX;
    }
}

internal class Increase : Instruction {
    public int register { get; init; }

    public Increase(int register) {
        this.register = register;
    }

    public override void Execute(int[] registers, ref int programCounter, Instruction[] instructions) {
        registers[this.register]++;
    }
}

internal class Decrease : Instruction {
    public int register { get; init; }
    public bool isRegister { get; init; }

    public Decrease(int register) {
        this.register = register;
    }

    public override void Execute(int[] registers, ref int programCounter, Instruction[] instructions) {
        registers[this.register]--;
    }
}

internal class JumpNotZero : Instruction {
    public int register { get; init; }
    public int offset { get; init; }
    public bool isRegister { get; init; }
    public bool offsetIsRegister { get; init; }

    public JumpNotZero(int register, int offset, bool isRegister, bool offsetIsRegister) {
        this.register = register;
        this.offset = offset;
        this.isRegister = isRegister;
        this.offsetIsRegister = offsetIsRegister;
    }

    public override void Execute(int[] registers, ref int programCounter, Instruction[] instructions) {
        if ((this.isRegister && registers[this.register] != 0)
            || (!this.isRegister && this.register != 0)
        ) {
            int offset = this.offsetIsRegister ? registers[this.offset] : this.offset;
            programCounter += offset - 1;
        }
    }
}

internal class Toggle : Instruction {
    public int register { get; init; }

    public Toggle(int register) {
        this.register = register;
    }

    public override void Execute(int[] registers, ref int programCounter, Instruction[] instructions) {
        int index = programCounter + registers[this.register];
        if (index < 0 || index >= instructions.Length) {
            return;
        }

        instructions[index] = instructions[index] switch {
            Copy copy => new JumpNotZero(copy.valX, copy.valY, copy.xIsRegister, copy.yIsRegister),
            Decrease decrease => new Increase(decrease.register),
            Increase increase => new Decrease(increase.register),
            JumpNotZero jumpNotZero => new Copy(jumpNotZero.register, jumpNotZero.offset, jumpNotZero.isRegister, jumpNotZero.offsetIsRegister),
            Toggle toggle => new Increase(toggle.register),
            _ => throw new ArgumentOutOfRangeException()
        };
    }
}
