const std = @import("std");

pub fn main(init: std.process.Init) !void {
    // Program setup
    const io = init.io;
    // Get a writer for stdout
    var stdout_writer = std.Io.File.stdout().writer(io, &.{});
    const stdout = &stdout_writer.interface;

    // Allocator stuff
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = &arena.allocator();

    //
    // Solving
    //

    const input = @embedFile("input.txt");
}
