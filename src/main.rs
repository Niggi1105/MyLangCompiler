mod codegeneration;
mod parser;

use crate::parser::ast::TypeAST;
use crate::parser::lexer::Lexer;
use codegeneration::typechecks::Typechecker;
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
    println!("{:?}", parsed);
    //run typechecks
    Typechecker::new(parsed, None, None, TypeAST::Void).check_types();
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_type_checks(){
        
        let path = "./MyLangData/test_lexer.myla";
        //read program file
        let program = fs::read(path).expect("can't read program");
        //build the lexer
        let lexer = Lexer::new(program);
        //build parser
        let mut pars = Parser::new(lexer);
        //run the parser
        let parsed = pars.parse();
        println!("{:?}", parsed);
        //run typechecks
        Typechecker::new(parsed, None, None, TypeAST::Void).check_types();
    }
}