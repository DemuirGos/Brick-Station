use std::{cell::RefCell, rc::Rc};

use super::{bus::Bus, cartridge, cpu::Cpu, device::Device, ram::Ram, interfaces::{DeviceOps, Originator}, ppu::Ppu};

pub struct State<'a> {
    pub bus: Rc<RefCell<Bus<'a>>>,
    pub cpu: Rc<RefCell<Cpu<'a>>>,
    pub ppu: Rc<RefCell<Ppu<'a>>>,
    pub cartridge: Rc<RefCell<cartridge::Cartridge<'a>>>,
}

impl<'a> State<'a> {
    pub fn new() -> State<'a> {
        let ram = Rc::new(RefCell::new(Device::Ram(Ram::new())));
        let bus = Rc::new(RefCell::new(Bus::new()));
        let cpu = Rc::new(RefCell::new(Cpu::new()));
        let ppu = Rc::new(RefCell::new(Ppu::new()));
        let cartridge = Rc::new(RefCell::new(cartridge::Cartridge::new(16, 16)));

        bus.borrow_mut().add_device(ram.clone());

        (*cpu).borrow_mut().bus = Some(bus.clone());
        (*cartridge).borrow_mut().bus = Some(bus.clone());
        (*ppu).borrow_mut().bus = Some(bus.clone());
        (*ppu).borrow_mut().cartridge = Some(cartridge.clone());


        let state = Rc::new(RefCell::new(State {
            bus : Rc::clone(&bus),
            cpu : Rc::clone(&cpu),
            ppu : Rc::clone(&ppu),
            cartridge : Rc::clone(&cartridge),
        }));

        

        (*bus).borrow_mut().write(0x1FFC, 0x00, Originator::Cpu);
        (*bus).borrow_mut().write(0x1FFD, 0x01, Originator::Cpu);
        (*cpu).borrow_mut().reset();

        State {
            bus,
            cpu,
            ppu,
            cartridge
        }
    }
}

impl<'a> Clone for State<'a> {
    fn clone(&self) -> Self {
        let cpu_ref = (*self.cpu).borrow();
        let cartridge_ref = (*self.cartridge).borrow();
        let ppu_ref = (*self.ppu).borrow();
        
        
        
        let cloned_state = State {
            cpu: Rc::new(RefCell::new((*cpu_ref).clone())),
            bus: self.bus.borrow().clone_state(),
            cartridge: Rc::new(RefCell::new((*cartridge_ref).clone())),
            ppu: Rc::new(RefCell::new((*ppu_ref).clone())),
        };

        let bus_ref = cloned_state.bus.clone();
        (*cloned_state.cpu).borrow_mut().bus = Some(bus_ref.clone());
        (*cloned_state.cartridge).borrow_mut().bus = Some(bus_ref.clone());
        (*cloned_state.ppu).borrow_mut().bus = Some(bus_ref.clone());
        (*cloned_state.ppu).borrow_mut().cartridge = Some(cloned_state.cartridge.clone());

        cloned_state
    }
} 