use super::{address_mode::AddressMode, cpu::Cpu, opcodes::Opcode, registers::Flag, interfaces::DeviceOps};

#[derive(Debug, Clone, PartialEq)]
pub struct Instructions {
    pub mnemonic  : Opcode,
    pub opcode    : u8,
    pub cycles    : u8,
    pub address_mode : AddressMode,
}

impl Instructions {
    pub fn new(mnemonic: Opcode, opcode: u8, cycles: u8, address_mode: AddressMode) -> Instructions {
        Instructions {
            mnemonic : mnemonic,
            opcode    : opcode,
            cycles    : cycles,
            address_mode,
        }
    }

    pub fn operation(&self, cpu_ref: &mut Cpu) -> bool {
        let jump_to_relative_address = |cpu_ref: &mut Cpu| {
            cpu_ref.cycle += 1;

            let value = {
                let target_address = (cpu_ref.address_mode.address_rel & 0x00FF) as i8;
                cpu_ref.registers.pc as i16 + (target_address as i16)
            } as u16;
            
            cpu_ref.address_mode.address_abs = value;
            if cpu_ref.registers.pc >> 8 != cpu_ref.address_mode.address_abs >> 8 {
                cpu_ref.cycle += 1;
            }

            cpu_ref.registers.pc = cpu_ref.address_mode.address_abs;
        };

        cpu_ref.opcode = self.opcode;
        match self.mnemonic {
            Opcode::ADC => todo!(),
            Opcode::SBC => todo!(),
            Opcode::AND => todo!(),
            Opcode::OR => todo!(),
            Opcode::SL => todo!(),
            Opcode::SR => todo!(),
            Opcode::ROL => todo!(),
            Opcode::ROR => todo!(),
            Opcode::BRC => todo!(),
            Opcode::BRS => todo!(),
            Opcode::CMP => todo!(),
            Opcode::DEC => todo!(),
            Opcode::INC => todo!(),
            Opcode::EOR => todo!(),
            Opcode::JMP => todo!(),
            Opcode::LDR => todo!(),
            Opcode::NOP => todo!(),
            Opcode::MOV => todo!(),
            Opcode::PLR => todo!(),
            Opcode::PHR => todo!(),
            Opcode::STR => todo!(),
            Opcode::BRK => todo!(),
            Opcode::JSR => todo!(),
            Opcode::RTS => todo!(),
            Opcode::RTI => todo!(),
        }
    }
}