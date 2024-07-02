#[derive(Debug)]
struct Token {
    kind: TokenKind,
    span: Span,
}

#[derive(Debug)]
enum TokenKind {}

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
