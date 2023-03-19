use std::{collections::HashMap, fs::File, io::{BufReader, BufRead}};

use super::{address_mode::AddressMode, cpu::Cpu};

#[derive(Debug, Clone, PartialEq)]
pub struct Instructions {
    pub mnemonic : String,
    pub opcode    : u8,
    pub cycles    : u8,
    pub address_mode : AddressMode,
}

impl Instructions {
    pub fn new(mnemonic: String, opcode: u8, cycles: u8, addressMode: AddressMode) -> Instructions {
        Instructions {
            mnemonic : mnemonic,
            opcode    : opcode,
            cycles    : cycles,
            address_mode : addressMode,
        }
    }

    pub fn operation(&self, cpu_ref: &mut Cpu) -> bool {
        false
    }
}
