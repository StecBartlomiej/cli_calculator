use anyhow::{Result, bail, ensure, Context};

/*
#[derive(Debug)]
pub enum Error {
    OutOfBound,
    ParsingError,

}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::OutOfBound => write!(f, "Array out of bounds"),
            Error::ParsingError=> write!(f, "Parrsing error"),
        }
    }
}
*/

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum TokenType {
    #[default]
    Invalid,
    Number,
    Plus,
    Minus,
    Division,
    Mulitplication,
    OpenParentheses,
    EndParentheses,
}


#[derive(Debug, Clone, Copy, Default)]
pub struct Token {
    token_type: TokenType,
    value: Option<f32>,
}

impl Token {
    pub fn new(token_type: TokenType, value: Option<f32>) -> Token {
        Token{token_type, value}
    }
}

#[derive(Debug)]
pub struct TokenList {
    list: Vec<Token>,
    idx: usize

}

// Change returns to Result<> with anyhow crate
#[allow(unused)]
impl TokenList {
    pub fn new(list: Vec<Token>) -> TokenList {
        TokenList{list, idx: 0}
    }

    pub fn next(&mut self) {
        self.idx += 1;
    }

    pub fn peek(&self) -> Option<Token> {
        self.list.get(self.idx).cloned()
    }

    pub fn back(&mut self) {
        assert!(self.idx > 0);
        self.idx -= 1;
    }

    pub fn push(&mut self, token: Token) {
        self.list.push(token);
    }

    pub fn empty(&self) -> bool {
        self.idx >= self.list.len()
    }

    pub fn clear(&mut self) {
        self.list.clear();
        self.idx = 0;
    }

}


pub fn get_tokens(vec_char: &Vec<char>) -> TokenList {
    let mut tokens: Vec<Token> = Vec::new();

    let mut idx = 0;

    while idx < vec_char.len() {
        let c = vec_char[idx];

        if c.is_digit(10) {
            let value = vec_char.iter()
                .skip(idx)
                .take_while(|x| x.is_digit(10) || **x == '.')
                .collect::<String>();

            idx += value.len();
            let f32_value = value.parse::<f32>().unwrap();

            tokens.push(Token::new(TokenType::Number, Some(f32_value)));
        }

        else {
            match c {
                '+' => tokens.push(Token::new(TokenType::Plus, None)),
                '-' => tokens.push(Token::new(TokenType::Minus, None)),
                '*' => tokens.push(Token::new(TokenType::Mulitplication, None)),
                '/' => tokens.push(Token::new(TokenType::Division, None)),
                '(' => tokens.push(Token::new(TokenType::OpenParentheses, None)),
                ')' => tokens.push(Token::new(TokenType::EndParentheses, None)),
                _ => tokens.push(Token::new(TokenType::Invalid, None)),
            };
            idx += 1;
        }
    }
    TokenList::new(tokens)
}

pub fn expression(tokens: &mut TokenList) -> Result<f32> {
    let mut left = term(tokens)?;

    while !tokens.empty() {
        let operand = tokens.peek().context("Expected + or -")?;
        tokens.next();

        match operand.token_type {
            TokenType::Plus => left += term(tokens)?,
            TokenType::Minus => left -= term(tokens)?,
            _ => {tokens.back(); break;},
        }
    }
    Ok(left)
}


fn term(tokens: &mut TokenList) -> Result<f32> {
    let mut left = primary(tokens)?;

    while !tokens.empty() {
        let operand = tokens.peek().context("Expected * or /")?;
        tokens.next();

        match operand.token_type {
            TokenType::Mulitplication => left *= primary(tokens)?,
            TokenType::Division => left /= primary(tokens)?,
            _ => {tokens.back(); break;},
        }
        tokens.next();
    }
    Ok(left)
}

fn primary(tokens: &mut TokenList) -> Result<f32> {
    let operand = tokens.peek().context("Expected number or (")?;

    if operand.token_type == TokenType::OpenParentheses {
        tokens.next();
        let result = expression(tokens)?;

        ensure!(tokens.peek().unwrap_or(Token::default()).token_type == TokenType::EndParentheses, "Lack of closing Parentheses");

        tokens.next();
        return Ok(result);
    }

    number(tokens)
}


fn number(tokens: &mut TokenList) -> Result<f32> {
    let mut is_negative = false;

    let mut x = tokens.peek().context("Out of bounds, expected number")?;

    if x.token_type == TokenType::Minus {
        is_negative = true;
        tokens.next();
        x = tokens.peek().context("No number after - symbol")?;
    }

    tokens.next();

    if x.token_type == TokenType::Number {
        let number = x.value.unwrap();

        match is_negative {
            true => return Ok(number * -1f32),
            false => return Ok(number),
        }
    }
    bail!("Expected number");
}

