mod codegeneration;
mod cpu_impl;
mod parser;

use crate::parser::lexer::Lexer;
use parser::parser::Parser;

use std::env;
use std::fs;

fn main() {
    //get path from env
    let path = env::args().nth(1).expect("no path provided");
    //read program file
    let program = fs::read(path).expect("can't read program");
    //build the lexer
    let lexer = Lexer::new(program);
    //build parser
    let mut pars = Parser::new(lexer);
    //run the parser
    let parsed = pars.parse();
    //run typechecks
}
