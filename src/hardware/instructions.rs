use super::{address_mode::AddressMode, cpu::{Cpu, self}, opcodes::Opcode, registers::Flag, interfaces::Device};

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
            cpu_ref.address_mode.address_abs = cpu_ref.registers.pc as u16 + cpu_ref.address_mode.address_rel;
            if(cpu_ref.registers.pc >> 8 != cpu_ref.address_mode.address_abs >> 8) {
                cpu_ref.cycle += 1;
            }

            cpu_ref.registers.pc = cpu_ref.address_mode.address_abs;
        };

        match self.mnemonic {
            Opcode::ADC => {
                cpu_ref.fetch();
                let value = cpu_ref.registers.fetched as u16;
                let result = cpu_ref.registers.a as u16 + value + cpu_ref.registers.get_flag(Flag::C) as u16;
                
                cpu_ref.registers.set_flag(Flag::C, result > 255);
                cpu_ref.registers.set_flag(Flag::Z, (result & 0x00FF) == 0);
                cpu_ref.registers.set_flag(Flag::N, (result & 0x80) != 0);
                cpu_ref.registers.set_flag(Flag::O, (result ^ cpu_ref.registers.a as u16) & (result ^ value) & 0x0080 != 0);

                cpu_ref.registers.a = result as u8;
                true
            },
            Opcode::SBC => {
                cpu_ref.fetch();

                let value = cpu_ref.registers.fetched as u16;
                let result = cpu_ref.registers.a as u16 - value - (1 - cpu_ref.registers.get_flag(Flag::C) as u16);
                
                cpu_ref.registers.set_flag(Flag::C, result > 255);
                cpu_ref.registers.set_flag(Flag::Z, (result & 0x00FF) == 0);
                cpu_ref.registers.set_flag(Flag::N, (result & 0x80) != 0);
                cpu_ref.registers.set_flag(Flag::O, (result ^ cpu_ref.registers.a as u16) & (result ^ value) & 0x0080 != 0);

                cpu_ref.registers.a = result as u8;
                true
            },
            Opcode::BIT => {
                cpu_ref.fetch();
                let value = cpu_ref.registers.a as u16 & cpu_ref.registers.fetched as u16 ; 
                cpu_ref.registers.set_flag(Flag::Z, value > 1 << 6);
                cpu_ref.registers.set_flag(Flag::Z, value == 0x00);
                cpu_ref.registers.set_flag(Flag::N, value & 0x80 != 0);
                false
            }
            Opcode::AND => {
                cpu_ref.fetch();
                cpu_ref.registers.a &= cpu_ref.registers.fetched;
                cpu_ref.registers.set_flag(Flag::Z, cpu_ref.registers.a == 0x00);
                cpu_ref.registers.set_flag(Flag::N, cpu_ref.registers.a & 0x80 != 0);
                true
            },
            Opcode::ORA => {
                cpu_ref.fetch();
                cpu_ref.registers.a |= cpu_ref.registers.fetched;
                cpu_ref.registers.set_flag(Flag::Z, cpu_ref.registers.a == 0x00);
                cpu_ref.registers.set_flag(Flag::N, cpu_ref.registers.a & 0x80 != 0);
                true
            },
            Opcode::ASL => {
                cpu_ref.fetch();
                let result = (cpu_ref.registers.fetched as u16) << 1;
                cpu_ref.registers.set_flag(Flag::N, result & 0x80 != 0);
                cpu_ref.registers.set_flag(Flag::Z, result & 0x80 != 0);
                cpu_ref.registers.set_flag(Flag::C, result > 255);

                if self.address_mode == AddressMode::Imp {
                    cpu_ref.registers.a = result as u8;
                } else {
                    cpu_ref.write(cpu_ref.address_mode.address_abs, result as u8)
                }

                false
            },
            Opcode::LSR => {
                cpu_ref.fetch();
                let result = (cpu_ref.registers.fetched as u16) >> 1;
                cpu_ref.registers.set_flag(Flag::N, result & 0x0080 != 0);
                cpu_ref.registers.set_flag(Flag::Z, result & 0x00FF == 0);

                if self.address_mode == AddressMode::Imp {
                    cpu_ref.registers.a = result as u8;
                } else {
                    cpu_ref.write(cpu_ref.address_mode.address_abs, result as u8)
                }

                false
            },
            Opcode::ROR => {
                cpu_ref.fetch();
                let result = ((cpu_ref.registers.fetched as u16) << 1) | cpu_ref.registers.get_flag(Flag::C) as u16;
                cpu_ref.registers.set_flag(Flag::C, result & 0xFF00 != 0);
                cpu_ref.registers.set_flag(Flag::N, result & 0x0080 != 0);
                cpu_ref.registers.set_flag(Flag::Z, result & 0x00FF == 0);

                if self.address_mode == AddressMode::Imp {
                    cpu_ref.registers.a = result as u8;
                } else {
                    cpu_ref.write(cpu_ref.address_mode.address_abs, result as u8)
                }

                false
            },
            Opcode::ROL => {
                cpu_ref.fetch();
                let result = ((cpu_ref.registers.fetched as u16) >> 1) | ((cpu_ref.registers.get_flag(Flag::C) as u16) << 7);
                cpu_ref.registers.set_flag(Flag::C, result & 0xFF00 != 0);
                cpu_ref.registers.set_flag(Flag::N, result & 0x0080 != 0);
                cpu_ref.registers.set_flag(Flag::Z, result & 0x00FF == 0);

                if self.address_mode == AddressMode::Imp {
                    cpu_ref.registers.a = result as u8;
                } else {
                    cpu_ref.write(cpu_ref.address_mode.address_abs, result as u8)
                }

                false
            },
            Opcode::BCC => {
                if(!cpu_ref.registers.get_flag(Flag::C)) {
                    jump_to_relative_address(cpu_ref);
                }

                false
            },
            Opcode::BCS => {
                if(cpu_ref.registers.get_flag(Flag::C)) {
                    jump_to_relative_address(cpu_ref);
                }

                false
            },
            Opcode::BEQ => {
                if(cpu_ref.registers.get_flag(Flag::Z)) {
                    jump_to_relative_address(cpu_ref);
                }

                false
            },
            Opcode::BNE => {
                if(!cpu_ref.registers.get_flag(Flag::Z)) {
                    jump_to_relative_address(cpu_ref);
                }
                false
            },
            Opcode::BMI => {
                if(cpu_ref.registers.get_flag(Flag::N)) {
                    jump_to_relative_address(cpu_ref);
                }
                false
            },
            Opcode::BPL => {
                if(!cpu_ref.registers.get_flag(Flag::N)) {
                    jump_to_relative_address(cpu_ref);
                }
                false
            },
            Opcode::BVC => {
                if(!cpu_ref.registers.get_flag(Flag::O)) {
                    jump_to_relative_address(cpu_ref);
                }
                false
            },
            Opcode::BVS => {
                if(cpu_ref.registers.get_flag(Flag::O)) {
                    jump_to_relative_address(cpu_ref);
                }
                false
            },
            Opcode::CLC => {
                cpu_ref.registers.set_flag(Flag::C, false);
                false
            },
            Opcode::CLD => {
                cpu_ref.registers.set_flag(Flag::D, false);
                false
            },
            Opcode::CLI => {
                cpu_ref.registers.set_flag(Flag::I, false);
                false
            },
            Opcode::CLV => {
                cpu_ref.registers.set_flag(Flag::O, false);
                false
            },
            Opcode::CLV => {
                cpu_ref.registers.set_flag(Flag::O, false);
                false
            },
            Opcode::CMP => {
                cpu_ref.fetch();
                
                let value = cpu_ref.registers.a as u16 - cpu_ref.registers.fetched as u16;
                cpu_ref.registers.set_flag(Flag::C, value >= 0);
                cpu_ref.registers.set_flag(Flag::Z, value == 0);
                cpu_ref.registers.set_flag(Flag::N, value & 0x0080 != 0);

                true
            },
            Opcode::CPX => {
                cpu_ref.fetch();
                
                let value = cpu_ref.registers.x as u16 - cpu_ref.registers.fetched as u16;
                cpu_ref.registers.set_flag(Flag::C, value >= 0);
                cpu_ref.registers.set_flag(Flag::Z, value == 0);
                cpu_ref.registers.set_flag(Flag::N, value & 0x0080 != 0);

                true
            },
            Opcode::CPY => {
                cpu_ref.fetch();
                
                let value = cpu_ref.registers.y as u16 - cpu_ref.registers.fetched as u16;
                cpu_ref.registers.set_flag(Flag::C, value >= 0);
                cpu_ref.registers.set_flag(Flag::Z, value == 0);
                cpu_ref.registers.set_flag(Flag::N, value & 0x0080 != 0);

                true
            },
            Opcode::DEC => {
                cpu_ref.fetch();
                let value = cpu_ref.registers.fetched - 1;
                cpu_ref.write(cpu_ref.address_mode.address_abs, value as u8);
                cpu_ref.registers.set_flag(Flag::Z, value == 0);
                cpu_ref.registers.set_flag(Flag::N, value & 0x0080 != 0);
                false
            },
            Opcode::INC => {
                cpu_ref.fetch();
                let value = cpu_ref.registers.fetched + 1;
                cpu_ref.write(cpu_ref.address_mode.address_abs, value as u8);
                cpu_ref.registers.set_flag(Flag::Z, value == 0);
                cpu_ref.registers.set_flag(Flag::N, value & 0x0080 != 0);
                false
            },
            Opcode::DEX => {
                cpu_ref.registers.x -= 1;
                cpu_ref.registers.set_flag(Flag::Z, cpu_ref.registers.x == 0);
                cpu_ref.registers.set_flag(Flag::N, cpu_ref.registers.x & 0x0080 != 0);
                false
            },
            Opcode::INX => {
                cpu_ref.registers.x += 1;
                cpu_ref.registers.set_flag(Flag::Z, cpu_ref.registers.x == 0);
                cpu_ref.registers.set_flag(Flag::N, cpu_ref.registers.x & 0x0080 != 0);
                false
            },
            Opcode::DEY => {
                cpu_ref.registers.y -= 1;
                cpu_ref.registers.set_flag(Flag::Z, cpu_ref.registers.x == 0);
                cpu_ref.registers.set_flag(Flag::N, cpu_ref.registers.x & 0x0080 != 0);
                false
            },
            Opcode::INY => {
                cpu_ref.registers.y += 1;
                cpu_ref.registers.set_flag(Flag::Z, cpu_ref.registers.x == 0);
                cpu_ref.registers.set_flag(Flag::N, cpu_ref.registers.x & 0x0080 != 0);
                false
            },
            Opcode::EOR => {
                cpu_ref.fetch();
                cpu_ref.registers.a ^= cpu_ref.registers.fetched;
                cpu_ref.registers.set_flag(Flag::Z, cpu_ref.registers.a == 0);
                cpu_ref.registers.set_flag(Flag::N, cpu_ref.registers.a & 0x0080 != 0);
                false
            },
            Opcode::JMP => {
                cpu_ref.registers.pc = cpu_ref.address_mode.address_abs;
                false
            },
            Opcode::LDA => {
                cpu_ref.fetch();
                cpu_ref.registers.a = cpu_ref.registers.fetched;
                cpu_ref.registers.set_flag(Flag::Z, cpu_ref.registers.a == 0);
                cpu_ref.registers.set_flag(Flag::N, cpu_ref.registers.a & 0x0080 != 0);
                false
            },
            Opcode::LDX => {
                cpu_ref.fetch();
                cpu_ref.registers.x = cpu_ref.registers.fetched;
                cpu_ref.registers.set_flag(Flag::Z, cpu_ref.registers.x == 0);
                cpu_ref.registers.set_flag(Flag::N, cpu_ref.registers.x & 0x0080 != 0);
                false
            },
            Opcode::LDY => {
                cpu_ref.fetch();
                cpu_ref.registers.y = cpu_ref.registers.fetched;
                cpu_ref.registers.set_flag(Flag::Z, cpu_ref.registers.y == 0);
                cpu_ref.registers.set_flag(Flag::N, cpu_ref.registers.y & 0x0080 != 0);
                false
            },
            Opcode::NOP => {
                match self.opcode {
                    0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => true,
                    _ => false
                }
            },
            Opcode::SEC => {
                cpu_ref.registers.set_flag(Flag::C, true);
                false
            },
            Opcode::SED => {
                cpu_ref.registers.set_flag(Flag::D, true);
                false
            },
            Opcode::SEI => {
                cpu_ref.registers.set_flag(Flag::I, true);
                false
            },
            Opcode::STA => {
                cpu_ref.write(cpu_ref.address_mode.address_abs, cpu_ref.registers.a);
                false
            },
            Opcode::STX => {
                cpu_ref.write(cpu_ref.address_mode.address_abs, cpu_ref.registers.x);
                false
            },
            Opcode::STY => {
                cpu_ref.write(cpu_ref.address_mode.address_abs, cpu_ref.registers.y);
                false
            },
            Opcode::TAX => {
                cpu_ref.registers.x = cpu_ref.registers.a;
                cpu_ref.registers.set_flag(Flag::Z, cpu_ref.registers.x == 0);
                cpu_ref.registers.set_flag(Flag::N, cpu_ref.registers.x & 0x0080 != 0);
                false
            },
            Opcode::TAY => {
                cpu_ref.registers.y = cpu_ref.registers.a;
                cpu_ref.registers.set_flag(Flag::Z, cpu_ref.registers.y == 0);
                cpu_ref.registers.set_flag(Flag::N, cpu_ref.registers.y & 0x0080 != 0);
                false
            },
            Opcode::TSX => {
                cpu_ref.registers.x = cpu_ref.registers.sp;
                cpu_ref.registers.set_flag(Flag::Z, cpu_ref.registers.x == 0);
                cpu_ref.registers.set_flag(Flag::N, cpu_ref.registers.x & 0x0080 != 0);
                false
            },
            Opcode::TXA => {
                cpu_ref.registers.a = cpu_ref.registers.x;
                cpu_ref.registers.set_flag(Flag::Z, cpu_ref.registers.a == 0);
                cpu_ref.registers.set_flag(Flag::N, cpu_ref.registers.a & 0x0080 != 0);
                false
            },
            Opcode::TXS => {
                cpu_ref.registers.sp = cpu_ref.registers.x;
                false
            },
            Opcode::TYA => {
                cpu_ref.registers.a = cpu_ref.registers.y;
                cpu_ref.registers.set_flag(Flag::Z, cpu_ref.registers.a == 0);
                cpu_ref.registers.set_flag(Flag::N, cpu_ref.registers.a & 0x0080 != 0);
                false
            },
            Opcode::PHA => {
                cpu_ref.write(0x0100 + cpu_ref.registers.sp as u16, cpu_ref.registers.a);
                cpu_ref.registers.sp -= 1;
                false
            },
            Opcode::PLA => {
                cpu_ref.registers.sp += 1;
                cpu_ref.registers.a =cpu_ref.read(0x0100 + cpu_ref.registers.sp as u16);
                cpu_ref.registers.set_flag(Flag::Z, cpu_ref.registers.a == 0);
                cpu_ref.registers.set_flag(Flag::N, cpu_ref.registers.a & 0x0080 != 0);
                false
            },
            Opcode::PHP => {
                cpu_ref.registers.set_flag(Flag::B, true);
                cpu_ref.registers.set_flag(Flag::U, true);
                cpu_ref.write(0x0100 + cpu_ref.registers.sp as u16, cpu_ref.registers.status);
                cpu_ref.registers.set_flag(Flag::B, false);
                cpu_ref.registers.set_flag(Flag::U, false);
                cpu_ref.registers.sp -= 1;
                false
            },
            Opcode::PLP => {
                cpu_ref.registers.sp += 1;
                cpu_ref.registers.status = cpu_ref.read(0x0100 + cpu_ref.registers.sp as u16);
                cpu_ref.registers.set_flag(Flag::U, false);
                false
            },
            Opcode::BRK => {
                cpu_ref.registers.pc += 1;
                cpu_ref.registers.set_flag(Flag::I, true);
                
                cpu_ref.write(0x0100 + cpu_ref.registers.sp as u16 - 0, ((cpu_ref.registers.pc >> 8) as u8));
                cpu_ref.write(0x0100 + cpu_ref.registers.sp as u16 - 1, cpu_ref.registers.pc as u8);
                cpu_ref.registers.sp -= 2;

                cpu_ref.registers.set_flag(Flag::B, true);
                cpu_ref.write(0x0100 + cpu_ref.registers.sp as u16, cpu_ref.registers.status as u8);
                cpu_ref.registers.sp -= 1;
                cpu_ref.registers.set_flag(Flag::B, false);

                let lo = cpu_ref.read(0xFFFE) as u16;
                let hi = cpu_ref.read(0xFFFE + 1) as u16;
                cpu_ref.registers.pc += (hi << 8) | lo;

                false
            },
            Opcode::JSR => {
                cpu_ref.registers.pc -= 1;

                cpu_ref.write(0x0100 + cpu_ref.registers.sp as u16 - 0, (cpu_ref.registers.pc >> 8) as u8);    
                cpu_ref.write(0x0100 + cpu_ref.registers.sp as u16 - 1, (cpu_ref.registers.pc) as u8);    
                cpu_ref.registers.sp -= 2;

                cpu_ref.registers.pc = cpu_ref.address_mode.address_abs;
                false
            },
            Opcode::RTS => {
                cpu_ref.registers.sp += 1;
                let lo = cpu_ref.read(0x0100 + cpu_ref.registers.sp as u16) as u16;    

                cpu_ref.registers.sp += 1;
                let hi = cpu_ref.read(0x0100 + cpu_ref.registers.sp as u16) as u16;    

                cpu_ref.registers.pc = (hi << 8) | lo; 
                false
            },
            Opcode::RTI => {
                cpu_ref.registers.sp += 1;
                cpu_ref.registers.status = cpu_ref.read(0x0100 + cpu_ref.registers.sp as u16);
                cpu_ref.registers.set_flag(Flag::B, false);
                cpu_ref.registers.set_flag(Flag::U, false);

                cpu_ref.registers.sp += 1;
                let lo = cpu_ref.read(0x0100 + cpu_ref.registers.sp as u16) as u16;    

                cpu_ref.registers.sp += 1;
                let hi = cpu_ref.read(0x0100 + cpu_ref.registers.sp as u16) as u16;    

                cpu_ref.registers.pc = (hi << 8) | lo; 
                false
            }
            _ => false 
        }
    }
}