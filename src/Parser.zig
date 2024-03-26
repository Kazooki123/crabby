const std = @import("std");

const Tokenizer = struct {
    const Self = @This();

    source: []const u8,
    index: usize,

    fn nextToken(self: *Self) !?[]const u8 {
        while (self.index < self.source.len and std.mem.isWhiteSpace(self.source[self.index])) {
            self.index += 1;
        }

        if (self.index >= self.source.len) return null;

        if (std.mem.startsWith(u8, self.source[self.index..], "output.print")) {
            const start = self.index;
            self.index += "output.print".len;
            return self.source[start..self.index];
        }

        if (self.source[self.index] == '"') {
            const start = self.index;
            self.index += 1;
            while (self.index < self.source.len and self.source[self.index] != '"') {
                self.index += 1;
            }
            if (self.index < self.source.len) {
                self.index += 1; // Skip the closing quote
                return self.source[start..self.index];
            } else {
                std.debug.print("Error: Unterminated string literal.\n", .{});
                return error.UnterminatedStringLiteral;
            }
        }

        if (self.source[self.index] == ';') {
            const start = self.index;
            self.index += 1;
            return self.source[start..self.index];
        }

        std.debug.print("Error: Unexpected character: '{}'.\n", .{self.source[self.index]});
        return error.UnexpectedCharacter;
    }
};

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    const file_path = "hello.cb";
    const file = try std.fs.cwd().openFile(file_path, .{});
    defer file.close();

    const source = try file.readToEndAlloc(allocator, 10 * 1024); // Read up to 10KB
    defer allocator.free(source);

    var tokenizer = Tokenizer{ .source = source, .index = 0 };
    while (true) {
        const token = try tokenizer.nextToken();
        if (token == null) break;

        std.debug.print("Token: '{}'\n", .{token});
    }
}
