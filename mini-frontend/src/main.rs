mod lexer;
mod parser;
mod ast;
mod errors;

use lexer::lex;
use parser::parse;

fn main(){
    let input = std::env::args().nth(1).expect("Please provide a source code as argument");

    let tokens = lex(&input).expect("Lexing failed");
    let program = parse(tokens).expect("Parsing failed");


    println!("{:#?}", program);


}