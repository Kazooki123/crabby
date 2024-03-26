const std = @import("std");

pub fn throwError(message: []const u8) !void {
    std.debug.print("Error: {}\n", .{message});
    return error.GenericError;
}

pub fn throwErrorWithLine(message: []const u8, line_number: usize) !void {
    std.debug.print("Error on line {}: {}\n", .{line_number, message});
    return error.GenericError;
}