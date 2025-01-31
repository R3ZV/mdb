const std = @import("std");

const Token = @import("token.zig").Token;
const TokenType = @import("token.zig").TokenType;

pub const Lexer = struct {
    const Self = @This();

    input: []const u8,
    alloc: std.mem.Allocator,

    pub fn new(input: []const u8, alloc: std.mem.Allocator) Lexer {
        return Self{
            .input = input,
            .alloc = alloc,
        };
    }

    pub fn tokens(self: *const Self) std.ArrayList(Token) {
        var toks = std.ArrayList(Token).init(self.alloc);
        var it = std.mem.split(u8, self.input, "\n");
        while (it.next()) |line| {
            const token = parse(line);
            toks.append(token) catch {};
        }

        return toks;
    }

    fn parse(line: []const u8) Token {
        if (std.mem.eql(u8, line, "")) {
            return Token.new(TokenType.NewLine, "");
        }

        if (line[0] == '#') {
            return parse_header(line);
        }

        if (std.ascii.isAlphabetic(line[0])) {
            return Token.new(.Text, line);
        }

        if (line[0] == '-') {
            return Token.new(.Ul, line[1..]);
        }

        if (line.len >= 2 and std.ascii.isAlphanumeric(line[0])) {
            return Token.new(.Ol, line[2..]);
        }

        return Token.new(.Illegal, "");
    }

    fn parse_header(line: []const u8) Token {
        var level: usize = 0;
        var i: usize = 0;
        while (line[i] == '#') : (i += 1) level += 1;
        while (line[i] == ' ') : (i += 1) {}
        return Token.new(TokenType.header_level(level), line[i..]);
    }
};

test {
    const testing = std.testing;
    const alloc = testing.allocator;

    const buff =
        \\# Foo
        \\## bar
        \\####### foobar #bazz
        \\some text
        \\1.
        \\2. list
        \\- todo
    ;

    const lexer = Lexer.new(buff, alloc);
    const tokens = lexer.tokens();
    defer tokens.deinit();

    const want = .{
        Token.new(.H1, "Foo"),
        Token.new(.H2, "bar"),
        Token.new(.H6, "foobar #bazz"),
        Token.new(.Text, "some text"),
        Token.new(.Ol, ""),
        Token.new(.Ol, " list"),
        Token.new(.Ul, " todo"),
    };

    try testing.expectEqual(want.len, tokens.items.len);
    for (tokens.items, want) |token, want_token| {
        try testing.expectEqual(want_token.content, token.content);
        try testing.expectEqual(want_token.token_type, token.token_type);
    }
}
