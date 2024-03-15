use crate::parser::{
    ast::{BinaryExpressionAST, BodyAST, CallAST, DeclarationAST, ExprAST, StmtAST, TypeAST},
    lexer::Token,
};

use super::resolver::{FunctionResolver, VarResolver};

pub struct Typechecker {
    var_resolver: VarResolver,
    funct_resolver: FunctionResolver,
    body: BodyAST,
    expected_rt_tp: TypeAST,
}

//TODO: add type inference for declarations and declarations with assignments if type is undefined
impl Typechecker {
    pub fn new(
        body: BodyAST,
        var_resolver: Option<VarResolver>,
        funct_resovler: Option<FunctionResolver>,
        expected_rt_tp: TypeAST,
    ) -> Self {
        Self {
            var_resolver: var_resolver.unwrap_or(VarResolver::new()),
            funct_resolver: funct_resovler.unwrap_or(FunctionResolver::new_from_body(&body)),
            body,
            expected_rt_tp,
        }
    }

    fn check_and_resolve_call(&self, call: &CallAST) -> TypeAST {
        let signt = self
            .funct_resolver
            .resolve_call(call.clone())
            .expect("function not found in scope");
        assert_eq!(signt.args.len(), call.args.len());
        //compare types between given and declared args
        signt
            .args
            .iter()
            .zip(call.args.iter())
            .for_each(|(signt_arg, call_arg)| {
                assert_eq!(
                    signt_arg.var_type,
                    self.check_and_resolve_expression(call_arg)
                )
            });
        if call.rt_value_ignored {
            TypeAST::Void
        } else {
            signt.rt_type
        }
    }

    fn check_iteger_bin_expr(&self, expr: &BinaryExpressionAST) {
        match self.check_and_resolve_expression(&expr.lhs) {
            TypeAST::I8 => assert_eq!(
                self.check_and_resolve_expression(&expr.rhs),
                TypeAST::I8,
                "Incompatible Types, type should be i8"
            ),
            TypeAST::I16 => assert_eq!(
                self.check_and_resolve_expression(&expr.rhs),
                TypeAST::I16,
                "Incompatible Types, type should be i16"
            ),
            TypeAST::I32 => assert_eq!(
                self.check_and_resolve_expression(&expr.rhs),
                TypeAST::I32,
                "Incompatible Types, type should be i32"
            ),
            TypeAST::U8 => assert_eq!(
                self.check_and_resolve_expression(&expr.rhs),
                TypeAST::U8,
                "Incompatible Types, type should be u8"
            ),
            TypeAST::U16 => assert_eq!(
                self.check_and_resolve_expression(&expr.rhs),
                TypeAST::U16,
                "Incompatible Types, type should be u16"
            ),
            TypeAST::U32 => assert_eq!(
                self.check_and_resolve_expression(&expr.rhs),
                TypeAST::U32,
                "Incompatible Types, type should be u32"
            ),
            other => panic!(
                "binary opperator XORINT is not supported for type: {}",
                other
            ),
        };
    }

    fn check_and_resolve_binary_expression(&self, expr: &BinaryExpressionAST) -> TypeAST {
        assert_eq!(
            self.check_and_resolve_expression(&expr.rhs),
            self.check_and_resolve_expression(&expr.lhs),
            "incompatible types lhs and rhs"
        );
        match &expr.op {
            Token::XorInt
            | Token::OrInt
            | Token::AndInt
            | Token::Minus
            | Token::Mult
            | Token::Divide => self.check_iteger_bin_expr(expr),
            Token::XorBool | Token::OrBool | Token::AndBool | Token::Not => {
                assert_eq!(
                    self.check_and_resolve_expression(&expr.rhs),
                    TypeAST::Bool,
                    "bool operators can only be applied to booleans"
                );
                assert_eq!(
                    self.check_and_resolve_expression(&expr.lhs),
                    TypeAST::Bool,
                    "bool operators can only be applied to booleans"
                );
            }
            Token::Plus => {
                todo!("add implementation for strings and integers")
            }
            other => panic!("not a vaild opperator: {}", other),
        }
        TypeAST::Undefined
    }

    fn check_and_resolve_expression(&self, expr: &ExprAST) -> TypeAST {
        match expr {
            //in case of variable resolve variable and return the type
            ExprAST::Variable(var) => {
                self.var_resolver
                    .resolve_variable(var)
                    .expect("use of undeclared variable")
                    .var_type
            }
            //in case of call resolve call and return type
            ExprAST::Call(call) => {
                self.funct_resolver
                    .resolve_call(call.clone())
                    .expect("call of undefined function")
                    .rt_type
            }
            ExprAST::Number(_num) => {
                //only lower 8 bits are used
                TypeAST::I32
            }
            ExprAST::BoolLiteral(_) => TypeAST::Bool,
            ExprAST::StringLiteral(_) => TypeAST::Str,
            ExprAST::BinaryExpression(bin_expr) => {
                self.check_and_resolve_binary_expression(&bin_expr)
            }
        }
    }

    fn check_return_stmt(&self, return_expr: &ExprAST) {
        assert_eq!(
            self.expected_rt_tp,
            self.check_and_resolve_expression(return_expr)
        );
    }

    pub fn check_types(&mut self) {
        for stmt in &self.body.stmts {
            match stmt {
                StmtAST::Declaration(decl) => self.var_resolver.add_decl(decl.clone()),
                StmtAST::DeclAssign(declassg) => {
                    self.var_resolver.add_decl(declassg.decl.clone());
                    assert_eq!(
                        declassg.decl.var_type,
                        self.check_and_resolve_expression(&declassg.value),
                        "invalid type"
                    );
                }
                StmtAST::Assign(ass) => {
                    assert_eq!(
                        self.var_resolver
                            .resolve_variable(&ass.var)
                            .expect("variable not found")
                            .var_type,
                        self.check_and_resolve_expression(&ass.value)
                    )
                }

                StmtAST::Call(cll) => {
                    assert_eq!(self.check_and_resolve_call(&cll), TypeAST::Void);
                }

                StmtAST::Function(func) => {
                    self.funct_resolver.add_signature(func.fn_signt.clone());
                    Self::new(
                        self.body.clone(),
                        Some(self.var_resolver.new_scoped()),
                        Some(self.funct_resolver.new_scoped()),
                        func.fn_signt.rt_type.clone(),
                    )
                    .check_types();
                }

                StmtAST::If(if_st) => {
                    assert_eq!(
                        self.check_and_resolve_expression(&if_st.condition),
                        TypeAST::Bool
                    );
                    Self::new(
                        self.body.clone(),
                        Some(self.var_resolver.new_scoped()),
                        Some(self.funct_resolver.new_scoped()),
                        TypeAST::Void,
                    )
                    .check_types();
                }

                StmtAST::Return(rtstmt) => self.check_return_stmt(&rtstmt.expr),
            }
        }
    }
}
