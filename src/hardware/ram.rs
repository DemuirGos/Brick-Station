use super::interfaces::Device;

pub struct Ram {
    pub Data : [u8; 0xFFFF]
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            Data : [0; 0xFFFF]
        }
    }
}
impl Device for Ram {
    fn withinRange(&self, addr: u16) -> bool {
        addr >= 0x0000 && addr <= 0xFFFF
    }

    fn read(&self, addr: u16) -> u8 {
        self.Data[addr as usize]
    }

    fn write(&self, addr: u16, value: u8) -> () {
        self.Data[addr as usize] = value
    }
}