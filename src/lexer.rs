use crate::token::{Token, TokenType};

pub struct Lexer {
    input: String,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    pub fn tokens(&self) -> Vec<Token> {
        let mut tokens = Vec::new();

        for line in self.input.lines() {
            if line == "" {
                continue;
            }

            let token = Self::parse(line.to_string());

            tokens.push(token);
        }

        tokens
    }

    fn parse_header(line: String) -> Token {
        let mut it = line.chars().into_iter();
        let level: String = it.by_ref().take_while(|c| *c == '#').collect();
        let it = it.skip_while(|c| c.is_whitespace());

        let content = it.collect();
        Token::new(TokenType::header_level(&level), content)
    }

    fn parse(line: String) -> Token {
        if line.starts_with('#') {
            return Self::parse_header(line);
        }

        if line.starts_with('-') {
            return Token::new(TokenType::Ul, line.chars().skip(1).collect());
        }

        if line.chars().nth(0).unwrap().is_alphabetic() {
            return Token::new(TokenType::P, line);
        }

        if line.len() >= 2 && line.chars().nth(0).unwrap().is_alphanumeric() {
            return Token::new(TokenType::Ol, line.chars().skip(2).collect());
        }

        Token::new(TokenType::Eof, "".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_tokens() {
        let buff = r#"
            # Foo
            ## bar
            ### bar3
            #### bar4
            ##### bar5
            ####### foobar #bazz
            some text
            1.
            2. list
            - todo"#;

        let want = Vec::from([
            Token::new(TokenType::H1, "Foo".to_string()),
            Token::new(TokenType::H2, "bar".to_string()),
            Token::new(TokenType::H3, "bar3".to_string()),
            Token::new(TokenType::H4, "bar4".to_string()),
            Token::new(TokenType::H5, "bar5".to_string()),
            Token::new(TokenType::H6, "foobar #bazz".to_string()),
            Token::new(TokenType::P, "some text".to_string()),
            Token::new(TokenType::Ol, "".to_string()),
            Token::new(TokenType::Ol, " list".to_string()),
            Token::new(TokenType::Ul, " todo".to_string()),
        ]);

        let tokens = Lexer::new(buff.to_string()).tokens();
        assert_eq!(tokens.len(), want.len());
        for i in 0..tokens.len() {
            assert_eq!(tokens[i], want[i]);
        }
    }
}
