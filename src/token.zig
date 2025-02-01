pub const TokenType = enum {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    P,
    Code,
    Link,
    Quote,
    Image,
    NewLine,
    Illegal,
    Ol,
    Ul,

    pub fn header_level(level: usize) TokenType {
        return switch (level) {
            1 => .H1,
            2 => .H2,
            3 => .H3,
            4 => .H4,
            5 => .H5,
            else => .H6,
        };
    }
};

pub const Token = struct {
    const Self = @This();

    token_type: TokenType,
    content: []const u8,

    pub fn new(tok_type: TokenType, content: []const u8) Token {
        return Self{ .token_type = tok_type, .content = content };
    }
};
