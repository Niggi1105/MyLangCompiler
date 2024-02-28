//{
//  Body
//}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BodyAST {
    pub exprs: Vec<StmtAST>,
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
    Custom(String), //name
    Undefined,
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
    pub name: String,
    pub value: ExprAST,
    pub is_mut: bool,
    pub var_type: TypeAST,
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
    pub name: String,
    pub args: Vec<DeclarationAST>,
    pub body: BodyAST,
    pub rt_type: String,
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
}

//a + b
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BinaryExpressionAST {
    pub rhs: ExprAST,
    pub lhs: ExprAST,
}

//return a
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ReturnStmtAST {
    pub expr: ExprAST,
}
