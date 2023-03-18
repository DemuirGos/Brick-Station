use super::{address_mode::Address_Mode, cpu::Cpu};

pub struct Instructions {
    mnemonic : String,
    opcode    : u8,
    cycles    : u8,
    size      : u8,
    address_mode : Address_Mode,
    operation : Fn(&mut Cpu) -> bool
}

pub static Metadata : [256, Instructions] = [

]; 

impl Instructions {
    fn new(mnemonic: String, opcode: u8, cycles: u8, size: u8, addressMode: Address_Mode, operation: Fn(&mut Cpu) -> u8) -> Instructions {
        Instructions {
            mnemonic : mnemonic,
            opcode    : opcode,
            cycles    : cycles,
            size      : size,
            address_mode : addressMode,
            operation : operation
        }
    }
}
