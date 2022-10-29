namespace day25;

internal abstract class Instruction {
    
    public abstract int? Execute(int[] registers, ref int programCounter);

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

    public override int? Execute(int[] registers, ref int programCounter) {
        if (!this.yIsRegister) {
            return null;
        }

        registers[this.valY] = this.xIsRegister
            ? registers[this.valX]
            : this.valX;
        return null;
    }
}

internal class Increase : Instruction {
    public int register { get; init; }

    public Increase(int register) {
        this.register = register;
    }

    public override int? Execute(int[] registers, ref int programCounter) {
        registers[this.register]++;
        return null;
    }
}

internal class Decrease : Instruction {
    public int register { get; init; }
    public bool isRegister { get; init; }

    public Decrease(int register) {
        this.register = register;
    }

    public override int? Execute(int[] registers, ref int programCounter) {
        registers[this.register]--;
        return null;
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

    public override int? Execute(int[] registers, ref int programCounter) {
        if ((this.isRegister && registers[this.register] != 0)
            || (!this.isRegister && this.register != 0)
        ) {
            int offset = this.offsetIsRegister ? registers[this.offset] : this.offset;
            programCounter += offset - 1;
        }

        return null;
    }
}

internal class Out : Instruction {
    public int register { get; init; }
    public bool isRegister { get; init; }

    public Out(int register, bool isRegister) {
        this.register = register;
        this.isRegister = isRegister;
    }

    public override int? Execute(int[] registers, ref int programCounter) {
        return this.isRegister ? registers[this.register] : this.register;
    }
}
