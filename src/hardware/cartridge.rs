use std::{rc::Rc, cell::RefCell};

use super::{interfaces::DeviceOps, bus::Bus, mapper::Mapper};

#[derive(Clone)]
pub enum Reader {
    Cpu, Ppu
}

#[derive(Clone)]
pub struct Memory {
    pub program_region: Region,  
    pub palette_region: Region,  
} 
#[derive(Clone)]
pub struct Region {
    pub data: Vec<u8>,
    pub number_of_banks: u8,
}

#[derive(Clone)]
pub struct Cartridge<'a> {
    pub bus    : Option<Rc<RefCell<Bus<'a>>>>,
    pub memory : Memory,
    pub mapper : Mapper,
    pub mode   : Reader
} 

impl<'a> DeviceOps for Cartridge<'a>
{
    fn read(&self, addr: u16) -> u8 {
        let forward_addr = self.mapper.forward_read(addr, &self.mode);
        match self.mode {
            Reader::Cpu => {
                self.memory.program_region.data[forward_addr as usize]
            },
            Reader::Ppu => {
                self.memory.palette_region.data[forward_addr as usize]
            }
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> () {
        let forward_addr = self.mapper.forward_write(addr, &self.mode);
        match self.mode {
            Reader::Cpu => {
                self.memory.program_region.data[forward_addr as usize] = value;
            },
            Reader::Ppu => {
                self.memory.palette_region.data[forward_addr as usize] = value;
            }
        }
    }

    fn within_range(&self, addr: u16) -> bool {
        addr >= 0x8000 && addr <= 0xFFFF
    }
} 