#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    RIGHT,
    LEFT,
    PLUS,
    MINUS,
    LBRACKET,
    RBRACKET,
    READ,
    WRITE,
}

#[derive(Debug)]
pub enum Ast {
    INCMP,
    DECMP,
    INCVAL,
    DECVAL,
    READ,
    WRITE,
    BRANCH {inner: Vec<Ast>}
}

#[derive(Copy, Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<char>,
}