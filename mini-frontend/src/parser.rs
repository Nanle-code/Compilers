use pest::Parser;
use pest_derive::Parser;

use crate::ast::*;
use crate::lexer::Token;

#[derive(Parser)]
#[grammar = "../grammar.pest"]
pub struct MiniParser;

pub fn parse(tokens: Vec<Token>) -> Result<Program, String> {
    let source = tokens_to_string(tokens);

    let pairs = MiniParser::parse(Rule::program, &source)
        .map_err(|e| format!("Parsing error: {}", e))?;

    build_ast(pairs)
}

fn tokens_to_string(tokens: Vec<Token>) -> String {
    tokens.into_iter().map(|t| format_token(t)).collect()
}                 

fn format_token(token: Token) -> String {
    match token {
        Token::Let => "let".to_string(),
        Token::Identifier(name) => name,
        Token::Number(num) => num.to_string(),
        Token::Equals => "=".to_string(),
        Token::Plus => "+".to_string(),
        Token::Semicolon => ";".to_string(),
        Token::LeftBrace => "(".to_string(),
        Token::RightBrace => ")".to_string(),
        Token::Whitespace => " ".to_string(),
        _ => "".into(),
    }

}

fn build_ast(pairs: pest::iterators::Pairs<Rule>) -> Result<Program, String> {
    println!("Pairs: {:#?}", pairs);

    Ok(Program { statements: vec![] })
}