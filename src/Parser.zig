const std = @import("std");

const Tokenizer = struct {
    const Self = @This();

    source: []const u8,
    index: usize,

    // Function to get the next token
    pub fn nextToken(self: *Self) []const u8 {
        // Skip whitespace
        while (self.index < self.source.len and std.ascii.isWhitespace(self.source[self.index])) {
            self.index += 1;
        }

        if (self.index >= self.source.len) return null;

        const char = self.source[self.index];
        switch (char) {
            // Handle string literals
            '"' => {
                const start = self.index;
                self.index += 1;
                while (self.index < self.source.len and self.source[self.index] != '"') {
                    self.index += 1;
                }
                if (self.index < self.source.len and self.source[self.index] == '"') {
                    self.index += 1; // Skip the closing quote
                    return self.source[start..self.index];
                } else {
                    std.debug.print("Error: Unterminated string literal.\\n", .{});
                    return null;
                }
            },

            // Handle semicolon
            ';' => {
                const start = self.index;
                self.index += 1;
                return self.source[start..self.index];
            },

            // Handle keywords
            else => {
                if (std.mem.startsWith(u8, self.source[self.index..], "boolean")) {
                    const start = self.index;
                    self.index += "boolean".len;
                    return self.source[start..self.index];
                } else if (std.mem.startsWith(u8, self.source[self.index..], "var")) {
                    const start = self.index;
                    self.index += "var".len;
                    return self.source[start..self.index];
                } else if (std.mem.startsWith(u8, self.source[self.index..], "fun")) {
                    const start = self.index;
                    self.index += "fun".len;
                    return self.source[start..self.index];
                } else {
                    // Handle other characters (error case)
                    std.debug.print("Error: Unexpected character: '{}'.\\n", .{char});
                    return null;
                }
            }
        }
    }
};

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    const file_path = "hello.cb"; // Change this to your actual file path
    const file = try std.fs.cwd().openFile(file_path, .{});
    defer file.close();

    const source = try file.readToEndAlloc(allocator, 10 * 1024); // Read up to 10KB
    defer allocator.free(source);

    var tokenizer = Tokenizer{ .source = source, .index = 0 };
    while (true) {
        const token = tokenizer.nextToken();
        if (token == null) break;

        // Print each token
        std.debug.print("Token: '{}\\n'", .{token});
    }

    // Print "Hello, World!"
    std.debug.print("Hello, World!\\n", .{});
}
