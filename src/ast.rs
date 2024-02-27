use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum ExprAST {
    NumberExpr(NumberExprAST),
    VariableExpr(VariableExprAST),
    BinaryExpr(Box<BinaryExprAST>),
    CallExpr(CallExprAST),
    Prototype(PrototypeAST),
    Function(Box<FunctionAST>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NumberExprAST {
    value: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableExprAST {
    name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExprAST {
    lhs: ExprAST,
    rhs: ExprAST,
    op: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExprAST {
    callee: String,
    args: Vec<ExprAST>,
}

//barebone of a function
#[derive(Debug, Clone, PartialEq)]
pub struct PrototypeAST {
    name: String,
    args: Vec<String>, //contains names of the arguments, type is always f64
}

//a function body
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionAST {
    proto: PrototypeAST,
    body: ExprAST,
}

impl NumberExprAST {
    pub fn new(n: f64) -> Self {
        Self { value: n }
    }
}
impl VariableExprAST {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl BinaryExprAST {
    pub fn new(rhs: ExprAST, lhs: ExprAST, op: Token) -> Self {
        Self { rhs, lhs, op }
    }

    pub fn rhs(&self) -> ExprAST {
        self.rhs.clone()
    }

    pub fn lhs(&self) -> ExprAST {
        self.lhs.clone()
    }

    pub fn op(&self) -> Token {
        self.op.clone()
    }
}

impl CallExprAST {
    pub fn new(callee: String, args: Vec<ExprAST>) -> Self {
        Self { callee, args }
    }
}

impl PrototypeAST {
    pub fn new(name: String, args: Vec<String>) -> Self {
        Self { name, args }
    }
}

impl FunctionAST {
    pub fn new(proto: PrototypeAST, body: ExprAST) -> Self {
        Self { proto, body }
    }
}
