use std::{fs::File, io::Write};

use crate::codegen::cpu::registers::Registers;
use super::registers::RegisterImpl;

pub struct InstructionList {
    file_name: String,
    file_handle: File,
    register: Registers,

}

impl InstructionList {
    pub fn new(name: String) -> Self {
        InstructionList { 
            file_name: name.clone(), 
            file_handle: File::options().create(true).append(true).open(name).unwrap(),
            register: Registers::new()
        }
    }

    pub fn write_data (&mut self, data: &str) { 
        let _ = self.file_handle.write(data.as_bytes());
        return;
    }
}


impl InstructionList {
    // for now it will panic if the registers are not found
    pub fn i_load(&mut self, value: i64) -> RegisterImpl {
        let allocated_register: RegisterImpl = {
            let reg = self.register.alloc_register(value)
                .expect("No available register"); 
            reg.clone() 
        };

        let assembly_ins = format!("\tmovq\t{}, {}\n", value, allocated_register.reg.to_str());

        self.write_data(&assembly_ins);
        return allocated_register;
    }

    pub fn i_add(reg1: &mut RegisterImpl, reg2: &mut RegisterImpl) -> RegisterImpl {
        let answer = reg1.value.unwrap_or(0) + reg2.value.unwrap_or(0);
        reg1.in_use = true;
        reg1.value = Some(answer);
        return *reg1;
    }
    
}

