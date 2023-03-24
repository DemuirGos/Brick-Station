#[derive(Debug, Clone, Copy)]
pub enum Flag {
    C = 1 << 0, // Carry
    Z = 1 << 1, // Zero
    I = 1 << 2, // Interrupt
    D = 1 << 3, // Decimal
    B = 1 << 4, // Break
    U = 1 << 5, // Unused
    O = 1 << 6, // Overflow
    N = 1 << 7, // Negative
}

#[derive(Clone)]
pub struct Registers{
    pub  banks : [u8; 0xFF],
    pub  pc    : u16,
    pub  sp    : u8,
    pub  status: u8,
    pub  local : u8,
} 

impl Registers {
    pub fn new() -> Registers {
        Registers {
            banks : [0; 0xFF],
            pc    : 0,
            sp    : 0,
            status: 0,
            local : 0,
        }
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        match flag {
            Flag::C => (self.status & (Flag::C as u8)) != 0,
            Flag::Z => (self.status & (Flag::Z as u8)) != 0,
            Flag::I => (self.status & (Flag::I as u8)) != 0,
            Flag::D => (self.status & (Flag::D as u8)) != 0,
            Flag::B => (self.status & (Flag::B as u8)) != 0,
            Flag::U => (self.status & (Flag::U as u8)) != 0,
            Flag::O => (self.status & (Flag::O as u8)) != 0,
            Flag::N => (self.status & (Flag::N as u8)) != 0,
        }
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) -> () {
        self.status = match flag {
            Flag::C => if value { self.status | (Flag::C as u8) } else { self.status & !(Flag::C as u8) },
            Flag::Z => if value { self.status | (Flag::Z as u8) } else { self.status & !(Flag::Z as u8) },
            Flag::I => if value { self.status | (Flag::I as u8) } else { self.status & !(Flag::I as u8) },
            Flag::D => if value { self.status | (Flag::D as u8) } else { self.status & !(Flag::D as u8) },
            Flag::B => if value { self.status | (Flag::B as u8) } else { self.status & !(Flag::B as u8) },
            Flag::U => if value { self.status | (Flag::U as u8) } else { self.status & !(Flag::U as u8) },
            Flag::O => if value { self.status | (Flag::O as u8) } else { self.status & !(Flag::O as u8) },
            Flag::N => if value { self.status | (Flag::N as u8) } else { self.status & !(Flag::N as u8) },
        }
    }
}