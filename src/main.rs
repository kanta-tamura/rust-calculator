#[derive(Debug)]
enum TokenKind {
    Num(i64),   // numerical value
    Add,        // "+"
    Sub,        // "-"
    Mul,        // "*"
    Div,        // "/"
    LParen,     // "("
    RParen,     // ")"
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
    tokens.push(Token::new(TokenKind::Add));
    tokens.push(Token::new(TokenKind::Sub));
    tokens.push(Token::new(TokenKind::Num(4)));
    tokens.push(Token::new(TokenKind::Mul));
    tokens.push(Token::new(TokenKind::Div));
    tokens.push(Token::new(TokenKind::Num(6)));
    tokens.push(Token::new(TokenKind::LParen));
    tokens.push(Token::new(TokenKind::RParen));
    tokens.push(Token::new(TokenKind::EOF));

    let mut compare_num = Vec::new();
    let mut compere_symbol = Vec::new();
    for token in tokens {
        match token.kind {
            TokenKind::Num(val) => { compare_num.push(val) },
            TokenKind::Add    => { compere_symbol.push("+") },
            TokenKind::Sub    => { compere_symbol.push("-") },
            TokenKind::Mul    => { compere_symbol.push("*") },
            TokenKind::Div    => { compere_symbol.push("/") },
            TokenKind::LParen => { compere_symbol.push("(") },
            TokenKind::RParen => { compere_symbol.push(")") },
            TokenKind::EOF    => { compere_symbol.push(" ") },
        }
    }

    let result_num = vec![12, 4, 6];
    let result_symbol = vec!["+", "-", "*", "/", "(", ")", " "];
    assert_eq!(result_num, compare_num);
    assert_eq!(result_symbol, compere_symbol);
}

fn main() {

}
