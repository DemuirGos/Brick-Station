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

#[derive(Debug, Clone, Copy)]
pub struct Registers {
    pub x       : u8, // A 15..7 F 7..0
    pub y       : u8, // D 15..7 E 7..0
    pub a       : u8,  // H 15..7 L 7..0
    pub pc      : u16,
    pub sp      : u8,
    pub flags   : u8, 
    pub status  : u8,
    pub fetched : u8,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            x       : 0,
            y       : 0,
            a       : 0,
            pc      : 0,
            sp      : 0,
            flags   : 0,
            status  : 0,
            fetched : 0,
        }
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        match flag {
            Flag::C => (self.flags & (Flag::C as u8)) != 0,
            Flag::Z => (self.flags & (Flag::Z as u8)) != 0,
            Flag::I => (self.flags & (Flag::I as u8)) != 0,
            Flag::D => (self.flags & (Flag::D as u8)) != 0,
            Flag::B => (self.flags & (Flag::B as u8)) != 0,
            Flag::U => (self.flags & (Flag::U as u8)) != 0,
            Flag::O => (self.flags & (Flag::O as u8)) != 0,
            Flag::N => (self.flags & (Flag::N as u8)) != 0,
        }
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) -> () {
        self.flags = match flag {
            Flag::C => if value { self.flags | (Flag::C as u8) } else { self.flags & !(Flag::C as u8) },
            Flag::Z => if value { self.flags | (Flag::Z as u8) } else { self.flags & !(Flag::Z as u8) },
            Flag::I => if value { self.flags | (Flag::I as u8) } else { self.flags & !(Flag::I as u8) },
            Flag::D => if value { self.flags | (Flag::D as u8) } else { self.flags & !(Flag::D as u8) },
            Flag::B => if value { self.flags | (Flag::B as u8) } else { self.flags & !(Flag::B as u8) },
            Flag::U => if value { self.flags | (Flag::U as u8) } else { self.flags & !(Flag::U as u8) },
            Flag::O => if value { self.flags | (Flag::O as u8) } else { self.flags & !(Flag::O as u8) },
            Flag::N => if value { self.flags | (Flag::N as u8) } else { self.flags & !(Flag::N as u8) },
        }
    }
}