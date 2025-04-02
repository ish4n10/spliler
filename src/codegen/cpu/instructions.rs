use std::{fs::File, io::Write};

use crate::{codegen::cpu::registers::Registers, parser::ast::ASTNode, parser::ast::ASTNodeType};
use super::registers::RegisterImpl;

pub struct InstructionList {
    file_handle: File,
    register: Registers,
}

impl InstructionList {
    pub fn new(name: String) -> Self {
        InstructionList { 
            file_handle: File::options().create(true).append(true).open(name).unwrap(),
            register: Registers::new(),
        }
    }

    /// **Compile AST recursively**
    pub fn compile(&mut self, root: &Box<ASTNode>) -> RegisterImpl {
        match root.get_operation() {
            ASTNodeType::AIntLit => {
                // Load integer into a register
                let value = root.get_value().unwrap_or(0);
                return self.i_load(value);
            }
            ASTNodeType::AAdd => {
                let left_result = self.compile(root.get_left().as_ref().expect("Left operand missing"));
                let right_result = self.compile(root.get_right().as_ref().expect("Right operand missing"));
                return self.i_add(left_result, right_result);
            }
            _ => panic!("Unsupported ASTNodeType: {:?}", root.get_operation()),
        }
    }


    pub fn preamble(&mut self) {
        let data = "\
        section .text\n\
        global _start\n\
        _start:\n\
        \tpush rbp\n\
        \tmov rbp, rsp\n";
        self.write_data(data);
    }

    pub fn finalize(&mut self) {
        let data = "\
        \tpop rbp\n\
        \tret\n";
        self.write_data(data);
    }
    /// **Write assembly instruction to file**
    fn write_data(&mut self, data: &str) { 
        let _ = self.file_handle.write_all(data.as_bytes());
    }
}

impl InstructionList {

    pub fn i_load(&mut self, value: i64) -> RegisterImpl {
        let reg = {
            let allocated_reg = self.register.alloc_register(value)
                .expect("No available register");
            allocated_reg.clone() 
        };
    
        let assembly_ins = format!("\tmov\t{}, {}\n", reg.reg.to_str(), value);
        self.write_data(&assembly_ins);
        reg
    }

    pub fn i_add(&mut self, reg1: RegisterImpl, mut reg2: RegisterImpl) -> RegisterImpl {
        let assembly_ins = format!("\tadd\t{}, {}\n", reg1.reg.to_str(), reg2.reg.to_str());
        self.write_data(&assembly_ins);
        self.register.free_register(&mut reg2);
        reg1
    }
}