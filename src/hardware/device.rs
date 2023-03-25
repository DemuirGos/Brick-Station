use super::{interfaces::{DeviceOps, Originator}, ram::Ram, cpu::Cpu, ppu::Ppu, cartridge::Cartridge};

#[derive(Clone)]
pub enum Device<'a> {
    Ram(Ram),
    Cpu(Cpu<'a>),
    Ppu(Ppu<'a>),
    Cartridge(Cartridge<'a>)
}

impl<'a> DeviceOps for Device<'a> {
    fn within_range(&self, addr: u16) -> bool {
        match self {
            Device::Ram(ram) => ram.within_range(addr),
            Device::Cpu(cpu) => cpu.within_range(addr),
            Device::Ppu(ppu) => ppu.within_range(addr),
            Device::Cartridge(cartridge) => cartridge.within_range(addr)
        }
    }

    fn read(&self, addr: u16, reader: Originator) -> u8 {
        match self {
            Device::Ram(ram) => ram.read(addr, reader),
            Device::Cpu(cpu) => cpu.read(addr, reader),
            Device::Ppu(ppu) => ppu.read(addr, reader),
            Device::Cartridge(cartridge) => cartridge.read(addr, reader)
        }
    }

    fn write(&mut self, addr: u16, value: u8, reader: Originator) -> () {
        match self {
            Device::Ram(ram) => ram.write(addr, value, reader),
            Device::Cpu(cpu) => cpu.write(addr, value, reader),
            Device::Ppu(ppu) => ppu.write(addr, value, reader),
            Device::Cartridge(cartridge) => cartridge.write(addr, value, reader)
        }
    }
}