const std = @import("std");
const Token = @import("token.zig").Token;

pub fn create_html(tokens: []const Token) ![]const u8 {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const alloc = gpa.allocator();

    var i: usize = 0;
    var html: []u8 = "";
    while (i < tokens.len) : (i += 1) {
        var fmt: []u8 = undefined;
        // TODO: rewrite this, you can tabel switch and break a value from it
        // TODO: add identation
        switch (tokens[i].token_type) {
            .H1 => fmt = try std.fmt.allocPrint(alloc, "<h1>{s}</h1>\n", .{tokens[i].content}),
            .H2 => fmt = try std.fmt.allocPrint(alloc, "<h2>{s}</h2>\n", .{tokens[i].content}),
            .H3 => fmt = try std.fmt.allocPrint(alloc, "<h3>{s}</h3>\n", .{tokens[i].content}),
            .H4 => fmt = try std.fmt.allocPrint(alloc, "<h4>{s}</h4>\n", .{tokens[i].content}),
            .H5 => fmt = try std.fmt.allocPrint(alloc, "<h5>{s}</h5>\n", .{tokens[i].content}),
            .H6 => fmt = try std.fmt.allocPrint(alloc, "<h6>{s}</h6>\n", .{tokens[i].content}),
            .NewLine => fmt = try std.fmt.allocPrint(alloc, "<br/>\n", .{}),
            .P => {
                var aux: []u8 = "";
                aux = try std.mem.concat(alloc, u8, &[2][]const u8{ aux, "<p>\n" });

                while (i < tokens.len and tokens[i].token_type == .P) : (i += 1) {
                    const p = try std.fmt.allocPrint(alloc, "{s}\n", .{tokens[i].content});
                    aux = try std.mem.concat(alloc, u8, &[2][]const u8{ aux, p });
                }

                i -= 1;

                fmt = try std.mem.concat(alloc, u8, &[2][]const u8{ aux, "</p>\n" });
            },
            .Ol => {
                var aux: []u8 = "";
                aux = try std.mem.concat(alloc, u8, &[2][]const u8{ aux, "<ol>\n" });

                while (i < tokens.len and tokens[i].token_type == .Ol) : (i += 1) {
                    const item = try std.fmt.allocPrint(alloc, "<li>{s}</li>\n", .{tokens[i].content});
                    aux = try std.mem.concat(alloc, u8, &[2][]const u8{ aux, item });
                }

                i -= 1;

                fmt = try std.mem.concat(alloc, u8, &[2][]const u8{ aux, "</ol>\n" });
            },
            .Ul => {
                var aux: []u8 = "";
                aux = try std.mem.concat(alloc, u8, &[2][]const u8{ aux, "<ul>\n" });

                while (i < tokens.len and tokens[i].token_type == .Ul) : (i += 1) {
                    const item = try std.fmt.allocPrint(alloc, "<li>{s}</li>\n", .{tokens[i].content});
                    aux = try std.mem.concat(alloc, u8, &[2][]const u8{ aux, item });
                }

                i -= 1;

                fmt = try std.mem.concat(alloc, u8, &[2][]const u8{ aux, "</ul>\n" });
            },
            .Code => unreachable,
            .Link => unreachable,
            .Quote => unreachable,
            .Image => unreachable,
            .Illegal => unreachable,
        }

        html = try std.mem.concat(alloc, u8, &[2][]const u8{ html, fmt });
    }

    return html;
}
