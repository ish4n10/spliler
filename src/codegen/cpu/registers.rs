#[derive(Copy, Clone, Debug)]
pub enum RegisterList {
    invalid,
    r8,
    r9,
    r10,
    r11,
}

impl RegisterList {
    pub fn to_str(&self) -> &'static str {
        match self {
            RegisterList::invalid => "invalid",
            RegisterList::r10 => "r10",
            RegisterList::r11 => "r11",
            RegisterList::r8 => "r8",
            RegisterList::r9 => "r9",
        }
    }
}


#[derive(Copy, Clone, Debug)]
pub struct RegisterImpl {
    pub reg: RegisterList,
    pub in_use: bool, // free or in use
}


impl RegisterImpl {
    pub fn create_empty() -> Self {
        RegisterImpl { reg: RegisterList::invalid, in_use: true }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Registers {
    registers: [RegisterImpl; 4],
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            registers: [
                RegisterImpl {
                    reg: RegisterList::r8,
                    in_use: false,
                },
                RegisterImpl {
                    reg: RegisterList::r9,
                    in_use: false,
                },
                RegisterImpl {
                    reg: RegisterList::r10,
                    in_use: false,
                },
                RegisterImpl {
                    reg: RegisterList::r11,
                    in_use: false,
                },
            ],
        }
    }

    pub fn free_all(&mut self) {
        for register in &mut self.registers {
            register.in_use = false;
        }
    }

    pub fn free_register(&mut self, reg: &mut RegisterImpl) {
        for register in &mut self.registers {
          if register.reg.to_str() == reg.reg.to_str() {
            register.in_use = false;
          }
        }
    }
    
    pub fn alloc_register(&mut self, value: i64) -> Option<&RegisterImpl> {
        
        for register in &mut self.registers {
            if register.in_use == false {
                register.in_use = true;
                return Some(register);
            }
        }
        return None;
    }

    
}
