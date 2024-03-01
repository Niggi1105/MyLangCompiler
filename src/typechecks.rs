use crate::ast::BodyAST;

struct TypeChecking {
    ast: BodyAST,
}

impl TypeChecking {
    ///does type checking and infering when possible
    ///
    ///returns the AST with each variable having a type assigned
    ///
    ///panics if the type of a variable can not be infered or there are other conflicts
    pub fn check_types(self) -> Self {
        unimplemented!()
    }
}
