use crate::ast::BodyAST;

struct ASMCodeGenerator {
    ast: BodyAST,
    cpu_specs: Specs,
    instr_set: InstrSet,
}

struct Specs {
    registers: u8,
    ram_size: u16,
}

struct InstrSet {}

impl ASMCodeGenerator {}
