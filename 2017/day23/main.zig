const std = @import("std");

fn lineCount(comptime text: []const u8) usize {
    var n = 0;
    var iter = std.mem.tokenizeScalar(u8, text, '\n');
    while (iter.next()) |_| {
        n += 1;
    }
    return n;
}

const Arg = union(enum) {
    reg: u8,
    val: i32,
};

const Operation = enum {
    set,
    sub,
    mul,
    jnz,
};

const Instruction = struct {
    op: Operation,
    x: Arg,
    y: Arg,
};

fn parseArg(comptime token: []const u8) Arg {
    // Check length = 1, and is ascii
    if (token.len == 1 and std.ascii.isAlphabetic(token[0])) {
        return Arg{ .reg = token[0] - 'a' };
    } else {
        const val = std.fmt.parseInt(i32, token, 10) catch @compileError("Invalid argument");
        return Arg{ .val = val };
    }
}

fn parseProgram(comptime text: []const u8) [lineCount(text)]Instruction {
    @setEvalBranchQuota(100_000);
    var program: [lineCount(text)]Instruction = undefined;

    var iter = std.mem.tokenizeScalar(u8, text, '\n');
    var i: usize = 0;
    while (iter.next()) |line| : (i += 1) {
        var tokens = std.mem.tokenizeScalar(u8, line, ' ');
        const op_token = tokens.next() orelse @compileError("Invalid instruction");
        const x = tokens.next() orelse @compileError("Missing X argument");
        const y = tokens.next() orelse @compileError("Missing Y argument");

        const op: Operation =
            if (std.mem.eql(u8, op_token, "set"))
                .set
            else if (std.mem.eql(u8, op_token, "sub"))
                .sub
            else if (std.mem.eql(u8, op_token, "mul"))
                .mul
            else if (std.mem.eql(u8, op_token, "jnz"))
                .jnz
            else
                @compileError("Unknown operation");

        program[i] = Instruction{
            .op = op,
            .x = parseArg(x),
            .y = parseArg(y),
        };
    }

    return program;
}

fn runInstruction(comptime program: []const Instruction, ip: usize, registers: []i32, mul_count: *u32) usize {
    const instr = program[ip];
    switch (instr.op) {
        Operation.set => {
            const val = switch (instr.y) {
                .reg => registers[instr.y.reg],
                .val => instr.y.val,
            };
            registers[instr.x.reg] = val;
        },
        Operation.sub => {
            const val = switch (instr.y) {
                .reg => registers[instr.y.reg],
                .val => instr.y.val,
            };
            registers[instr.x.reg] -= val;
        },
        Operation.mul => {
            mul_count.* += 1;
            const val = switch (instr.y) {
                .reg => registers[instr.y.reg],
                .val => instr.y.val,
            };
            registers[instr.x.reg] *= val;
        },
        Operation.jnz => {
            const x_val = switch (instr.x) {
                .reg => registers[instr.x.reg],
                .val => instr.x.val,
            };

            if (x_val != 0) {
                const y_val = switch (instr.y) {
                    .reg => registers[instr.y.reg],
                    .val => instr.y.val,
                };
                const i32_ip: i32 = @intCast(ip);
                return @intCast(i32_ip + y_val);
            }
        },
    }
    return ip + 1;
}

fn runProgram(comptime program: []const Instruction, registers: []i32) u32 {
    var ip: usize = 0;
    var mul_count: u32 = 0;
    while (ip < program.len) {
        ip = runInstruction(program, ip, registers, &mul_count);
    }
    return mul_count;
}

fn isPrime(n: u32) bool {
    if (n % 2 == 0) {
        return false;
    }

    var p: u32 = 3;
    while (p < std.math.sqrt(n) + 1) : (p += 2) {
        if (n % p == 0) {
            return false;
        }
    }

    return true;
}

pub fn main(init: std.process.Init) !void {
    const io = init.io;
    // Get a writer for stdout
    var stdout_writer = std.Io.File.stdout().writer(io, &.{});
    const stdout = &stdout_writer.interface;

    const input = @embedFile("input.txt");
    const program = comptime parseProgram(input);

    // Part 1
    var part1_registers: [8]i32 = @splat(0);
    const mul_count = runProgram(&program, &part1_registers);
    try stdout.print("[Part 1] Mul instruction count: {}\n", .{mul_count});

    // Part 2
    // With input analysis
    var b: u32 = 99 * 100 + 100_000;
    const c: u32 = b + 17_000;
    var h: u32 = 0;
    while (b <= c) : (b += 17) {
        if (!isPrime(b)) {
            h += 1;
        }
    }
    try stdout.print("[Part 2] Register h: {}\n", .{h});
}
