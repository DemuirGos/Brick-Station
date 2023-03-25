pub mod mapper_0;
use self::mapper_0::Mapper0;

use super::interfaces::Originator;

#[derive(Clone)]
pub struct Mapper {
    pub mapper_id: u8,
    pub number_of_prg_banks: u8,
    pub number_of_chr_banks: u8,
}

impl Mapper {
    pub fn new(number_of_prg_banks: u8, number_of_chr_banks: u8) -> Mapper {
        Mapper {
            mapper_id: 0,
            number_of_prg_banks,
            number_of_chr_banks,
        }
    }
}

impl Mapper {
    pub fn forward_read(&self, addr: u16, reader: Originator) -> u16 {
        match self.mapper_id {
            0 => Mapper0::forward_read(self, addr, reader),
            _ => panic!("Mapper not implemented")
        }
    }

    pub fn forward_write(&mut self, addr: u16, reader: Originator) -> u16 {
        match self.mapper_id {
            0 => Mapper0::forward_write(self, addr, reader),
            _ => panic!("Mapper not implemented")
        }
    }
}
