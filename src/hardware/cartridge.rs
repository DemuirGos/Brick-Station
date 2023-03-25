use std::{rc::Rc, cell::RefCell};

use super::{interfaces::{DeviceOps, Originator}, bus::Bus, mapper::Mapper};

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
} 

impl<'a> Cartridge<'a> {
    pub fn new(number_of_chr_banks: u8, number_of_prg_banks: u8) -> Cartridge<'a> {
        Cartridge {
            bus    : None,
            memory : Memory {
                program_region: Region {
                    data: vec![0; 0x8000],
                    number_of_banks: number_of_prg_banks
                },
                palette_region: Region {
                    data: vec![0; 0x2000],
                    number_of_banks: number_of_chr_banks
                }
            },
            mapper : Mapper::new(number_of_prg_banks, number_of_chr_banks)
        }
    }
}

impl<'a> DeviceOps for Cartridge<'a>
{
    fn read(&self, addr: u16, mode: Originator) -> u8 {
        let forward_addr = self.mapper.forward_read(addr, mode);
        match mode {
            Originator::Cpu => {
                self.memory.program_region.data[forward_addr as usize]
            },
            Originator::Ppu => {
                self.memory.palette_region.data[forward_addr as usize]
            }
        }
    }

    fn write(&mut self, addr: u16, value: u8, mode: Originator) -> () {
        let forward_addr = self.mapper.forward_write(addr, mode);
        match mode {
            Originator::Cpu => {
                self.memory.program_region.data[forward_addr as usize] = value;
            },
            Originator::Ppu => {
                self.memory.palette_region.data[forward_addr as usize] = value;
            }
        }
    }

    fn within_range(&self, addr: u16) -> bool {
        addr >= 0x8000 
    }
} 