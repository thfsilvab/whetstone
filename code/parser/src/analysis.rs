/// Used to translate characters from the code file to known symbols, and reduce the scope and code
/// necessary for the next steps in the lexical analysis
enum Symbol {
    Unknown,
    Whitespace,
}

impl From<&char> for Symbol {
    fn from(value: &char) -> Self {
        match value {
            '\u{0009}' | '\u{000B}' | '\u{000C}' => Symbol::Whitespace,
            _ => Symbol::Unknown,
        }
    }
}

// Used as a unit in the stream of tokens that's transmited to the parser
enum Token {}

/// Used to perform lexical analysis and generate tokens for an input code
struct Lexer<'a> {
    code: &'a str,
}

impl<'a> Lexer<'a> {
    fn new(code: &'a str) -> Self {
        Self { code }
    }

    fn transition(&mut self, symbol: &Symbol) -> Option<Token> {
        todo!()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
