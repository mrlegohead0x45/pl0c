#[derive(Debug)]
pub(crate) struct Token {
    kind: TokenKind,
    span: Span,
}

#[derive(Debug)]
enum TokenKind {
    Integer(i64),

    Plus,
    Minus,
    Asterisk,
    Slash,

    LParen,
    RParen,

    Eq,
    ColonEq,
    EqEq,
    NotEq,

    LAngle,
    RAngle,
    LAngleEq,
    RAngleEq,

    Comma,
    Semicolon,
}

#[derive(Debug)]
struct Span {
    start: Position,
    end: Position,
}

#[derive(Debug)]
struct Position {
    line: usize,
    column: usize,
}
