
pub struct Cpu {
    registers : Registers,
    bus       : &Port,
    cycle     : i32,
    opcode    : u8,
    address_mode : Addressing_Data
} 

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            registers : Registers::new(),
            cycle : 0,
            address_mode : Addressing_Data::new()
        }
    }

    fn reset(&self) -> unit {
        self.registers = Registers::new();
        self.cycle = 0;
        self.opcode = 0;
    }

    fn interrupt(&self, isMaskable: bool) -> unit {

    }

    fn fetch(&self) -> u8 {
        let opcode_metadata = Metadata[self.Opcode];
        if(opcode_metadata.address_mode != Address_Mode::Imp) {
            self.registers.fetched = self.read(self.address_mode.address_abs);
        }
        self.registers.fetched
    }

    fn tick(&self) -> unit {
        if(self.cycle == 0) {
            self.opcode = self.read(self.registers.pc);
            self.registers.pc += 1;

            let instruction_data = Metadata[self.Opcode];

            self.Cycles = instruction_data.Cycles;
            let additional_cycles1 = self.address_mode.handle(instruction_data.address_mode, self);
            let additional_cycles2 = instruction_data.operation(self);
            self.cycles += additional_cycles1 && additional_cycles2 as i32;
        }
        self.cycle -= 1;
    }
    
    
    fn connect_bus(&self, bus: &Bus) -> unit {
        self.bus = bus;
    }

    fn run(&self, program : &[u8]) -> unit {
        self.reset();
        loop {
            self.tick();
        }
    }

}

impl Device for Cpu {
    fn read(&self, addr : u8 ) -> u8 {
        self.bus.read(addr)
    }
    
    fn write(&self, addr : u8, data: u8) -> unit {
        self.bus.write(addr, data)
    }
}