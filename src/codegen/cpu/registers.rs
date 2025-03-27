pub enum RegisterList {
    r8,
    r9,
    r10,
    r11,
}

impl RegisterList {
    fn to_str(&self) -> &'static str {
        match self {
            RegisterList::r10 => "r10",
            RegisterList::r11 => "r11",
            RegisterList::r8 => "r8",
            RegisterList::r9 => "r9",
        }
    }
}

struct RegisterImpl {
    reg: RegisterList,
    in_use: bool, // free or in use
    value: Option<i64>,
}

struct Registers {
    registers: [RegisterImpl; 4],
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            registers: [
                RegisterImpl {
                    reg: RegisterList::r8,
                    in_use: false,
                    value: None,
                },
                RegisterImpl {
                    reg: RegisterList::r9,
                    in_use: false,
                    value: None,
                },
                RegisterImpl {
                    reg: RegisterList::r10,
                    in_use: false,
                    value: None,
                },
                RegisterImpl {
                    reg: RegisterList::r11,
                    in_use: false,
                    value: None,
                },
            ],
        }
    }

    pub fn free_all(&mut self) {
        for register in &mut self.registers {
            register.in_use = false;
            register.value = None;
        }
    }

    pub fn alloc_register(&mut self, value: i64) -> Option<&RegisterImpl> {
        
        for register in &mut self.registers {
            if register.in_use == false {
                register.in_use = true;
                register.value = Some(value);
                return Some(register);
            }
        }
        return None;
    }

    
}
