use crate::{
    ast::{
        AssignStmtAST, BinaryExpressionAST, BodyAST, BoolAST, CallAST, DeclAssignAST,
        DeclarationAST, ExprAST, FunctionAST, IfStmtAST, NumberAST, ReturnStmtAST, StmtAST,
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

    ///advances the lexer to the next token, stores the new token in current token and returns a
    ///clone of the new token
    pub fn get_next_token(&mut self) -> Token {
        self.cur_token = self.lexer.get_next_token();
        self.cur_token.clone()
    }

    ///returns the precedence for the current token, returns -1 if the token is not an operator,     
    fn operator_precedence(&self) -> i8 {
        match &self.cur_token {
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
        }
    }

    ///parses num and eats its token
    fn parse_number(&mut self, num: i32) -> NumberAST {
        let n = NumberAST { num };
        self.get_next_token();
        n
    }

    ///parses string literal and eats its token
    fn parse_string_literal(&mut self, lit: String) -> StringLiteralAST {
        let l = StringLiteralAST { str: lit };
        self.get_next_token();
        l
    }

    ///parses the Current token to a type and eats the current token
    ///only support for primitives so far
    fn parse_type(&mut self) -> TypeAST {
        let t = match &self.cur_token {
            Token::U8 => TypeAST::U8,
            Token::U16 => TypeAST::U16,
            Token::U32 => TypeAST::U32,
            Token::I8 => TypeAST::I8,
            Token::I16 => TypeAST::I16,
            Token::I32 => TypeAST::I32,
            Token::Str => TypeAST::Str,
            Token::Char => TypeAST::Char,
            Token::Bool => TypeAST::Bool,
            Token::Void => TypeAST::Void,
            //Token::Identifier(name) => TypeAST::Custom(name.to_string()),
            other => panic!(
                "Error in line: {:?}, unexpected token: {:?}, expected Type",
                self.lexer.current_line(),
                other
            ),
        };
        self.get_next_token();
        t
    }

    ///constructs a function call, where the return value is not ignored
    fn parse_call_expr(&mut self, name: String) -> CallAST {
        let mut args = Vec::new();
        while self.cur_token != Token::RightParen {
            //eat '(' or ','
            self.get_next_token();
            args.push(self.parse_expression());
        }
        //eat ')'
        self.get_next_token();
        CallAST {
            callee: name,
            args,
            rt_value_ignored: false,
        }
    }

    ///parses the right side of an assignment
    fn parse_assign(&mut self, name: String) -> AssignStmtAST {
        //eat '='
        self.get_next_token();
        let value = self.parse_expression();
        if self.cur_token != Token::SemiColon {
            panic!(
                "Error in line: {:?}, unexpected token: {:?}, expected ';'",
                self.lexer.current_line(),
                self.cur_token
            )
        }
        AssignStmtAST {
            var: VariableAST { name },
            value,
        }
    }

    ///for call expressions and variables inside expressions
    fn parse_identifier(&mut self, ident: String) -> ExprAST {
        //eats the identifier
        if self.get_next_token() != Token::LeftParen {
            //its a variable
            return ExprAST::Variable(VariableAST { name: ident });
        }
        ExprAST::Call(self.parse_call_expr(ident))
    }

    ///this is called when an identifier is found outside of expressions
    ///it is either a call with ignored return value or an assignment
    fn parse_ident_stmt(&mut self, ident: String) -> StmtAST {
        //eats the identifier
        let stmt = match self.get_next_token() {
            //...
            //foo(2,6)
            //...
            Token::LeftParen => {
                let mut call = self.parse_call_expr(ident);
                call.rt_value_ignored = true;
                StmtAST::Call(call)
            }
            Token::Assign => StmtAST::Assign(Box::new(self.parse_assign(ident))),
            other => panic!(
                "Error in line: {:?}, unexpected token: {:?}, expected '(' or '='",
                self.lexer.current_line(),
                other
            ),
        };
        //eat the semi colon
        self.get_next_token();
        stmt
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
        assert_eq!(self.cur_token, Token::Declaration);
        //eat "let"
        self.get_next_token();
        let is_mut = if self.cur_token == Token::Mut {
            //eat the 'mut' if it exists
            self.get_next_token();
            true
        } else {
            false
        };
        let name = match &self.cur_token {
            Token::Identifier(ident) => ident.to_string(),
            other => panic!(
                "Error in line: {:?}, unexpected token: {:?}, expected identifier",
                self.lexer.current_line(),
                other
            ),
        };
        //eat identifier
        let var_type = if self.get_next_token() == Token::Colon {
            //eat the ':'
            self.get_next_token();
            self.parse_type()
        } else {
            TypeAST::Undefined
        };
        //eat the type
        if self.cur_token == Token::SemiColon {
            //eat the ';'
            self.get_next_token();
            StmtAST::Declaration(DeclarationAST {
                name,
                var_type,
                is_mut,
            })
        } else if self.cur_token == Token::Assign {
            //eat the '='
            self.get_next_token();
            let val = self.parse_expression();

            if self.cur_token != Token::SemiColon {
                panic!(
                    "Error in line: {:?}, unexpected token: {:?}, expected ';'",
                    self.lexer.current_line(),
                    self.cur_token
                );
            }
            //eat the ';'
            self.get_next_token();
            //parse expr
            StmtAST::DeclAssign(DeclAssignAST {
                name,
                value: val,
                is_mut,
                var_type,
            })
        } else {
            panic!(
                "Error in line: {:?}, unexpected token: {:?}, expected ';' or '='",
                self.lexer.current_line(),
                self.cur_token
            );
        }
    }

    fn parse_argument(&mut self) -> (DeclarationAST, bool) {
        let is_mut = if self.cur_token == Token::Mut {
            //eat 'mut' if exists
            self.get_next_token();
            true
        } else {
            false
        };

        if let Token::Identifier(arg_name) = self.cur_token.clone() {
            //eat name
            if self.get_next_token() != Token::Colon {
                panic!(
                    "Error in line: {:?}, unexpected token: {:?}, expected ':'",
                    self.lexer.current_line(),
                    self.cur_token
                );
            }
            self.get_next_token();
            let arg_tp = self.parse_type();
            let is_last = if self.cur_token == Token::Comma {
                self.get_next_token();
                false
            } else if self.cur_token == Token::RightParen {
                true
            } else {
                panic!(
                    "Error in line: {:?}, unexpected token: {:?}, expected ',' or ')'",
                    self.lexer.current_line(),
                    self.cur_token
                )
            };
            (
                DeclarationAST {
                    name: arg_name,
                    var_type: arg_tp,
                    is_mut,
                },
                is_last,
            )
        } else {
            panic!(
                "Error in line: {:?}, unexpected token: {:?}, expected identifier",
                self.lexer.current_line(),
                self.cur_token
            )
        }
    }

    fn parse_function_def(&mut self) -> FunctionAST {
        //eat 'fn'
        if let Token::Identifier(name) = self.get_next_token() {
            //eat function name
            if let Token::LeftParen = self.get_next_token() {
                //eat '('
                let mut args = Vec::new();
                //only look for arguments if there are any
                if self.get_next_token() != Token::RightParen {
                    let mut is_last = false;
                    while !is_last {
                        let (arg, last) = self.parse_argument();
                        is_last = last;
                        args.push(arg);
                    }
                }
                //eat ')'
                let rt_type = if self.get_next_token() == Token::Arrow {
                    //eat '->'
                    self.get_next_token();
                    //parses and eats the type
                    self.parse_type()
                } else {
                    TypeAST::Void
                };

                //check for '{'
                if self.cur_token != Token::LeftBrace {
                    panic!(
                        "Error in line: {:?}, unexpected token: {:?}, expected '{{'",
                        self.lexer.current_line(),
                        self.cur_token
                    )
                }

                //eat '{'
                self.get_next_token();
                let body = self.parse_body();
                FunctionAST {
                    name,
                    args,
                    body,
                    rt_type,
                }
            } else {
                panic!(
                    "Error in line: {:?}, unexpected token: {:?}, expected '()'",
                    self.lexer.current_line(),
                    self.cur_token
                );
            }
        } else {
            panic!(
                "Error in line: {:?}, unexpected token: {:?}, expected identifier",
                self.lexer.current_line(),
                self.cur_token
            );
        }
    }

    fn parse_return_stmt(&mut self) -> ReturnStmtAST {
        //eat 'return'
        self.get_next_token();
        let rtstmt = ReturnStmtAST {
            expr: self.parse_expression(),
        };
        if self.cur_token == Token::SemiColon {
            //eat ';'
            self.get_next_token();
        } else {
            panic!(
                "Error in line: {:?}, unexpected token: {:?}, expected ';'",
                self.lexer.current_line(),
                self.cur_token
            )
        }
        rtstmt
    }

    fn parse_if_stmnt(&mut self) -> IfStmtAST {
        self.get_next_token();
        let condition = self.parse_expression();
        assert_eq!(self.get_next_token(), Token::LeftBrace);
        let body = self.parse_body();
        IfStmtAST { condition, body }
    }

    fn parse_bool_expr(&mut self) -> BoolAST {
        let bl = BoolAST {
            value: self.cur_token == Token::True,
        };
        self.get_next_token();
        bl
    }

    fn parse_body(&mut self) -> BodyAST {
        let mut stmts = Vec::new();
        loop {
            match &self.cur_token {
                Token::Declaration => stmts.push(self.parse_declaration()),
                Token::Definition => stmts.push(StmtAST::Function(self.parse_function_def())),
                Token::Return => stmts.push(StmtAST::Return(Box::new(self.parse_return_stmt()))),
                Token::Identifier(ident) => stmts.push(self.parse_ident_stmt(ident.to_string())),
                Token::Comment(com) => {
                    self.get_next_token();
                }
                Token::RightBrace => {
                    //eat '}'
                    self.get_next_token();
                    break;
                }
                Token::If => stmts.push(StmtAST::If(self.parse_if_stmnt())),
                other => panic!(
                    "Error in line: {:?}, unexpected token: {:?}, expected statement",
                    self.lexer.current_line(),
                    other
                ),
            };
        }
        BodyAST { stmts }
    }

    fn parse_primary_expression(&mut self) -> ExprAST {
        match &self.cur_token {
            Token::Identifier(ident) => self.parse_identifier(ident.to_string()),
            Token::StringLiteral(lit) => {
                ExprAST::StringLiteral(self.parse_string_literal(lit.to_string()))
            }
            Token::Number(num) => ExprAST::Number(self.parse_number(*num)),
            Token::True => ExprAST::BoolLiteral(self.parse_bool_expr()),
            Token::False => ExprAST::BoolLiteral(self.parse_bool_expr()),
            Token::LeftParen => self.parse_paren_expr(),
            other => panic!(
                "Error in line: {:?}, unexpected token: {:?}, expected Primary",
                self.lexer.current_line(),
                other
            ),
        }
    }

    fn parse_expression(&mut self) -> ExprAST {
        let lhs = self.parse_primary_expression();
        self.parse_binary_op_rhs(0, lhs)
    }

    fn parse_binary_op_rhs(&mut self, expr_prec: i8, mut lhs: ExprAST) -> ExprAST {
        loop {
            let tok_prec = self.operator_precedence();

            //the left side has higher precedence, resolve it first
            if tok_prec < expr_prec {
                return lhs;
            }

            let binop = self.cur_token.clone();
            //eat the binop
            self.get_next_token();

            //parse binary expr after binary operator
            let mut rhs = self.parse_primary_expression();

            //if prec of operator after rhs is higher than prec of operator between lhs and rhs,
            //give rhs as lhs to the pending op
            let next_prec = self.operator_precedence();
            if tok_prec < next_prec {
                rhs = self.parse_binary_op_rhs(tok_prec + 1, rhs);
            }

            lhs = ExprAST::BinaryExpression(Box::new(BinaryExpressionAST {
                rhs,
                lhs,
                op: binop,
            }));
        }
    }

    pub fn parse(&mut self) -> BodyAST {
        let mut program_elements = Vec::new();
        loop {
            match &self.cur_token {
                Token::Comment(_cmt) => {
                    self.get_next_token();
                }
                Token::Definition => {
                    program_elements.push(StmtAST::Function(self.parse_function_def()))
                }
                Token::EOF => break,
                other => panic!(
                    "Error in line: {:?}, unexpected token: {:?}, expected 'fn' or '//'",
                    self.lexer.current_line(),
                    other
                ),
            };
        }
        BodyAST {
            stmts: program_elements,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_functions() {
        let mprogram = "fn foo(mut a: u8, mut b: u8) -> u8 {
                            return a + b;  
                        }
                        fn main() -> void {
                            return foo(2,5);
                        }";
        let lexer = Lexer::new(mprogram.into());
        let mut parser = Parser::new(lexer);
        let body = parser.parse();
    }

    #[test]
    fn test_parse_assignment() {
        let mprogram = "fn main() -> void {
                            let a = 10;
                            let b = 20;
                            let c = a + b;
                        }";
        let lexer = Lexer::new(mprogram.into());
        let mut parser = Parser::new(lexer);
        let body = parser.parse();
    }

    #[test]
    fn test_parse_binary_expr() {
        let mprogram = "fn main() -> void {
                            let mut a: u8 = 10;
                            let b: u8 = 20;
                            let c: u16 = a + b * 3 - 1;
                        }";
        let lexer = Lexer::new(mprogram.into());
        let mut parser = Parser::new(lexer);
        let body = parser.parse();
    }

    #[test]
    fn test_parse_binary_expr_with_function_call() {
        let mprogram = "fn foo(a: u8, b: u8) -> u8 {
                            return a * a * b * b;
                        }
                        fn main() -> void {
                            let mut a: u8 = 10;
                            let b: u8 = 20;
                            let c: u16 = a + b * 3 + foo(3,2);
                        }";
        let lexer = Lexer::new(mprogram.into());
        let mut parser = Parser::new(lexer);
        let body = parser.parse();
    }
}
