pub mod mapper_0;
use std::{rc::Rc, cell::RefCell};

use self::mapper_0::Mapper0;

use super::{interfaces::DeviceOps, cartridge::{Cartridge, self, Reader}};

#[derive(Clone)]
pub struct Mapper {
    pub mapper_id: u8,
    pub number_of_prg_banks: u8,
    pub number_of_chr_banks: u8,
}

impl Mapper {
    pub fn forward_read(&self, addr: u16, reader: &Reader) -> u16 {
        match self.mapper_id {
            0 => Mapper0::forward_read(self, addr, reader.to_owned()),
            _ => panic!("Mapper not implemented")
        }
    }

    pub fn forward_write(&mut self, addr: u16, reader: &Reader) -> u16 {
        match self.mapper_id {
            0 => Mapper0::forward_write(self, addr, reader.to_owned()),
            _ => panic!("Mapper not implemented")
        }
    }

    fn within_range(&self, addr: u16) -> bool {
        addr >= 0x8000 
    }
}
