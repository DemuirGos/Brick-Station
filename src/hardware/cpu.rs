use super::{
    registers::Registers, 
    instructions::Metadata, 
    address_mode::{
        Address_Mode, 
        Addressing_Data,
    }, 
    interfaces::Device,
    bus::Bus
};

pub struct Cpu {
    pub registers : Registers,
    pub bus       : Bus,
    pub cycle     : i32,
    pub opcode    : u8,
    pub address_mode : Addressing_Data
} 

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            registers : Registers::new(),
            cycle : 0,
            address_mode : Addressing_Data::new(),
            opcode : 0,
            bus : Bus::new()
        }
    }

    fn reset(&self) -> () {
        self.registers = Registers::new();
        self.cycle = 0;
        self.opcode = 0;
    }

    fn interrupt(&self, isMaskable: bool) -> () {

    }

    fn fetch(&self) -> u8 {
        let opcode_metadata = Metadata[self.opcode];
        if(opcode_metadata.address_mode != Address_Mode::Imp) {
            self.registers.fetched = self.read(self.address_mode.address_abs);
        }
        self.registers.fetched
    }

    fn tick(&self) -> () {
        if(self.cycle == 0) {
            self.opcode = self.read(self.registers.pc as u16);
            self.registers.pc += 1;

            let instruction_data = Metadata[self.opcode];

            self.cycle = instruction_data.Cycles;
            let additional_cycles1 = self.address_mode.handle(instruction_data.address_mode, self);
            let additional_cycles2 = instruction_data.operation(self);
            self.cycle += (additional_cycles1 && additional_cycles2) as i32;
        }
        self.cycle -= 1;
    }
    
    
    fn connect_bus(&self, bus: &mut Bus) -> () {
        self.bus = bus;
    }

    fn run(&self, program : &[u8]) -> () {
        self.reset();
        loop {
            self.tick();
        }
    }

}

impl Device for Cpu {
    fn read(&self, addr : u16 ) -> u8 {
        self.bus.read(addr)
    }
    
    fn write(&self, addr : u16, data: u8) -> () {
        self.bus.write(addr, data)
    }

    fn withinRange(&self, addr: u16) -> bool {
        true
    }
}