#[derive(Debug)]
enum TokenKind {
    Num(i64),   // numerical value
    EOF,        // terminal symbol
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,   // type of token
}

impl Token {
    fn new(kind: TokenKind) -> Self {
        Self {
            kind,
        }
    }
}

#[test]
fn test_token() {
    let mut tokens = Vec::new();
    tokens.push(Token::new(TokenKind::Num(12)));
    tokens.push(Token::new(TokenKind::EOF));

    let mut compere = Vec::new();
    for token in tokens {
        match token.kind {
            TokenKind::Num(val) => { compere.push(val) }
            TokenKind::EOF => {}
        }
    }

    let result = vec![12];
    assert_eq!(result, compere);
}

fn main() {

}
