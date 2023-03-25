use crate::hardware::interfaces::Originator;

use super::Mapper;

pub struct Mapper0();

impl Mapper0 {
    pub fn forward_read(mapper :&Mapper, addr: u16, reader: Originator) -> u16 {
        match (addr, reader) {
            (0x8000..=0xFFFF, Originator::Cpu)  => 
                addr & (if mapper.number_of_prg_banks > 1 {
                    0x7FFF
                } else {
                    0x3FFF
                }),
            (0x0000..=0x1FFF, Originator::Ppu)  => addr,
            _ => panic!("Invalid address")
        }
    }

    pub fn forward_write(mapper :&Mapper, addr: u16, reader: Originator) -> u16 {
        match (addr, reader) {
            (0x8000..=0xFFFF, Originator::Cpu)  => 
                addr & (if mapper.number_of_prg_banks > 1 {
                    0x7FFF
                } else {
                    0x3FFF
                }),
            (0x0000..=0x1FFF, Originator::Ppu)  => 
                if mapper.number_of_chr_banks > 0 {
                    addr
                } else {
                    panic!("Invalid address")
                },
            _ => panic!("Invalid address")
        }
    }
}