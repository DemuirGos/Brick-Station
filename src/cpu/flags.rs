type Flags {
    fs : u8
}

impl Flags {
    pub fn new() -> Self {
        Flags {
            0
        }
    }

    pub fn Z(&self) -> u8 {
        (self.fs >> 7) && 1
    }
    pub fn N(&self) -> u8 {
        (self.fs >> 6) && 1
    }
    pub fn H(&self) -> u8 {
        (self.fs >> 5) && 1
    }
    pub fn C(&self) -> u8 {
        (self.fs >> 4) && 1
    }
}