#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    Number,
    Plus,
    Minus,
    Division,
    Mulitplication,
    Invalid,
}

#[derive(Debug, Clone, Copy)]
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

#[allow(unused)]
impl TokenList {
    pub fn new(list: Vec<Token>) -> TokenList {
        TokenList{list, idx: 0}
    }

    pub fn next(&mut self) -> Option<Token> {
        if self.empty() {
            return None;
        }
        self.idx += 1;
        Some(self.list[self.idx - 1])
    }

    pub fn back_one(&mut self) {
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
                _ => tokens.push(Token::new(TokenType::Invalid, None)),
            };
            idx += 1;
        }
    }
    TokenList::new(tokens)
}

pub fn expression(tokens: &mut TokenList) -> f32 {
    let mut left = term(tokens);

    while !tokens.empty() {
        let operand = tokens.next().unwrap().token_type;

        match operand {
            TokenType::Plus => left += term(tokens),
            TokenType::Minus => left -= term(tokens),
            _ => break,
        }
    }
    left
}


fn term(tokens: &mut TokenList) -> f32 {
    let mut left = primary(tokens);

    while !tokens.empty() {
        let operand = tokens.next().unwrap().token_type;

        match operand {
            TokenType::Mulitplication => left *= primary(tokens),
            TokenType::Division => left /= primary(tokens),
            _ => {tokens.back_one(); break},
        }
    }
    left
}

fn primary(tokens: &mut TokenList) -> f32 {
    number(tokens)
}


fn number(tokens: &mut TokenList) -> f32 {
    let mut is_negative = false;

    let mut x = tokens.next().unwrap();

    if x.token_type == TokenType::Minus {
        is_negative = true;
        x = tokens.next().unwrap();
    }

    if x.token_type == TokenType::Number {
        let number = x.value.unwrap();
        if is_negative {
            return number * -1f32;
        }
        return number;
    }
    0f32
}
