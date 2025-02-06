use crate::token::{Token, TokenType};

pub fn create_html(tokens: &Vec<Token>) -> String {
    let mut html = String::new();
    for token in tokens {
        // TODO: add identation
        match token.ttype() {
            TokenType::H1 => html += format!("<h1>{}</h1>\n", token.content()).as_str(),
            TokenType::H2 => html += format!("<h2>{}</h2>\n", token.content()).as_str(),
            TokenType::H3 => html += format!("<h3>{}</h3>\n", token.content()).as_str(),
            TokenType::H4 => html += format!("<h4>{}</h4>\n", token.content()).as_str(),
            TokenType::H5 => html += format!("<h5>{}</h5>\n", token.content()).as_str(),
            TokenType::H6 => html += format!("<h6>{}</h6>\n", token.content()).as_str(),
            TokenType::P => {
                todo!();
                // var aux: []u8 = "";
                // aux = try std.mem.concat(alloc, u8, &[2][]const u8{ aux, "<p>\n" });

                // while (i < tokens.len and tokens[i].token_type == .P) : (i += 1) {
                //     const p = try std.fmt.allocPrint(alloc, "{s}\n", .{tokens[i].content});
                //     aux = try std.mem.concat(alloc, u8, &[2][]const u8{ aux, p });
                // }

                // i -= 1;

                // fmt = try std.mem.concat(alloc, u8, &[2][]const u8{ aux, "</p>\n" });
            }
            TokenType::Ol => {
                todo!();
                // var aux: []u8 = "";
                // aux = try std.mem.concat(alloc, u8, &[2][]const u8{ aux, "<ol>\n" });

                // while (i < tokens.len and tokens[i].token_type == .Ol) : (i += 1) {
                //     const item = try std.fmt.allocPrint(alloc, "<li>{s}</li>\n", .{tokens[i].content});
                //     aux = try std.mem.concat(alloc, u8, &[2][]const u8{ aux, item });
                // }

                // i -= 1;

                // fmt = try std.mem.concat(alloc, u8, &[2][]const u8{ aux, "</ol>\n" });
            }
            TokenType::Ul => {
                todo!();
                // var aux: []u8 = "";
                // aux = try std.mem.concat(alloc, u8, &[2][]const u8{ aux, "<ul>\n" });

                // while (i < tokens.len and tokens[i].token_type == .Ul) : (i += 1) {
                //     const item = try std.fmt.allocPrint(alloc, "<li>{s}</li>\n", .{tokens[i].content});
                //     aux = try std.mem.concat(alloc, u8, &[2][]const u8{ aux, item });
                // }

                // i -= 1;

                // fmt = try std.mem.concat(alloc, u8, &[2][]const u8{ aux, "</ul>\n" });
            }
            TokenType::Eof => continue,
            TokenType::Code => todo!(),
            TokenType::Link => todo!(),
            TokenType::Quote => todo!(),
            TokenType::Image => todo!(),
            TokenType::Illegal => todo!(),
        }
    }

    html
}
