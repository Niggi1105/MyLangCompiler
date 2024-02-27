use crate::lexer::Token;

pub struct BodyAST {
    pub exprs: Vec<ExprAST>,
}

pub enum ExprAST {
    Function(FunctionAST),
    Definition(Declaration),
    Variable(VariableAST),
    Call(CallExprAST),
    BinaryExpression(Box<BinaryExpressionAST>),
    Number(NumberAST),
    StringLiteral(StringLiteralAST),
    Assign(Box<AssignExprAST>),
    Return(Box<ReturnStmtAST>),
}

/// a hardcoded integer value
pub struct NumberAST {
    pub num: i32,
}

///used in expressions, will be resolved by code gen
pub struct VariableAST {
    pub name: String,
}

pub struct AssignExprAST {
    pub var: VariableAST,
    pub value: ExprAST,
}

pub struct StringLiteralAST {
    pub str: String,
}

pub struct FunctionAST {
    pub name: String,
    pub args: Vec<Declaration>,
    pub body: BodyAST,
    pub rt_type: String,
}

pub struct Declaration {
    pub name: String,
    pub var_type: String,
    pub is_const: bool,
}

pub struct CallExprAST {
    pub callee: String,
    pub args: Vec<ExprAST>,
}

pub struct BinaryExpressionAST {
    pub rhs: ExprAST,
    pub lhs: ExprAST,
}

pub struct ReturnStmtAST {
    pub expr: ExprAST,
}
