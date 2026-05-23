pub const Space = enum {
    Open,
    Amber,
    Bronze,
    Copper,
    Desert,

    pub fn fromChar(c: u8) ?@This() {
        return switch (c) {
            '.' => .Open,
            'A' => .Amber,
            'B' => .Bronze,
            'C' => .Copper,
            'D' => .Desert,
            else => null,
        };
    }

    pub fn multiplier(self: *const @This()) u32 {
        return switch (self.*) {
            .Open => @panic("Can't move to an open space"),
            .Amber => 1,
            .Bronze => 10,
            .Copper => 100,
            .Desert => 1000,
        };
    }

    pub fn roomIndex(self: *const @This()) usize {
        return switch (self.*) {
            .Open => @panic("No room"),
            .Amber => 0,
            .Bronze => 1,
            .Copper => 2,
            .Desert => 3,
        };
    }
};
