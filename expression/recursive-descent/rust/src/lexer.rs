#[derive(Debug)]
pub enum TokenType {
    LeftParen,
    Minus,
    Number,
    Plus,
    Pow,
    RightParen,
    Slash,
    Star,
    Invalid,
}

#[derive(Debug)]
pub struct Token {
    kind: TokenType,
    spelling: String,
}

impl Token {
    pub fn new(kind: TokenType, spelling: String) -> Self {
        Token { kind, spelling }
    }
}

#[derive(Debug)]
pub struct Position {
    line: isize,
    column: isize,
}

impl Position {
    pub fn new(line: isize, column: isize) -> Self {
        Position { line, column }
    }
}

pub struct Lexer {
    source: String,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer { source }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        vec![]
    }
}
