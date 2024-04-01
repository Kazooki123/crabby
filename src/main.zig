const std = @import("std");
const Tokenizer = @import("Tokenizer.zig");

const MaxFileSize = std.math.maxInt(usize);

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();

    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    // There must be always atleast one argument, the executable
    std.debug.assert(args.len >= 1);

    if (args.len < 2) {
        std.log.err("Error! You must specify a file name! {s} <filename>", .{args[0]});
        return;
    }

    const filepath = args[1];

    const file = try if (std.fs.path.isAbsolute(filepath))
        std.fs.openFileAbsolute(filepath, .{})
    else
        std.fs.cwd().openFile(filepath, .{});
    defer file.close();

    const source = try file.readToEndAlloc(allocator, MaxFileSize);
    defer allocator.free(source);

    var tokenizer = Tokenizer{ .source = source, .index = 0 };

    while (true) {
        const token = try tokenizer.nextToken();
        std.debug.print("Token: '{}'\n", .{token});

        if (token == .EOF)
            break;
    }

    // Print "Hello, World!"
    std.debug.print("Hello, World!\n", .{});
}
