

#[cfg(test)]
mod tests {
    use super::*;

    fn compare_tokens(vec1: Vec<Token>, vec2: Vec<Token>) -> bool {
        vec1.len() == vec2.len() && vec1.iter().zip(vec2.iter()).all(|(a, b)| a == b)
    }

    #[test]
    fn test_single_operators() {
        let mut tokenizer = Tokenizer::new("+ - * / %");
        let tokens = tokenizer.tokenize().unwrap();
        let expected_tokens = vec![
            Token::Plus, Token::Minus, Token::Mul, Token::Div, Token::Mod
        ];
        assert!(compare_tokens(tokens, expected_tokens));
    }

    #[test]
    fn test_numbers() {
        let mut tokenizer = Tokenizer::new("123 456.789");
        let tokens = tokenizer.tokenize().unwrap();
        let expected_tokens = vec![
            Token::Number(123.0), Token::Number(456.789)
        ];
        assert!(compare_tokens(tokens, expected_tokens));
    }

    #[test]
    fn test_identifiers() {
        let mut tokenizer = Tokenizer::new("abc xyz123 a1b2c3");
        let tokens = tokenizer.tokenize().unwrap();
        let expected_tokens = vec![
            Token::Identifier("abc".to_string()),
            Token::Identifier("xyz123".to_string()),
            Token::Identifier("a1b2c3".to_string())
        ];
        assert!(compare_tokens(tokens, expected_tokens));
    }

    #[test]
    fn test_mixed_input() {
        let mut tokenizer = Tokenizer::new("123 + abc - 45.67 * xyz / %");
        let tokens = tokenizer.tokenize().unwrap();
        let expected_tokens = vec![
            Token::Number(123.0),
            Token::Plus,
            Token::Identifier("abc".to_string()),
            Token::Minus,
            Token::Number(45.67),
            Token::Mul,
            Token::Identifier("xyz".to_string()),
            Token::Div,
            Token::Mod
        ];
        assert!(compare_tokens(tokens, expected_tokens));
    }

    #[test]
    fn test_number_with_two_dots() {
        let mut tokenizer = Tokenizer::new("12.34.56");
        let result = tokenizer.tokenize();
        assert!(result.is_err());
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Plus,
    Minus,
    Mul,
    Div,
    Mod,

    LParen,
    RParen,

    LBrace,
    RBrace,

    LBracket,
    RBracket,

    Equal,
    EqualEqual, // Token for '=='

    Greater,
    GreaterEqual, // Token for '>='

    Less,
    LessEqual, // Token for '<='

    SemiColon,
    Comma,
    Ampersand,

    Arrow,

    Number(f64),
    Identifier(String),
    Function,
    While,

    Invalid,
}

pub struct Tokenizer {
    index: usize,
    current_char: char,
    code: String
}

impl Tokenizer{
    pub fn new(code: &str) -> Tokenizer {
        Tokenizer {
            index: 0,
            current_char: '\0',
            code: code.to_string()
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut vector = Vec::new();

        self.next_char();

        while self.current_char != '\0' {
            self.skip_whitespace();

            match self.current_char {
                '+' => vector.push(Token::Plus),
                '-' => {
                    if self.peek_char() == '>' {
                        vector.push(Token::Arrow);
                        self.next_char();
                    } else {
                        vector.push(Token::Minus);
                    }
                },
                '*' => vector.push(Token::Mul),
                '/' => vector.push(Token::Div),
                '%' => vector.push(Token::Mod),
                '(' => vector.push(Token::LParen),
                ')' => vector.push(Token::RParen),
                '[' => vector.push(Token::LBracket),
                ']' => vector.push(Token::RBracket),
                '{' => vector.push(Token::LBrace),
                '}' => vector.push(Token::RBrace),
                ';' => vector.push(Token::SemiColon),
                '=' => {
                    if self.peek_char() == '=' {
                        vector.push(Token::EqualEqual);
                        self.next_char();
                    } else {
                        vector.push(Token::Equal);
                    }
                },
                ',' => vector.push(Token::Comma),
                '&' => vector.push(Token::Ampersand),
                '>' => {
                    if self.peek_char() == '=' {
                        vector.push(Token::GreaterEqual);
                        self.next_char();
                    } else {
                        vector.push(Token::Greater);
                    }
                },
                '<' => {
                    if self.peek_char() == '=' {
                        vector.push(Token::LessEqual);
                        self.next_char();
                    } else {
                        vector.push(Token::Less);
                    }
                },
                other => {
                    if other.is_digit(10) || other == '.' {
                        let mut number_str = String::new();
                        let mut dot = false;

                        while self.current_char.is_digit(10) || self.current_char == '.' {
                            if self.current_char == '.' {
                                if dot {
                                    return Err("Found second dot in a number.".to_string());
                                } else {
                                    dot = true;
                                }
                            }
                            number_str.push(self.current_char);
                            self.next_char();
                        }

                        let number = match number_str.parse() {
                            Ok(r) => r,
                            Err(e) => return Err(format!("{}", e))
                        };
                        vector.push(Token::Number(number));
                        continue;
                    } else if other.is_alphabetic() || other == '_' {
                        let mut string = String::new();

                        while self.current_char.is_alphabetic() || self.current_char.is_digit(10) || self.current_char == '_' {
                            string.push(self.current_char);
                            self.next_char();
                        }
                        match string.as_str() {
                            "fn" => {
                                vector.push(Token::Function);
                                continue;
                            },
                            "while" => {
                                vector.push(Token::While);
                                continue;
                            },
                            _ => {
                                vector.push(Token::Identifier(string));
                                continue;
                            }
                        }
                    }
                }
            }

            self.next_char();
        }
        Ok(vector)
    }

    pub fn skip_whitespace(&mut self) {
        while self.current_char.is_whitespace() {
            self.next_char();
        }
    }

    pub fn next_char(&mut self) {
         if self.index < self.code.len() {
            self.current_char = self.code.chars().collect::<Vec<_>>()[self.index];
            self.index += 1;
        } else {
            self.current_char = '\0';
        }
    }

    pub fn peek_char(&mut self) -> char {
        if self.index < self.code.len() {
            return self.code.chars().collect::<Vec<_>>()[self.index];
        }
        '\0'
    }
}