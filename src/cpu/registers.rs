mod cpu {
    type Registers {
        AF : u16, // A 15..7 F 7..0
        BC : u16, // B 15..7 C 7..0
        DE : u16, // D 15..7 E 7..0
        HL : u16  // H 15..7 L 7..0
    }
    impl Registers {
        pub fn new() -> Registers {
            Registers {
                AF: 0,
                BC: 0,
                DE: 0,
                HL: 0
            }
        }

        pub fn C(&self) -> u8 {
            self.BC as u8
        }
        pub fn B(&self) -> u8 {
            (self.BC >> 8) as u8
        }

        pub fn F(&self) -> u8 {
            self.AF as u8
        }
        pub fn A(&self) -> u8 {
            (self.AF >> 8) as u8
        }

        pub fn E(&self) -> u8 {
            self.DE as u8
        }
        pub fn D(&self) -> u8 {
            (self.DE >> 8) as u8
        }

        pub fn L(&self) -> u8 {
            self.HL as u8
        }
        pub fn H(&self) -> u8 {
            (self.HL >> 8) as u8
        }
    }
}

