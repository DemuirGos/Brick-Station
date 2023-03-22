use std::{ops::Range, cell::RefCell, rc::Rc};

use super::{interfaces::{DeviceOps}, ram::Ram, cpu::Cpu, device::Device};

pub struct Bus<'a> {
    pub processor: Option<Rc<RefCell<Cpu<'a>>>>,
    pub devices : Vec<Rc<RefCell<Device<'a>>>>
}

impl<'a> Bus<'a> {
    pub fn new() -> Bus<'a> {
        Bus {
            processor: None,
            devices: Vec::new()
        }
    }

    pub fn load_program(&mut self, program: Vec<u8>) {
        
    }

    pub fn connect_processor(&mut self, processor: Rc<RefCell<Cpu<'a>>>) -> () {
        self.processor = Some(processor);
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

    pub fn tick(&mut self) -> () {
        let mut self_processor = self.processor.as_ref().unwrap().borrow_mut();
        if(self_processor.cycle == 0) {
            self_processor.opcode = self_processor.read(self_processor.registers.pc as u16);
            self_processor.registers.pc += 1;

            let instruction_data = self_processor.instruction_set.get(&self_processor.opcode).unwrap().to_owned();

            self_processor.cycle = instruction_data.cycles as i32;
            
            let additional_cycles1 = instruction_data.address_mode.handle(&mut self_processor);
            let additional_cycles2 = instruction_data.operation(&mut self_processor);

            self_processor.cycle += (additional_cycles1 && additional_cycles2) as i32;
        }
        self_processor.cycle -= 1;
    }

    pub fn clone_state(&self) -> Rc<RefCell<Bus<'a>>> {
        let mut bus = Rc::new(RefCell::new(Bus::new()));
        let mut cpu = {
            let mut cpu = self.processor.as_ref().unwrap().borrow_mut();
            Rc::new(RefCell::new((*cpu).clone()))
        };

        for plugin in self.devices.iter() {
            let mut device = plugin.borrow_mut().to_owned();
            let mut device_clone = Rc::new(RefCell::new(device.clone()));
            bus.borrow_mut().add_device(device_clone);
        }

        bus.borrow_mut().connect_processor(cpu.clone());

        (*cpu).borrow_mut().bus = Some(Rc::downgrade(&bus));


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
