namespace day12;

internal abstract class Instruction {
    
    public abstract void Execute(int[] registers, ref int programCounter);

}

internal class Copy : Instruction {
    private int valX;
    private int valY;
    private bool xIsRegister;

    public Copy(int valX, int valY, bool xIsRegister) {
        this.valX = valX;
        this.valY = valY;
        this.xIsRegister = xIsRegister;
    }

    public override void Execute(int[] registers, ref int programCounter) {
        registers[this.valY] = this.xIsRegister
            ? registers[this.valX]
            : this.valX;
    }
}

internal class Increase : Instruction {
    private int register;

    public Increase(int register) {
        this.register = register;
    }

    public override void Execute(int[] registers, ref int programCounter) {
        registers[this.register]++;
    }
}

internal class Decrease : Instruction {
    private int register;

    public Decrease(int register) {
        this.register = register;
    }

    public override void Execute(int[] registers, ref int programCounter) {
        registers[this.register]--;
    }
}

internal class JumpNotZero : Instruction {
    private int register;
    private int offset;
    private bool isRegister;

    public JumpNotZero(int register, int offset, bool isRegister) {
        this.register = register;
        this.offset = offset;
        this.isRegister = isRegister;
    }

    public override void Execute(int[] registers, ref int programCounter) {
        if ((this.isRegister && registers[this.register] != 0)
            || (!this.isRegister && this.register != 0)
        ) {
            programCounter += this.offset - 1;
        }
    }
}
