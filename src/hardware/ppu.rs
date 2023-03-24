use std::{rc::Rc, cell::RefCell};

use super::{bus::Bus, interfaces::DeviceOps, cartridge::Cartridge};

#[derive(Clone)]
pub struct Vram {
    pub name_tables : [u8; 2048],
    pub palette     : [u8; 32],
}

#[derive(Clone)]
pub struct Ppu<'a> {
    pub bus       : Option<Rc<RefCell<Bus<'a>>>>,
    pub cartridge : Option<Rc<RefCell<Cartridge<'a>>>>,
    pub vram : Vram
} 

impl<'a> DeviceOps for Ppu<'a>
{
    fn read(&self, addr: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, addr: u16, value: u8) -> () {
        todo!()
    }

    fn within_range(&self, addr: u16) -> bool {
        addr >= 0x2000 && addr <= 0x3FFF
    }
} 