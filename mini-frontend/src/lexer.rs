use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("let")]
    Let,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[regex("[0-9]+", |lex| lex.slice().parse::<u32>().unwrap())]
    Number(u32),

    #[token("=")]
    Equals,

    #[token("+")]
    Plus,

    #[token(";")]
    Semicolon,

    #[token("(")]
    LeftBrace,

    #[token(")")]
    RightBrace,

    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
}

pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut lexer = Token::lexer(input);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next() {
        match token {
            Ok(tok) => tokens.push(tok),
            Err(_) => return Err(format!("Unexpected token: {}", lexer.slice())),
        }
    }
println!("Tokens: {:#?}", tokens);
    Ok(tokens)


}