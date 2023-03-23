use super::{interfaces::{DeviceOps}, ram::Ram, cpu::Cpu};

#[derive(Clone)]
pub enum Device<'a> {
    Ram(Ram),
    Cpu(Cpu<'a>)
}

impl<'a> DeviceOps for Device<'a> {
    fn within_range(&self, addr: u16) -> bool {
        match self {
            Device::Ram(ram) => ram.within_range(addr),
            Device::Cpu(cpu) => cpu.within_range(addr)
        }
    }

    fn read(&self, addr: u16) -> u8 {
        match self {
            Device::Ram(ram) => ram.read(addr),
            Device::Cpu(cpu) => cpu.read(addr)
        }
    }

    fn write(&mut self, addr: u16, value: u8) -> () {
        match self {
            Device::Ram(ram) => ram.write(addr, value),
            Device::Cpu(cpu) => cpu.write(addr, value)
        }
    }
}