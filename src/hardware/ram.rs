use super::interfaces::{DeviceOps};

#[derive(Debug, Clone, Copy)]
pub struct Ram {
    pub data : [u8; 0xFFFF]
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            data : [0; 0xFFFF]
        }
    }
}

impl DeviceOps for Ram {
    fn within_range(&self, addr: u16) -> bool {
        addr >= 0x0000 && addr <= 0xFFFF
    }

    fn read(&self, addr: u16) -> u8 {
        self.data[addr as usize] 
    }

    fn write(&mut self, addr: u16, value: u8) -> () {
        self.data[addr as usize] = value
    }
}