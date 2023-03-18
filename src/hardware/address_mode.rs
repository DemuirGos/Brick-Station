use super::cpu::Cpu;
use super::interfaces::Device;
pub enum Address_Mode {
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

pub struct Addressing_Data {
    pub address_abs : u16,
    pub address_rel : u16,
}

impl Addressing_Data {
    pub fn new() -> Addressing_Data {
        Addressing_Data {
            address_abs : 0,
            address_rel : 0,
        }
    }
}

impl Address_Mode {
    pub fn handle(&self, cpu_ref: &Cpu) -> bool {
        match self {
            Address_Mode::Imp => {
                cpu_ref.registers.fetched = cpu_ref.registers.a;
                false
            }, 
            Address_Mode::Imm => {
                cpu_ref.address_mode.address_abs = cpu_ref.registers.pc;
                cpu_ref.registers.pc += 1;
                false
            },
            Address_Mode::Zp0 => {
                cpu_ref.address_mode.address_abs = cpu_ref.read(cpu_ref.registers.pc as u16) as u16;
                cpu_ref.registers.pc += 1;
                cpu_ref.address_mode.address_abs &= 0x00FF;
                false
            },
            Address_Mode::Zpx => {
                cpu_ref.address_mode.address_abs = (cpu_ref.read(cpu_ref.registers.pc as u16) + cpu_ref.registers.x) as u16;
                cpu_ref.registers.pc += 1;
                cpu_ref.address_mode.address_abs &= 0x00FF;
                false
            },
            Address_Mode::Zpy => {
                cpu_ref.address_mode.address_abs = (cpu_ref.read(cpu_ref.registers.pc as u16) + cpu_ref.registers.y) as u16;
                cpu_ref.registers.pc += 1;
                cpu_ref.address_mode.address_abs &= 0x00FF;
                false
            },
            Address_Mode::Abs => {
                let lo = cpu_ref.read(cpu_ref.registers.pc as u16);
                cpu_ref.registers.pc += 1;
                let hi = cpu_ref.read(cpu_ref.registers.pc as u16);
                cpu_ref.registers.pc += 1;
                cpu_ref.address_mode.address_abs = (hi << 8) | lo;
                false
            },
            Address_Mode::Abx => {
                let lo = cpu_ref.read(cpu_ref.registers.pc as u16);
                cpu_ref.registers.pc += 1;
                let hi = cpu_ref.read(cpu_ref.registers.pc as u16);
                cpu_ref.registers.pc += 1;
                cpu_ref.address_mode.address_abs = cpu_ref.registers.x + (((hi << 8) | lo) as u8);
                if cpu_ref.address_mode.address_abs & 0xFF00 != hi << 8 {
                    true
                }
                else {
                    false
                }
            },
            Address_Mode::Aby => {
                let lo = cpu_ref.read(cpu_ref.registers.pc as u16);
                cpu_ref.registers.pc += 1;
                let hi = cpu_ref.read(cpu_ref.registers.pc as u16);
                cpu_ref.registers.pc += 1;
                cpu_ref.address_mode.address_abs = cpu_ref.registers.y + (((hi << 8) | lo) as u8);
                if cpu_ref.address_mode.address_abs & 0xFF00 != hi << 8 {
                    true
                }
                else {
                    false
                }
            },
            Address_Mode::Ind => {
                let lo = cpu_ref.read(cpu_ref.registers.pc as u16);
                cpu_ref.registers.pc += 1;
                let hi = cpu_ref.read(cpu_ref.registers.pc as u16);
                cpu_ref.registers.pc += 1;
                let ptr = (hi << 8) | lo;
                let hi_ptr = if lo == 0x00ff { ptr & 0xff00 } else { ptr + 1 };
                cpu_ref.address_mode.address_abs = (cpu_ref.read(hi_ptr as u16)) << 8 | cpu_ref.read(ptr as u16);
                false
            },
            Address_Mode::Izx => {
                let t = cpu_ref.read(cpu_ref.registers.pc as u16);
                cpu_ref.registers.pc += 1;
                
                let lo = cpu_ref.read(((t + cpu_ref.registers.x) & 0x00FF) as u16);
                let hi = cpu_ref.read(((t + cpu_ref.registers.x + 1) & 0x00FF) as u16);
                cpu_ref.address_mode.address_abs = (hi << 8) | lo;
                false
            },
            Address_Mode::Izy => {
                let t = cpu_ref.read(cpu_ref.registers.pc as u16);
                cpu_ref.registers.pc += 1;
                
                let lo = cpu_ref.read((t & 0x00FF) as u16);
                let hi = cpu_ref.read(((t + 1) & 0x00FF) as u16);
                cpu_ref.address_mode.address_abs = cpu_ref.registers.y + ((hi << 8) | lo);
                if cpu_ref.address_mode.address_abs & 0xFF00 != hi << 8 {
                    true
                }
                else {
                    false
                }
            },
            Address_Mode::Rel => {
                cpu_ref.address_mode.address_rel = cpu_ref.read(cpu_ref.registers.pc as u16);
                cpu_ref.registers.pc += 1;
                if cpu_ref.address_mode.address_rel & 0x80 != 0 {
                    cpu_ref.address_mode.address_rel |= 0xFF00;
                }
                false
            }
        }
    }
}