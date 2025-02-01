const std = @import("std");
const Lexer = @import("lexer.zig").Lexer;
const html = @import("html.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const alloc = gpa.allocator();

    // TODO: use concat?
    const path = "example/articles/blog1.md";
    var buff: [1024]u8 = undefined;
    _ = try std.fs.cwd().readFile(path, &buff);

    const lexer = Lexer.new(&buff, alloc);
    const tokens = lexer.tokens();
    defer tokens.deinit();

    for (tokens.items) |token| {
        std.debug.print("Type: '{}', Content: '{s}'\n", .{ token.token_type, token.content });
    }

    const page = try html.create_html(tokens.items);
    std.debug.print("Html:\n{s}", .{page});
}

test {
    _ = @import("lexer.zig");
}
