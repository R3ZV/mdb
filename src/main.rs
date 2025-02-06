mod html;
mod lexer;
mod token;

use crate::html::create_html;
use crate::lexer::Lexer;

fn main() {
    let path = String::from("example/articles/blog1.md");
    let file_content = std::fs::read_to_string(&path).unwrap();

    let lexer = Lexer::new(file_content);
    let tokens = lexer.tokens();

    for token in tokens.iter() {
        println!("{:?}", token);
    }

    let page = create_html(&tokens);
    dbg!(page);
}
