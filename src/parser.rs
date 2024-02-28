use crate::{
    ast::{
        AssignStmtAST, CallExprAST, DeclAssignAST, DeclarationAST, ExprAST, NumberAST, StmtAST,
        StringLiteralAST, TypeAST, VariableAST,
    },
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
            //Token::Modulo => 19,
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

    fn parse_type(&mut self) -> TypeAST {
        match self.cur_token {
            Token::U8 => TypeAST::U8,
            Token::U16 => TypeAST::U16,
            Token::U32 => TypeAST::U32,
            Token::I8 => TypeAST::I8,
            Token::I16 => TypeAST::I16,
            Token::I32 => TypeAST::I32,
            Token::Str => TypeAST::Str,
            Token::Char => TypeAST::Char,
            Token::Identifier(name) => TypeAST::Custom(name),
            other => panic!("unexpected token: {:?}, expected Type", other),
        }
    }

    fn parse_call_expr(&mut self, name: String) -> CallExprAST {
        let mut args = Vec::new();
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
        return CallExprAST { callee: name, args };
    }

    fn parse_assign(&mut self, name: String) -> AssignStmtAST {
        //eat '='
        self.get_next_token();
        AssignStmtAST {
            var: VariableAST { name },
            value: self.parse_expression(),
        }
    }

    ///for call expressions and variables inside expressions
    fn parse_identifier(&mut self, ident: String) -> ExprAST {
        if self.get_next_token() != Token::LeftParen {
            //its a variable
            return ExprAST::Variable(VariableAST { name: ident });
        }
        ExprAST::Call(self.parse_call_expr(ident))
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

    fn parse_declaration(&mut self) -> StmtAST {
        //eat "let"
        self.get_next_token();
        let is_mut = self.cur_token == Token::Mut;
        let name = match &self.cur_token {
            Token::Identifier(ident) => ident.to_string(),
            other => panic!("unexpected token: {:?}, expected identifier", other),
        };
        let var_type = if self.get_next_token() == Token::Colon {
            self.parse_type()
        } else {
            TypeAST::Undefined
        };
        if self.get_next_token() == Token::SemiColon {
            StmtAST::Declaration(DeclarationAST {
                name,
                var_type,
                is_mut,
            })
        } else if self.cur_token == Token::Assign {
            //eat the '='
            self.get_next_token();
            //parse expr
            StmtAST::DeclAssign(DeclAssignAST {
                name,
                value: self.parse_expression(),
                is_mut,
                var_type,
            })
        } else {
            panic!(
                "unexpected token: {:?}, expected ';' or '='",
                self.cur_token
            );
        }
    }

    fn parse_primary_expression(&mut self) -> ExprAST {
        match &self.cur_token {
            Token::Identifier(ident) => self.parse_identifier(ident.to_string()),
            Token::StringLiteral(lit) => {
                ExprAST::StringLiteral(self.parse_string_literal(lit.to_string()))
            }
            Token::Number(num) => ExprAST::Number(self.parse_number(*num)),
            Token::LeftParen => self.parse_paren_expr(),
            other => panic!("unexpected token: {:?}, expected Primary", other),
        }
    }

    ///this is called when an identifier is found outside of expressions
    ///it is either a call with ignored return value or an assignment
    fn parse_ident_stmt(&mut self, ident: String) -> StmtAST {
        match self.get_next_token() {
            //...
            //foo(2,6)
            //...
            Token::LeftParen => StmtAST::Call(self.parse_call_expr(ident)),
            Token::Assign => StmtAST::Assign(Box::new(self.parse_assign(ident))),
            other => panic!("unexpected token: {:?}, expected '(' or '='", other),
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
        let parsed = parser.parse_primary_expression();
        panic!("{:?}", parsed)
    }
}
