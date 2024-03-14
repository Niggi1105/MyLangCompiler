use crate::parser::ast::{BodyAST, CallAST, DeclarationAST, ExprAST, StmtAST, TypeAST};

use super::resolver::{FunctionResolver, VarResolver};

pub struct Typechecker {
    var_resolver: VarResolver,
    funct_resolver: FunctionResolver,
    body: BodyAST,
}

impl Typechecker {
    pub fn new(
        body: BodyAST,
        var_resolver: Option<VarResolver>,
        funct_resovler: Option<FunctionResolver>,
    ) -> Self {
        Self {
            var_resolver: var_resolver.unwrap_or(VarResolver::new()),
            funct_resolver: funct_resovler.unwrap_or(FunctionResolver::new()),
            body,
        }
    }

    fn check_and_resolve_call(&self, call: &CallAST) -> TypeAST {
        unimplemented!()
    }

    fn check_expression(&self, exrp: &ExprAST) {
        unimplemented!()
    }

    fn check_and_resolve_expression(&self, exrp: &ExprAST) -> TypeAST {
        unimplemented!()
    }

    fn check_return_stmt(&self, return_expr: &ExprAST) {
        unimplemented!()
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
                    )
                    .check_types();
                }

                StmtAST::Return(rtstmt) => self.check_return_stmt(&rtstmt.expr),
            }
        }
    }
}
