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

pub struct Registers {
    x       : u8, // A 15..7 F 7..0
    y       : u8, // D 15..7 E 7..0
    a       : u8,  // H 15..7 L 7..0
    pc      : u8,
    sp      : u8,
    flags   : u8, 
    status  : u8,
    fetched : u8,
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

    pub fn GetFlag(&self, flag: Flag) -> Flag {
        self.flags & flag
    }

    pub fn SetFlag(&self, flag: Flag) -> Flag {
        self.flags |= flag;
    }
}