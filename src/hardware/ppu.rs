use std::{rc::Rc, cell::RefCell};

use super::{bus::Bus, interfaces::{DeviceOps, Originator}, cartridge::Cartridge};

#[derive(Clone)]
pub struct Vram {
    pub name_tables : [u8; 2048],
    pub palette     : [u8; 32],
}

#[derive(Clone)]
pub struct Ppu<'a> {
    pub bus       : Option<Rc<RefCell<Bus<'a>>>>,
    pub cartridge : Option<Rc<RefCell<Cartridge<'a>>>>,
    pub vram : Vram,
    pub cycle : i16,
    pub scanline : i16,
    pub frame : u16,
} 

impl<'a> Ppu<'a> {
    pub fn new() -> Ppu<'a> {
        Ppu {
            bus       : None,
            cartridge : None,
            cycle     : 0,
            scanline  : 0,
            frame     : 0,
            vram : Vram {
                name_tables : [0; 2048],
                palette     : [0; 32]
            },
        }
    }

    pub fn tick(&mut self) -> () {
        self.cycle += 1;
        if self.cycle >= 341
        {
            self.cycle = 0;
            self.scanline += 1;
            if (self.scanline >= 261)
            {
                self.scanline = -1;
                self.frame += 1;
            }
        }
    }
}

impl<'a> DeviceOps for Ppu<'a>
{
    fn read(&self, addr: u16, reader: Originator) -> u8 {
        match reader {
            Originator::Ppu => {
                let forward_addr = addr & 0x3FFF;
                0
            },
            Originator::Cpu => {
                match addr {
                    0 => {
                        todo!()
                    },
                    1 => {
                        todo!()
                    },
                    2 => {
                        todo!()
                    },
                    3 => {
                        todo!()
                    },
                    4 => {
                        todo!()
                    },
                    5 => {
                        todo!()
                    },
                    6 => {
                        todo!()
                    },
                    7 => {
                        todo!()
                    },
                    _ => 0
                }
            }
        }
    }

    fn write(&mut self, addr: u16, value: u8, writer: Originator) -> () {
        match writer {
            Originator::Ppu => {
                let forward_addr = addr & 0x3FFF;
                todo!()
            },
            Originator::Cpu => {
                match addr {
                    0 => {
                        todo!()
                    },
                    1 => {
                        todo!()
                    },
                    2 => {
                        todo!()
                    },
                    3 => {
                        todo!()
                    },
                    4 => {
                        todo!()
                    },
                    5 => {
                        todo!()
                    },
                    6 => {
                        todo!()
                    },
                    7 => {
                        todo!()
                    },
                    _ => ()
                }
            }
        }
    }

    fn within_range(&self, addr: u16) -> bool {
        addr >= 0x2000 && addr <= 0x3FFF
    }
} 