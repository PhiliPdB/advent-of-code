const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Define options for the year and day to run.
    const year = b.option(u16, "year", "The year of the challenge to run");
    const day = b.option(u8, "day", "The day of the challenge to run");

    if (year == null or day == null) {
        std.log.err("Please provide both -Dyear=<year> and -Dday=<day> options.\n", .{});
        return;
    }

    // Define constants for executable name and path
    const name = b.fmt("{d:0>4}_day{d:0>2}", .{ year.?, day.? });
    const main_path = b.path(b.fmt("{d:0>4}/day{d:0>2}/main.zig", .{ year.?, day.? }));

    // Add executable target
    const exe = b.addExecutable(.{
        .name = name,
        .root_module = b.createModule(.{
            .root_source_file = main_path,
            .target = target,
            .optimize = optimize,
        }),
    });
    b.installArtifact(exe);

    // Add step to directly run the program
    const run_step = b.step("run", "Run the program");
    const run_cmd = b.addRunArtifact(exe);
    run_step.dependOn(&run_cmd.step);
    run_cmd.step.dependOn(b.getInstallStep());

    // Pass through additional arguments
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    // Step to generate a new day from the template main.zig in templates/
    const generate_step = b.step("generate", "Generate a new year/day");
    const generate_cmd = b.addWriteFile(main_path.getPath(b), @embedFile("templates/main.zig"));
    generate_step.dependOn(&generate_cmd.step);
}
