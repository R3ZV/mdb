use crate::token::{Token, TokenType};

pub fn create_html(tokens: &Vec<Token>) -> String {
    let mut html = String::new();
    let mut i = 0;
    while i < tokens.len() {
        // TODO: add identation
        match tokens[i].ttype() {
            TokenType::H1 => html += format!("<h1>{}</h1>\n", tokens[i].content()).as_str(),
            TokenType::H2 => html += format!("<h2>{}</h2>\n", tokens[i].content()).as_str(),
            TokenType::H3 => html += format!("<h3>{}</h3>\n", tokens[i].content()).as_str(),
            TokenType::H4 => html += format!("<h4>{}</h4>\n", tokens[i].content()).as_str(),
            TokenType::H5 => html += format!("<h5>{}</h5>\n", tokens[i].content()).as_str(),
            TokenType::H6 => html += format!("<h6>{}</h6>\n", tokens[i].content()).as_str(),
            TokenType::P => {
                let mut para = String::from("<p>\n");

                while i < tokens.len() && tokens[i].ttype() == TokenType::P {
                    para.push_str(tokens[i].content().as_str());
                    i += 1;
                }
                i -= 1;

                para.push_str("\n</p>\n");
                html.push_str(para.as_str());
            }
            TokenType::Ol => {
                let mut list = String::from("<ol>\n");

                while i < tokens.len() && tokens[i].ttype() == TokenType::Ol {
                    list.push_str(format!("<li>{}</li>\n", tokens[i].content()).as_str());
                    i += 1;
                }
                i -= 1;

                list.push_str("</ol>\n");
                html.push_str(list.as_str());
            }
            TokenType::Ul => {
                let mut list = String::from("<ul>\n");

                while i < tokens.len() && tokens[i].ttype() == TokenType::Ul {
                    list.push_str(format!("<li>{}</li>\n", tokens[i].content()).as_str());
                    i += 1;
                }
                i -= 1;

                list.push_str("</ul>\n");
                html.push_str(list.as_str());
            }
            TokenType::Eof => break,
            TokenType::Code => todo!(),
            TokenType::Link => todo!(),
            TokenType::Quote => todo!(),
            TokenType::Image => todo!(),
            TokenType::Illegal => todo!(),
        }

        i += 1;
    }

    html
}
