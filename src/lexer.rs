use std::{cmp::Ordering, fmt, str::Chars};

use itertools::{multipeek, MultiPeek};

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_alphanumeric(c: char) -> bool {
    is_digit(c) || is_alpha(c)
}

fn is_whitespace(c: char) -> bool {
    c.is_ascii_whitespace()
}

#[derive(Clone, Debug)]
pub struct Scanner<'a> {
    source: &'a str,
    current: usize,
    iter: MultiPeek<Chars<'a>>,
    position: Position,
}

impl<'a> Scanner<'a> {
    pub fn init(source: &'a str) -> Self {
        Scanner {
            source,
            current: 0,
            iter: multipeek(source.chars()),
            position: Position::init(),
        }
    }

    fn advance(&mut self) -> Option<char> {
        match self.iter.next() {
            Some(ch) => {
                self.current += ch.len_utf8();
                if ch == '\n' {
                    self.position.next_line();
                } else {
                    self.position.next_column();
                }

                Some(ch)
            }
            None => None,
        }
    }

    fn is_next_char<P>(&mut self, predicate: &P) -> bool
    where
        P: Fn(char) -> bool,
    {
        self.iter.reset_peek();

        match self.iter.peek() {
            Some(&ch) => predicate(ch),
            None => false,
        }
    }

    fn is_next_two_chars<P1, P2>(&mut self, predicate1: &P1, predicate2: &P2) -> bool
    where
        P1: Fn(char) -> bool,
        P2: Fn(char) -> bool,
    {
        self.iter.reset_peek();

        match self.iter.peek() {
            Some(&ch1) => match self.iter.peek() {
                Some(&ch2) => predicate1(ch1) && predicate2(ch2),
                None => false,
            },
            None => false,
        }
    }

    fn chomp_if<P>(&mut self, predicate: &P) -> bool
    where
        P: Fn(char) -> bool,
    {
        if self.is_next_char(predicate) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn chomp_char(&mut self, expected: char) -> bool {
        self.chomp_if(&|ch| ch == expected)
    }

    fn chomp_while<P>(&mut self, predicate: &P)
    where
        P: Fn(char) -> bool,
    {
        while self.is_next_char(predicate) {
            self.advance();
        }
    }

    fn number(&mut self) -> TokenKind {
        self.chomp_while(&is_digit);

        if self.is_next_two_chars(&|c| c == '.', &is_digit) {
            self.advance();
            self.chomp_while(&is_digit);
        }

        TokenKind::Number
    }

    fn string(&mut self) -> TokenKind {
        self.chomp_while(&|c| c != '"' && c != '\n');

        if self.chomp_char('"') {
            TokenKind::String
        } else {
            TokenKind::UnterminatedStringError
        }
    }

    fn identifier(&mut self, start: usize) -> TokenKind {
        self.chomp_while(&is_alphanumeric);

        match &self.source[start..self.current] {
            "and" => TokenKind::And,
            "class" => TokenKind::Class,
            "else" => TokenKind::Else,
            "false" => TokenKind::False,
            "for" => TokenKind::For,
            "fun" => TokenKind::Fun,
            "if" => TokenKind::If,
            "nil" => TokenKind::Nil,
            "or" => TokenKind::Or,
            "print" => TokenKind::Print,
            "return" => TokenKind::Return,
            "super" => TokenKind::Super,
            "this" => TokenKind::This,
            "true" => TokenKind::True,
            "var" => TokenKind::Var,
            "while" => TokenKind::While,
            _ => TokenKind::Identifier,
        }
    }

    pub fn next_token(&mut self) -> Option<Token<'a>> {
        self.chomp_while(&is_whitespace);

        let start = self.current;
        let position = self.position;

        let next_char = self.advance()?;

        let token = match next_char {
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            ';' => TokenKind::Semicolon,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => {
                if self.chomp_char('/') {
                    self.chomp_while(&|c| c != '\n');
                    TokenKind::Comment
                } else {
                    TokenKind::Slash
                }
            }
            '!' => {
                if self.chomp_char('=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                }
            }
            '=' => {
                if self.chomp_char('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                }
            }
            '<' => {
                if self.chomp_char('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }
            }
            '>' => {
                if self.chomp_char('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            }
            '"' => self.string(),
            c if is_digit(c) => self.number(),
            c if is_alpha(c) => self.identifier(start),
            _ => TokenKind::UnexpectedCharacterError,
        };

        let lexeme = &self.source[start..self.current];

        Some(Token::new(token, lexeme, position))
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Token<'a> {
    kind: TokenKind,
    lexeme: &'a str,
    position: Position,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, lexeme: &'a str, position: Position) -> Self {
        Token {
            kind,
            lexeme,
            position,
        }
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:<14} '{}'", self.position, format!("{:?}", self.kind), self.lexeme)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TokenKind {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two characters.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // Others.
    Comment,
    UnterminatedStringError,
    UnexpectedCharacterError,
}

/// A position in the source file (line:column)
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn init() -> Self {
        Position { line: 1, column: 1 }
    }

    pub fn new(line: usize, column: usize) -> Self {
        Position { line, column }
    }

    pub fn next_column(&mut self) {
        self.column += 1;
    }

    pub fn next_line(&mut self) {
        self.line += 1;
        self.column = 1;
    }
}

impl Default for Position {
    fn default() -> Self {
        Position::init()
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        match self.line.cmp(&other.line) {
            Ordering::Equal => Some(self.column.cmp(&other.column)),
            ord => Some(ord),
        }
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Position) -> Ordering {
        match self.line.cmp(&other.line) {
            Ordering::Equal => self.column.cmp(&other.column),
            ord => ord,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:>3}:{:<3}", self.line, self.column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position() {
        let mut position = Position::init();

        assert_eq!(position, Position::new(1, 1));
        position.next_line();
        assert_eq!(position, Position::new(2, 1));
        position.next_column();
        position.next_column();
        assert_eq!(position, Position::new(2, 3));
    }

    #[test]
    fn number() {
        let source = "42 13.37";
        let mut scanner = Scanner::init(source);

        let expected_token_1 = Token::new(TokenKind::Number, "42", Position::new(1, 1));
        let expected_token_2 = Token::new(TokenKind::Number, "13.37", Position::new(1, 4));
        assert_eq!(scanner.next_token(), Some(expected_token_1));
        assert_eq!(scanner.next_token(), Some(expected_token_2));
        assert_eq!(scanner.next_token(), None);
    }

    #[test]
    fn string() {
        let source = r#"print "Hello, world!""#;
        let mut scanner = Scanner::init(source);

        let expected_token_1 = Token::new(TokenKind::Print, "print", Position::new(1, 1));
        let expected_token_2 =
            Token::new(TokenKind::String, r#""Hello, world!""#, Position::new(1, 7));
        assert_eq!(scanner.next_token(), Some(expected_token_1));
        assert_eq!(scanner.next_token(), Some(expected_token_2));
        assert_eq!(scanner.next_token(), None);
    }

    #[test]
    fn program() {
        let source = r#"
fun add() {
    var a = 42;
    var b = 13.37;
    return a + b; // The sum!
}"#;
        let mut scanner = Scanner::init(source);

        let expected_token = Token::new(TokenKind::Fun, "fun", Position::new(2, 1));
        assert_eq!(scanner.next_token(), Some(expected_token));
        let expected_token = Token::new(TokenKind::Identifier, "add", Position::new(2, 5));
        assert_eq!(scanner.next_token(), Some(expected_token));
        let expected_token = Token::new(TokenKind::LeftParen, "(", Position::new(2, 8));
        assert_eq!(scanner.next_token(), Some(expected_token));
        let expected_token = Token::new(TokenKind::RightParen, ")", Position::new(2, 9));
        assert_eq!(scanner.next_token(), Some(expected_token));
        let expected_token = Token::new(TokenKind::LeftBrace, "{", Position::new(2, 11));
        assert_eq!(scanner.next_token(), Some(expected_token));

        let expected_token = Token::new(TokenKind::Var, "var", Position::new(3, 5));
        assert_eq!(scanner.next_token(), Some(expected_token));
        let expected_token = Token::new(TokenKind::Identifier, "a", Position::new(3, 9));
        assert_eq!(scanner.next_token(), Some(expected_token));
        let expected_token = Token::new(TokenKind::Equal, "=", Position::new(3, 11));
        assert_eq!(scanner.next_token(), Some(expected_token));
        let expected_token = Token::new(TokenKind::Number, "42", Position::new(3, 13));
        assert_eq!(scanner.next_token(), Some(expected_token));
        let expected_token = Token::new(TokenKind::Semicolon, ";", Position::new(3, 15));
        assert_eq!(scanner.next_token(), Some(expected_token));

        let expected_token = Token::new(TokenKind::Var, "var", Position::new(4, 5));
        assert_eq!(scanner.next_token(), Some(expected_token));
        let expected_token = Token::new(TokenKind::Identifier, "b", Position::new(4, 9));
        assert_eq!(scanner.next_token(), Some(expected_token));
        let expected_token = Token::new(TokenKind::Equal, "=", Position::new(4, 11));
        assert_eq!(scanner.next_token(), Some(expected_token));
        let expected_token = Token::new(TokenKind::Number, "13.37", Position::new(4, 13));
        assert_eq!(scanner.next_token(), Some(expected_token));
        let expected_token = Token::new(TokenKind::Semicolon, ";", Position::new(4, 18));
        assert_eq!(scanner.next_token(), Some(expected_token));

        let expected_token = Token::new(TokenKind::Return, "return", Position::new(5, 5));
        assert_eq!(scanner.next_token(), Some(expected_token));
        let expected_token = Token::new(TokenKind::Identifier, "a", Position::new(5, 12));
        assert_eq!(scanner.next_token(), Some(expected_token));
        let expected_token = Token::new(TokenKind::Plus, "+", Position::new(5, 14));
        assert_eq!(scanner.next_token(), Some(expected_token));
        let expected_token = Token::new(TokenKind::Identifier, "b", Position::new(5, 16));
        assert_eq!(scanner.next_token(), Some(expected_token));
        let expected_token = Token::new(TokenKind::Semicolon, ";", Position::new(5, 17));
        assert_eq!(scanner.next_token(), Some(expected_token));
        let expected_token = Token::new(TokenKind::Comment, "// The sum!", Position::new(5, 19));
        assert_eq!(scanner.next_token(), Some(expected_token));

        let expected_token = Token::new(TokenKind::RightBrace, "}", Position::new(6, 1));
        assert_eq!(scanner.next_token(), Some(expected_token));
        assert_eq!(scanner.next_token(), None);
    }
}
