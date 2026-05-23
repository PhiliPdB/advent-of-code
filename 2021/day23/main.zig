const std = @import("std");

const Map = @import("map.zig").Map;

pub fn Node(comptime n: usize) type {
    return struct {
        map: Map(n),
        score: u32,

        pub fn lessThan(context: void, a: @This(), b: @This()) std.math.Order {
            _ = context;
            return std.math.order(a.score, b.score);
        }
    };
}

fn getMinScore(comptime n: usize, map: Map(n), alloc: *const std.mem.Allocator) !u32 {
    var visited = std.AutoHashMap(Map(n), void).init(alloc.*);
    defer visited.deinit();

    var queue = std.PriorityDequeue(
        Node(n),
        void,
        Node(n).lessThan,
    ).empty;
    defer queue.deinit(alloc.*);

    try queue.push(alloc.*, .{ .map = map, .score = 0 });

    while (queue.popMin()) |node| {
        // Are we done?
        if (node.map.isFinished()) {
            return node.score;
        }

        // Already visited?
        if (visited.get(node.map)) |_| {
            continue;
        }
        try visited.put(node.map, {});

        // Generate new moves
        var moves = try node.map.generateMoves(alloc.*);
        for (moves.items) |move| {
            try queue.push(alloc.*, .{ .map = move.map, .score = node.score + move.score });
        }
        moves.deinit(alloc.*);
    }

    unreachable;
}

pub fn main(init: std.process.Init) !void {
    const io = init.io;
    // Get a writer for stdout
    var stdout_writer = std.Io.File.stdout().writer(io, &.{});
    const stdout = &stdout_writer.interface;

    // Allocator stuff
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = &arena.allocator();

    // Solving

    const part1_input = @embedFile("input.txt");
    const part1_map = comptime Map(2).fromString(part1_input);
    try stdout.print("[Part 1] Result: {}\n", .{try getMinScore(2, part1_map, allocator)});

    const part2_input = @embedFile("input_part2.txt");
    const part2_map = comptime Map(4).fromString(part2_input);
    try stdout.print("[Part 2] Result: {}\n", .{try getMinScore(4, part2_map, allocator)});
}
