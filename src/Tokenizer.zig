const std = @import("std");

const Self = @This();

const Token = union(enum) {
    Identifier: []const u8,
    String: []const u8,
    Keyword: Keyword,
    LParen, // Left Parenthesis
    RParen, // Right Parenthesis
    Dot,
    Semicolon,
    EOF,
};
const Keyword = enum { fun, @"var", boolean };

source: []const u8,
index: usize,

// Function to get the next token
const NextError = error{ UnterminatedString, UnknownCharacter };
pub fn nextToken(self: *Self) !Token {
    self.skipWhitespace();

    if (self.isEOF())
        return .EOF;

    const start = self.index;
    const char = self.advance();

    return switch (char) {
        // Handle string literals
        '"' => {
            while (!self.isEOF() and self.advance() != '"') {}

            if (self.isEOF())
                return NextError.UnterminatedString;

            return .{ .String = self.source[start..self.index] };
        },
        ';' => .Semicolon,
        '.' => .Dot,
        '(' => .LParen,
        ')' => .RParen,
        'a'...'z', 'A'...'Z' => {
            while (std.ascii.isAlphanumeric(self.peek())) : (_ = self.advance()) {}

            const str = self.source[start..self.index];

            return if (std.meta.stringToEnum(Keyword, str)) |kw|
                Token{ .Keyword = kw }
            else
                Token{ .Identifier = str };
        },
        else => NextError.UnknownCharacter,
    };
}

pub inline fn skipWhitespace(self: *Self) void {
    while (true) {
        if (self.isEOF())
            return;

        switch (self.source[self.index]) {
            ' ', '\t', '\n', '\r' => _ = self.advance(),
            else => return,
        }
    }
}

inline fn advance(self: *Self) u8 {
    defer self.index += 1;
    return self.peek();
}

inline fn peek(self: *Self) u8 {
    return self.source[self.index];
}

inline fn isEOF(self: Self) bool {
    return self.index >= self.source.len;
}
