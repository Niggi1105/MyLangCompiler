mod codegeneration;
mod cpu_impl;
mod parser;

use crate::parser::lexer::{self, Token};
use std::env;
use std::fs;

fn main() {
    let path = env::args().nth(1).expect("no path provided");
    let program = fs::read(path).expect("can't read program");
    let mut lexer = lexer::Lexer::new(program);
    loop {
        let tk = lexer.get_next_token();
        println!("{:?}", tk);
        if tk == Token::EOF {
            break;
        }
    }
}
