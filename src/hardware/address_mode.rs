use super::cpu::Cpu;
use super::interfaces::DeviceOps;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AddressMode {
    EFF, // effect 
    Imm, // immediate
    Zpr, // zero page
    Rel, // relative
    Abs, // absolute
    Ind, // indirect
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AddressingData {
    pub address_abs : u16,
    pub address_rel : u16,
}

impl AddressingData {
    pub fn new() -> AddressingData {
        AddressingData {
            address_abs : 0,
            address_rel : 0,
        }
    }
}

impl AddressMode {
    pub fn from_str(s: &str) -> AddressMode {
        match s {
            "IMM" => AddressMode::Imm,
            "ZPR" => AddressMode::Zpr,
            "REL" => AddressMode::Rel,
            "ABS" => AddressMode::Abs,
            "IND" => AddressMode::Ind,
            _ => AddressMode::EFF
        }
    }

    pub fn handle(&self, cpu_ref: &mut Cpu) -> bool {
        match self {
            AddressMode::Imm => {
                cpu_ref.registers.pc += 1;
                cpu_ref.address_mode.address_abs = cpu_ref.registers.pc as u16;
                cpu_ref.registers.pc += 1;
                false
            },
            AddressMode::Zpr => {
                let extra = {
                    let register_arg = cpu_ref.read(cpu_ref.registers.pc);
                    if register_arg < 0x00FF { 
                        cpu_ref.registers.banks[register_arg as usize] 
                    } else { 
                        0
                    }
                };
                cpu_ref.registers.pc += 1;
                cpu_ref.address_mode.address_abs = cpu_ref.read(cpu_ref.registers.pc as u16).into();
                cpu_ref.address_mode.address_abs += extra as u16;
                cpu_ref.registers.pc += 1;
                cpu_ref.address_mode.address_abs &= 0x00FF;
                false
            },
            AddressMode::Abs => {
                let extra = {
                    let register_arg = cpu_ref.read(cpu_ref.registers.pc);
                    if register_arg < 0x00FF { 
                        cpu_ref.registers.banks[register_arg as usize] 
                    } else { 
                        0
                    }
                };

                cpu_ref.registers.pc += 1;
                let lo = cpu_ref.read(cpu_ref.registers.pc as u16) as u16;
                cpu_ref.registers.pc += 1;
                let hi = cpu_ref.read(cpu_ref.registers.pc as u16) as u16;
                cpu_ref.registers.pc += 1;
                
                cpu_ref.address_mode.address_abs = (hi << 8) | lo;
                cpu_ref.address_mode.address_abs += extra as u16;
                false
            },
            AddressMode::Ind => {
                let extra :u16 = {
                    let register_arg = cpu_ref.read(cpu_ref.registers.pc);
                    if register_arg < 0x00FF { 
                        cpu_ref.registers.banks[register_arg as usize] as u16
                    } else { 
                        0
                    }
                };
                cpu_ref.registers.pc += 1;
                let lo = cpu_ref.read(cpu_ref.registers.pc as u16) as u16;
                cpu_ref.registers.pc += 1;
                let hi = cpu_ref.read(cpu_ref.registers.pc as u16) as u16;
                cpu_ref.registers.pc += 1;

                let ptr = (hi << 8) | lo;
                let hi_ptr = if lo == 0x00ff { ptr & 0xff00 } else { ptr + 1 };
                cpu_ref.address_mode.address_abs = ((cpu_ref.read(hi_ptr + extra) as u16) << 8) | (cpu_ref.read(ptr + extra) as u16);
                false
            },
            AddressMode::Rel => {
                cpu_ref.registers.pc += 1;
                cpu_ref.address_mode.address_rel = cpu_ref.read(cpu_ref.registers.pc) as u16;
                cpu_ref.registers.pc += 1;
                
                if cpu_ref.address_mode.address_rel & 0x0080 != 0 {
                    cpu_ref.address_mode.address_rel |= 0xFF00;
                }
                false
            }
            _ => false
        }
    }
}