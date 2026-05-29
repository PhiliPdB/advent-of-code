const std = @import("std");

const part1Days: u32 = 80;
const part2Days: u32 = 256;

fn getFishCount(input: []const u8) [9]u64 {
    @setEvalBranchQuota(25_000);
    var fishCount: [9]u64 = @splat(0);

    var iter = std.mem.tokenizeAny(u8, input, ",\n");
    while (iter.next()) |fish| {
        const fishIndex = std.fmt.parseInt(u8, fish, 10) catch @compileError("Invalid fish age");
        fishCount[fishIndex] += 1;
    }

    return fishCount;
}

fn simulateDays(initialFishCount: [9]u64, comptime days: u32) [9]u64 {
    var fishCount = initialFishCount;

    for (0..days) |_| {
        const newFish = fishCount[0];

        // Decrease by 1 day
        inline for (0..8) |i| {
            fishCount[i] = fishCount[i + 1];
        }

        // Create new fish
        fishCount[8] = newFish;

        // Reset fishes that reached day 0
        fishCount[6] += newFish;
    }

    return fishCount;
}

fn totalFish(fishCount: [9]u64) u64 {
    var total: u64 = 0;
    for (fishCount) |count| {
        total += count;
    }
    return total;
}

pub fn main(init: std.process.Init) !void {
    // Program setup
    const io = init.io;
    // Get a writer for stdout
    var stdout_writer = std.Io.File.stdout().writer(io, &.{});
    const stdout = &stdout_writer.interface;

    //
    // Solving
    //

    const input = @embedFile("input.txt");
    const initialFish = comptime getFishCount(input);

    const part1Answer = comptime totalFish(simulateDays(initialFish, part1Days));
    try stdout.print("[Part 1] Total fish: {d:>13}\n", .{part1Answer});

    const part2Answer = comptime totalFish(simulateDays(initialFish, part2Days));
    try stdout.print("[Part 2] Total fish: {d:>13}\n", .{part2Answer});
}
