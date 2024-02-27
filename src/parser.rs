use core::panic;

use crate::{
    ast::{CallExprAST, ExprAST, NumberAST, StringLiteralAST, VariableAST},
    lexer::{Lexer, Token},
};

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        Self {
            cur_token: lexer.get_next_token(),
            lexer,
        }
    }

    pub fn get_next_token(&mut self) -> Token {
        self.cur_token = self.lexer.get_next_token();
        self.cur_token.clone()
    }

    fn operator_precedence(&self) -> Option<i8> {
        Some(match &self.cur_token {
            Token::PlusAssign => 0,
            Token::MinusAssign => 0,
            Token::MultAssign => 0,
            Token::DivideAssign => 0,
            Token::AndIntAssign => 0,
            Token::OrIntAssign => 0,
            Token::XorIntAssign => 0,
            Token::OrBool => 1,
            Token::AndBool => 3,
            Token::XorBool => 5,
            Token::Equal => 7,
            Token::GreaterThan => 7,
            Token::LessThan => 7,
            Token::OrInt => 9,
            Token::XorInt => 11,
            Token::AndInt => 13,
            Token::LeftShift => 15,
            Token::RightShift => 15,
            Token::Plus => 17,
            Token::Minus => 17,
            Token::Mult => 19,
            Token::Divide => 19,
            Token::Modulo => 19,
            Token::Not => 21,
            _other => -1,
        })
    }

    fn parse_number(&mut self, num: i32) -> NumberAST {
        NumberAST { num }
    }

    fn parse_string_literal(&mut self, lit: String) -> StringLiteralAST {
        StringLiteralAST { str: lit }
    }

    ///call expressions and variables
    fn parse_identifier(&mut self, ident: String) -> ExprAST {
        if self.get_next_token() != Token::LeftParen {
            //its a variable
            return ExprAST::Variable(VariableAST { name: ident });
        }

        let mut args = Vec::new();
        //its a call expression
        while self.cur_token != Token::RightParen {
            self.get_next_token();
            println!("{:?}", self.cur_token);
            args.push(self.parse_expression());
            //eat comma
            match self.get_next_token() {
                Token::Comma => {}
                Token::RightParen => break,
                other => panic!("unexpected token: {:?}, expected ',' or ')'", other),
            }
        }
        return ExprAST::Call(CallExprAST {
            callee: ident,
            args,
        });
    }

    ///returns the parsed expression from within the parens
    fn parse_paren_expr(&mut self) -> ExprAST {
        assert_eq!(self.cur_token, Token::LeftParen);
        // eat '('
        self.get_next_token();
        //parse whatever is in the parens
        let expr = self.parse_expression();
        //eat ')'
        self.get_next_token();
        expr
    }

    fn parse_declaration(&mut self) -> ExprAST {}

    fn parse_primary(&mut self) -> ExprAST {
        match self.cur_token {
            Token::Identifier(ident) => self.parse_identifier(ident),
            Token::StringLiteral(lit) => ExprAST::StringLiteral(self.parse_string_literal(lit)),
            Token::Number(num) => ExprAST::Number(self.parse_number(num)),
            Token::LeftParen => self.parse_paren_expr(),
        }
    }

    fn parse_expression(&mut self) -> ExprAST {
        ExprAST::Number(NumberAST { num: 0 })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_identifier() {
        let program = "foo(a, b)";
        let lexer = Lexer::new(program.into());
        let mut parser = Parser::new(lexer);
        parser.parse_primary();
    }
}
