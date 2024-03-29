use crate::parser::ast::{
    BodyAST, BoolAST, CallAST, DeclAssignAST, DeclarationAST, ExprAST, FnSignatureAST, NumberAST,
    StmtAST, TypeAST, VariableAST,
};

pub struct VarResolver {
    vars: Vec<DeclarationAST>,
}

pub struct FunctionResolver {
    signt: Vec<FnSignatureAST>,
}

impl VarResolver {
    ///creates a fresh resolver with no variables stored
    pub fn new() -> Self {
        Self { vars: Vec::new() }
    }

    ///creates a new Resolver with access to all variables with lower scope
    pub fn new_scoped(&self) -> Self {
        Self {
            vars: self.vars.clone(),
        }
    }

    ///add a declaration of a variable to the resolver
    pub fn add_decl(&mut self, decl: DeclarationAST) {
        self.vars.push(decl)
    }

    ///get the coresponding Declaration to a Variable
    pub fn resolve_variable(&self, var: &VariableAST) -> Option<DeclarationAST> {
        self.vars.iter().find(|dec| dec.name == var.name).cloned()
    }
}

impl FunctionResolver {
    ///creates a fresh resolver with no variables stored
    pub fn new() -> Self {
        Self { signt: Vec::new() }
    }

    pub fn new_from_body(body: &BodyAST) -> Self {
        let mut new = Self::new();
        for stmt in &body.stmts {
            match stmt {
                StmtAST::Function(func) => new.add_signature(func.fn_signt.clone()),
                _ => {}
            }
        }
        new
    }

    pub fn add_from_body(&mut self, body: &BodyAST) {
        for stmt in &body.stmts {
            match stmt {
                StmtAST::Function(func) => self.add_signature(func.fn_signt.clone()),
                _ => {}
            }
        }
    }

    ///creates a new Resolver with access to all variables with lower scope
    pub fn new_scoped(&self) -> Self {
        Self {
            signt: self.signt.clone(),
        }
    }

    ///add a fn signature to the resolver
    pub fn add_signature(&mut self, decl: FnSignatureAST) {
        self.signt.push(decl)
    }

    ///get the coresponding Declaration to a Variable
    pub fn resolve_call(&self, call: CallAST) -> Option<FnSignatureAST> {
        self.signt
            .iter()
            .find(|sign| sign.name == call.callee)
            .cloned()
    }
}
