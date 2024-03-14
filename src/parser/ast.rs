use crate::parser::lexer::Token;

//{
//  Body
//}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BodyAST {
    pub stmts: Vec<StmtAST>,
}

/// code that has, returns or is a value
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExprAST {
    //var used in expr
    Variable(VariableAST),
    //calls as parts of expr, return value is importaint
    Call(CallAST),
    BinaryExpression(Box<BinaryExpressionAST>),
    Number(NumberAST),
    StringLiteral(StringLiteralAST),
    BoolLiteral(BoolAST),
}

/// code that only moves a value
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StmtAST {
    Assign(Box<AssignStmtAST>),
    //return value is ignored
    Call(CallAST),
    Declaration(DeclarationAST),
    DeclAssign(DeclAssignAST),
    //function definition
    Function(FunctionAST),
    Return(Box<ReturnStmtAST>),
    If(IfStmtAST),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TypeAST {
    U8,
    U16,
    U32,
    I8,
    I16,
    I32,
    Str,
    Char,
    Void,
    Bool,
    //Custom(String), //custom types are not yet supported
    Undefined,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BoolAST {
    pub value: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IfStmtAST {
    pub condition: ExprAST,
    pub body: BodyAST,
}

/// a hardcoded integer value
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NumberAST {
    pub num: i32,
}

/// used in expressions, will be resolved by code gen
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VariableAST {
    pub name: String,
}

//a = b
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssignStmtAST {
    pub var: VariableAST,
    pub value: ExprAST,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DeclAssignAST {
    pub decl: DeclarationAST,
    pub value: ExprAST,
}

//"some string"
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StringLiteralAST {
    pub str: String,
}

//fn foo(a: u8, b: u8) -> u8 {
//  Body
//}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FunctionAST {
    pub fn_signt: FnSignatureAST,
    pub body: BodyAST,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FnSignatureAST {
    pub name: String,
    pub args: Vec<DeclarationAST>,
    pub rt_type: TypeAST,
}

//let (mut) var: Type;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DeclarationAST {
    pub name: String,
    pub var_type: TypeAST,
    pub is_mut: bool,
}

//foo(8 , 2);
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CallAST {
    pub callee: String,
    pub args: Vec<ExprAST>,
    pub rt_value_ignored: bool,
}

//a + b
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BinaryExpressionAST {
    pub rhs: ExprAST,
    pub lhs: ExprAST,
    pub op: Token,
}

//return a
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ReturnStmtAST {
    pub expr: ExprAST,
}
