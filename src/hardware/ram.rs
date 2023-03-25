use super::{interfaces::{DeviceOps, Originator}};

#[derive(Debug, Clone, Copy)]
pub struct Ram {
    pub data : [u8; 2048]
}

impl Ram {
    pub fn new() -> Ram {
        Ram {
            data : [0; 2048]
        }
    }
}

impl DeviceOps for Ram {
    fn within_range(&self, addr: u16) -> bool {
        addr <= 0x1FFF
    }

    fn read(&self, addr: u16, _:Originator) -> u8 {
        self.data[(addr as usize) & 0x7FF] 
    }

    fn write(&mut self, addr: u16, value: u8, _:Originator) -> () {
        self.data[(addr as usize) & 0x7FF] = value
    }
}