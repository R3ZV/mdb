const std = @import("std");
const Lexer = @import("lexer.zig").Lexer;
const html = @import("html.zig");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const alloc = gpa.allocator();

    const buff =
        \\# Foo
        \\## bar
        \\### bar3
        \\#### bar4
        \\##### bar5
        \\####### foobar #bazz
        \\some text
        \\1.
        \\2. list
        \\- todo
    ;

    const lexer = Lexer.new(buff, alloc);
    const tokens = lexer.tokens();
    defer tokens.deinit();

    const page = try html.create_html(tokens.items);
    std.debug.print("Html:\n{s}", .{page});
}

test {
    _ = @import("lexer.zig");
}
