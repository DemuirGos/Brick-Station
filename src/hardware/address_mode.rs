use super::cpu::Cpu;
use super::interfaces::DeviceOps;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AddressMode {
    Imp, // implied
    Imm, // immediate
    Zp0, // zero page
    Zpx, // zero page, x
    Zpy, // zero page, y
    Rel, // relative
    Abs, // absolute
    Abx, // absolute, x
    Aby, // absolute, y
    Ind, // indirect
    Izx, // (indirect, x)
    Izy, // (indirect), y
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
            "IMP" => AddressMode::Imp,
            "IMM" => AddressMode::Imm,
            "ZP0" => AddressMode::Zp0,
            "ZPX" => AddressMode::Zpx,
            "ZPY" => AddressMode::Zpy,
            "REL" => AddressMode::Rel,
            "ABS" => AddressMode::Abs,
            "ABX" => AddressMode::Abx,
            "ABY" => AddressMode::Aby,
            "IND" => AddressMode::Ind,
            "IZX" => AddressMode::Izx,
            "IZY" => AddressMode::Izy,
            _ => panic!("Invalid Address Mode: {}", s),
        }
    }

    pub fn handle(&self, cpu_ref: &mut Cpu) -> bool {
        match self {
            AddressMode::Imp => {
                cpu_ref.registers.fetched = cpu_ref.registers.a;
                false
            }, 
            AddressMode::Imm => {
                cpu_ref.address_mode.address_abs = cpu_ref.registers.pc as u16;
                cpu_ref.registers.pc += 1;
                false
            },
            AddressMode::Zp0 => {
                cpu_ref.address_mode.address_abs = cpu_ref.read(cpu_ref.registers.pc as u16).into();
                cpu_ref.registers.pc += 1;
                cpu_ref.address_mode.address_abs &= 0x00FF;
                false
            },
            AddressMode::Zpx => {
                cpu_ref.address_mode.address_abs = cpu_ref.read(cpu_ref.registers.pc as u16).into();
                cpu_ref.address_mode.address_abs+= cpu_ref.registers.x as u16;
                cpu_ref.registers.pc += 1;
                cpu_ref.address_mode.address_abs &= 0x00FF;
                false
            },
            AddressMode::Zpy => {
                cpu_ref.address_mode.address_abs = cpu_ref.read(cpu_ref.registers.pc as u16).into();
                cpu_ref.address_mode.address_abs+= cpu_ref.registers.y as u16;
                cpu_ref.registers.pc += 1;
                cpu_ref.address_mode.address_abs &= 0x00FF;
                false
            },
            AddressMode::Abs => {
                let lo = cpu_ref.read(cpu_ref.registers.pc as u16) as u16;
                cpu_ref.registers.pc += 1;
                let hi = cpu_ref.read(cpu_ref.registers.pc as u16) as u16;
                cpu_ref.registers.pc += 1;
                cpu_ref.address_mode.address_abs = (hi << 8) | lo;
                false
            },
            AddressMode::Abx => {
                let lo = cpu_ref.read(cpu_ref.registers.pc as u16) as u16;
                cpu_ref.registers.pc += 1;
                let hi = cpu_ref.read(cpu_ref.registers.pc as u16) as u16;
                cpu_ref.registers.pc += 1;
                cpu_ref.address_mode.address_abs  = (hi << 8) | lo;
                cpu_ref.address_mode.address_abs += cpu_ref.registers.x as u16;
                if cpu_ref.address_mode.address_abs & 0xFF00 != (hi << 8).into() {
                    true
                }
                else {
                    false
                }
            },
            AddressMode::Aby => {
                let lo = cpu_ref.read(cpu_ref.registers.pc as u16) as u16;
                cpu_ref.registers.pc += 1;
                let hi = cpu_ref.read(cpu_ref.registers.pc as u16) as u16;
                cpu_ref.registers.pc += 1;
                cpu_ref.address_mode.address_abs  = (hi << 8) | lo;
                cpu_ref.address_mode.address_abs += cpu_ref.registers.y as u16;
                if cpu_ref.address_mode.address_abs & 0xFF00 != (hi << 8).into() {
                    true
                }
                else {
                    false
                }
            },
            AddressMode::Ind => {
                let lo = cpu_ref.read(cpu_ref.registers.pc as u16) as u16;
                cpu_ref.registers.pc += 1;
                let hi = cpu_ref.read(cpu_ref.registers.pc as u16) as u16;
                cpu_ref.registers.pc += 1;
                let ptr = (hi << 8) | lo;
                let hi_ptr = if lo == 0x00ff { ptr & 0xff00 } else { ptr + 1 };
                cpu_ref.address_mode.address_abs = (cpu_ref.read(hi_ptr) as u16) << 8 | (cpu_ref.read(ptr) as u16);
                false
            },
            AddressMode::Izx => {
                let t = cpu_ref.read(cpu_ref.registers.pc as u16) as u16;
                cpu_ref.registers.pc += 1;
                
                let lo = cpu_ref.read((t + cpu_ref.registers.x as u16) & 0x00FF) as u16;
                let hi = cpu_ref.read((t + 1 + cpu_ref.registers.x as u16) & 0x00FF) as u16;
                cpu_ref.address_mode.address_abs = (hi << 8) | lo;
                false
            },
            AddressMode::Izy => {
                let t = cpu_ref.read(cpu_ref.registers.pc) as u16;
                cpu_ref.registers.pc += 1;
                
                let lo = cpu_ref.read(t & 0x00FF) as u16;
                let hi = cpu_ref.read((t + 1) & 0x00FF) as u16;
                cpu_ref.address_mode.address_abs = cpu_ref.registers.y  as u16 + ((hi << 8) | lo);
                if cpu_ref.address_mode.address_abs & 0xFF00 != hi << 8 {
                    true
                }
                else {
                    false
                }
            },
            AddressMode::Rel => {
                cpu_ref.address_mode.address_rel = cpu_ref.read(cpu_ref.registers.pc) as u16;
                cpu_ref.registers.pc += 1;
                
                if cpu_ref.address_mode.address_rel & 0x0080 != 0 {
                    cpu_ref.address_mode.address_rel |= 0xFF00;
                }
                false
            }
        }
    }
}