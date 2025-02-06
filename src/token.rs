#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum TokenType {
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
    Illegal,
    Ol,
    Ul,
    Eof,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    ttype: TokenType,
    content: String,
}

impl Token {
    pub fn new(ttype: TokenType, content: String) -> Self {
        Self { ttype, content }
    }

    pub fn ttype(&self) -> TokenType {
        self.ttype
    }

    pub fn content(&self) -> String {
        self.content.clone()
    }
}

impl TokenType {
    pub fn header_level(level: &str) -> TokenType {
        match level {
            "#" => TokenType::H1,
            "##" => TokenType::H2,
            "###" => TokenType::H3,
            "####" => TokenType::H4,
            "#####" => TokenType::H5,
            _ => TokenType::H6,
        }
    }
}
