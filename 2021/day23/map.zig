const std = @import("std");

const Node = @import("main.zig").Node;
const Space = @import("space.zig").Space;

pub fn Map(comptime n: usize) type {
    return struct {
        hallway: [11]Space,
        rooms: [4][n]Space,

        fn distanceToRoom(hallwayIndex: usize, roomIndex: usize, roomSlot: usize) u32 {
            const i32HallwayIndex: i32 = @intCast(hallwayIndex);
            const i32RoomHallwayIndex: i32 = @intCast((roomIndex + 1) * 2);

            const hallwayDistance = @abs(i32HallwayIndex - i32RoomHallwayIndex);
            const roomSlotDistance: u32 = @intCast(n - roomSlot);

            return hallwayDistance + roomSlotDistance;
        }

        fn isBlocking(self: *const @This(), hallwayIndex: usize, roomIndex: usize) bool {
            const roomHallwayIndex = (roomIndex + 1) * 2;
            const hallwayRange =
                if (hallwayIndex < roomHallwayIndex)
                    self.hallway[(hallwayIndex + 1)..(roomHallwayIndex + 1)]
                else
                    self.hallway[roomHallwayIndex..hallwayIndex];

            for (hallwayRange) |space| {
                if (space != .Open) {
                    return true;
                }
            }
            return false;
        }

        pub fn isFinished(self: *const @This()) bool {
            const order = [_]Space{ .Amber, .Bronze, .Copper, .Desert };
            inline for (self.rooms, 0..) |room, r| {
                for (room) |space| {
                    if (space != order[r]) {
                        return false;
                    }
                }
            }
            return true;
        }

        pub fn generateMoves(self: *const @This(), alloc: std.mem.Allocator) !std.ArrayList(Node(n)) {
            var moves = std.ArrayList(Node(n)).empty;

            // Move from hallway back to room?
            hallwayFor: for (self.hallway, 0..) |space, i| {
                switch (space) {
                    .Open => continue,
                    else => {
                        if (self.isBlocking(i, space.roomIndex())) {
                            continue;
                        }

                        const room = &self.rooms[space.roomIndex()];
                        var roomSlot: usize = 0;
                        roomSearch: while (true) : (roomSlot += 1) {
                            if (roomSlot == n) {
                                continue :hallwayFor;
                            }

                            if (room[roomSlot] != .Open) {
                                continue :roomSearch;
                            }

                            // All lower numbers should be same as space
                            for (room[0..roomSlot]) |s| {
                                if (s != space) {
                                    continue :roomSearch;
                                }
                            }

                            break;
                        }

                        var newMap = self.*;
                        newMap.hallway[i] = .Open;
                        newMap.rooms[space.roomIndex()][roomSlot] = space;

                        try moves.append(alloc, .{
                            .map = newMap,
                            .score = distanceToRoom(i, space.roomIndex(), roomSlot) * space.multiplier(),
                        });
                    },
                }
            }

            // Hallway moves
            const hallwayIndices = [_]usize{ 0, 1, 3, 5, 7, 9, 10 };
            for (hallwayIndices) |hallwayIndex| {
                if (self.hallway[hallwayIndex] != .Open) {
                    continue;
                }

                roomFor: for (0..4) |room| {
                    if (self.isBlocking(hallwayIndex, room)) {
                        continue;
                    }

                    var roomSlot: usize = 0;
                    slotSearch: while (true) : (roomSlot += 1) {
                        if (roomSlot == n) {
                            continue :roomFor;
                        }

                        if (self.rooms[room][roomSlot] == .Open) {
                            continue :slotSearch;
                        }

                        for (self.rooms[room][roomSlot + 1 ..]) |space| {
                            if (space != .Open) {
                                continue :slotSearch;
                            }
                        }

                        break;
                    }

                    var newMap = self.*;
                    newMap.hallway[hallwayIndex] = self.rooms[room][roomSlot];
                    newMap.rooms[room][roomSlot] = .Open;

                    try moves.append(alloc, .{
                        .map = newMap,
                        .score = distanceToRoom(hallwayIndex, room, roomSlot) * self.rooms[room][roomSlot].multiplier(),
                    });
                }
            }

            return moves;
        }

        pub fn fromString(comptime s: []const u8) @This() {
            var lines: [3 + 2 * n][]const u8 = undefined;

            var iter = std.mem.tokenizeScalar(u8, s, '\n');
            var i: usize = 0;
            while (iter.next()) |line| : (i += 1) {
                lines[i] = line;
            }

            var rooms: [4][n]Space = undefined;
            for (2..(2 + n)) |lineIndex| {
                const roomIndices = [_]usize{ 3, 5, 7, 9 };
                for (roomIndices, 0..) |roomIndex, r| {
                    rooms[r][n - (lineIndex - 2) - 1] = Space.fromChar(lines[lineIndex][roomIndex]) orelse @panic("Invalid character in room");
                }
            }

            return .{
                .hallway = @splat(.Open),
                .rooms = rooms,
            };
        }
    };
}

