use crate::ast::{Expr};
use crate::tokenizer::Token;

pub struct Parser<'a> {
    index: usize,
    current_token: Option<&'a Token>,
    tokens: &'a [Token],
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Parser<'a> {
        Parser {
            index: 0,
            current_token: None,
            tokens,
        }
    }

    pub fn start_parsing(&mut self) -> Result<Expr, String> {
        self.next_token();
        self.parse()
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.parse_comparison()
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_add_sub()?;
        while let Some(token) = self.current_token {
            match token {
                Token::Greater | Token::Less | Token::LessEqual | Token::GreaterEqual | Token::EqualEqual => {

                    self.next_token();

                    let right = self.parse_add_sub()?;

                    left = Expr::BinaryOperator {
                        left: Box::new(left),
                        right: Box::new(right),
                        op: token.clone(),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }
    fn parse_add_sub(&mut self) -> Result<Expr, String> {
        
        let mut left = self.parse_mul_div()?;
        while let Some(token) = self.current_token {
            match token {
                Token::Plus | Token::Minus => {

                    self.next_token();

                    let right = self.parse_mul_div()?;

                    left = Expr::BinaryOperator {
                        left: Box::new(left),
                        right: Box::new(right),
                        op: token.clone(),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_mul_div(&mut self) -> Result<Expr, String> {
        
        let mut left = self.parse_function_call()?;
        while let Some(token) = self.current_token {
            match token {
                Token::Mul | Token::Div | Token::Mod => {

                    self.next_token();

                    let right = self.parse_function_call()?;

                    left = Expr::BinaryOperator {
                        left: Box::new(left),
                        right: Box::new(right),
                        op: token.clone(),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_function_call(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_leaf()?;
        while let Some(token) = self.current_token {
            match token {
                Token::LParen => {

                    self.next_token();


                    if let Some(Token::RParen) = self.current_token {
                        self.next_token();
                        return Ok(Expr::FunctionCall(Box::new(left), Vec::new()));
                    }

                    let mut args = Vec::new();
                    loop {
                        let arg = self.parse()?;
                        args.push(arg);


                        match self.current_token {
                            Some(Token::Comma) => {
                                self.next_token();

                            }
                            Some(Token::RParen) => {
                                self.next_token();

                                break;
                            }
                            _ => {
                                return Err("Expected ',' or ']' after an argument.".to_string());
                            }
                        }
                    }


                    left = Expr::FunctionCall(Box::new(left), args)
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_leaf(&mut self) -> Result<Expr, String> {
        
        match self.current_token {
            Some(Token::Number(value)) => {
                self.next_token();
                Ok(Expr::Number(*value))
            },
            Some(Token::LParen) => {
                self.next_token();
                let expr = self.parse()?;
                if let Some(Token::RParen) = self.current_token {
                    self.next_token();
                    Ok(expr)
                } else {
                    Err("Expected right paren!".to_string())
                }
            },
            Some(Token::Identifier(name)) => {
                self.next_token();
                return if let Some(Token::Equal) = self.current_token {
                    self.next_token();
                    let expr = self.parse()?;
                    Ok(Expr::Assign(name.clone(), Box::new(expr)))
                }  else {

                    Ok(Expr::Variable(name.clone()))
                }
            },
            Some(Token::Function) => {
                self.next_token();
                let mut args = Vec::new();
                while let Some(Token::Identifier(arg)) = self.current_token {
                    args.push(arg.clone());
                    self.next_token();
                    if let Some(Token::Comma) = self.current_token {
                        self.next_token();
                    } else {
                        break;
                    }
                }
                if let Some(Token::Arrow) = self.current_token {
                    self.next_token();
                    let expr = self.parse()?;
                    Ok(Expr::Function(args, Box::new(expr)))
                } else {
                    Err("kof".to_string())
                }
            },
            Some(Token::While) => {
                self.next_token();
                let condition = self.parse()?;
                let expr = self.parse()?;
                Ok(Expr::While(Box::new(condition), Box::new(expr)))
            },
            Some(Token::LBracket) => {
                let mut expressions = Vec::new();
                self.next_token();
                loop {
                    expressions.push(self.parse()?);
                    if let Some(Token::SemiColon) = self.current_token {
                        self.next_token(); // Move past the semicolon
                    } else {
                        break;
                    }
                }

                return if let Some(Token::RBracket) = self.current_token {
                    self.next_token();
                    Ok(Expr::CodeBlock(expressions))
                } else {
                    Err("Expected right bracket!".to_string())
                }
            },
            Some(Token::LBrace) => {
                let mut cases = Vec::new();
                let mut expressions = Vec::new();
                self.next_token();

                loop {
                    cases.push(self.parse()?);
                    if let Some(Token::Arrow) = self.current_token {
                        self.next_token();
                        expressions.push(self.parse()?);
                        if let Some(Token::Comma) = self.current_token {
                            self.next_token();
                        } else {
                            break
                        }
                    } else {
                        return Err("RJIRJ".to_string())
                    }
                }

                return if let Some(Token::RBrace) = self.current_token {
                    self.next_token();
                    Ok(Expr::Switch(cases, expressions))
                } else {
                    Err("Expected right brace!".to_string())
                }
            },
            _ => Err(format!("Unexpected token!, found {:?}", self.current_token)),
        }
    }

    fn next_token(&mut self) {
        if self.index < self.tokens.len() {
            self.current_token = Some(&self.tokens[self.index]);
            self.index += 1;
        } else {
            self.current_token = None;
        }
    }
}
