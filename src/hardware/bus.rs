use std::{ops::Range, cell::RefCell, rc::Rc};

use super::{interfaces::{DeviceOps}, ram::Ram, cpu::Cpu, device::Device};

pub struct Bus<'a> {
    pub devices : Vec<Rc<RefCell<Device<'a>>>>
}

impl<'a> Bus<'a> {
    pub fn new() -> Bus<'a> {
        Bus {
            devices: Vec::new()
        }
    }

    pub fn load_program(&mut self, program: Vec<u8>) {
        
    }

    pub fn add_device(&mut self, device: Rc<RefCell<Device<'a>>>) -> usize {
        self.devices.push(device);
        self.devices.len()
    }

    pub fn remove_device(&mut self, at: usize) -> () {
        if at >= self.devices.len() {
            return;
        }
        self.devices.remove(at);
    }

    pub fn clone_state(&self) -> Rc<RefCell<Bus<'a>>> {
        let mut bus = Rc::new(RefCell::new(Bus::new()));

        for plugin in self.devices.iter() {
            let mut device = plugin.borrow_mut().to_owned();
            let mut device_clone = Rc::new(RefCell::new(device.clone()));
            bus.borrow_mut().add_device(device_clone);
        }

        bus
    }
}

impl DeviceOps for Bus<'_> {
    fn read(&self, addr: u16) -> u8 {
        self.devices.iter()
            .filter(|device| device.borrow().within_range(addr))
            .map(|device| device.borrow().read(addr))
            .nth(0).unwrap()
    }

    fn write(&mut self, addr: u16, value: u8) -> () {
        self.devices.iter_mut()
            .filter(|device| device.borrow().within_range(addr))
            .for_each(|device| device.borrow_mut().write(addr, value));
    }
}
