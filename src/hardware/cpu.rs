use std::{rc::Rc, collections::HashMap, fs::File, io::{BufReader, BufRead}};

use super::{
    registers::Registers, 
    instructions::Instructions, 
    address_mode::{
        AddressMode, 
        AddressingData,
    }, 
    interfaces::Device,
    bus::Bus
};

pub struct Cpu {
    pub registers : Registers,
    pub bus       : Option<Box<Bus>>,
    pub cycle     : i32,
    pub opcode    : u8,
    pub address_mode : AddressingData,
    pub instruction_set : HashMap<u8, Instructions>
} 

impl Cpu {
    pub fn new() -> Cpu {
        let mut new_cpu = Cpu {
            registers : Registers::new(),
            bus       : Option::None,
            cycle     : 0,
            opcode    : 0,
            address_mode : AddressingData::new(),
            instruction_set : HashMap::new(),
        };
        new_cpu.setup();
        new_cpu
    }

    fn reset(&mut self) -> () {
        self.registers = Registers::new();
        self.cycle = 0;
        self.opcode = 0;
    }

    fn interrupt(&self, isMaskable: bool) -> () {

    }

    fn fetch(&mut self) -> u8 {
        let opcode_metadata = self.instruction_set.get(&self.opcode);
        if let Some(metadata) = opcode_metadata {
            if metadata.address_mode == AddressMode::Imm {
                self.registers.fetched = self.read(self.address_mode.address_abs);
            }
        }
        self.registers.fetched
    }

    fn tick(&mut self) -> () {
        if(self.cycle == 0) {
            self.opcode = self.read(self.registers.pc as u16);
            self.registers.pc += 1;

            let instruction_data = self.instruction_set.clone();
            let instruction_data = instruction_data.get(&self.opcode).unwrap();

            self.cycle = instruction_data.cycles as i32;
            
            let additional_cycles1 = instruction_data.address_mode.handle(self);
            let additional_cycles2 = instruction_data.operation(self);

            self.cycle += (additional_cycles1 && additional_cycles2) as i32;
        }
        self.cycle -= 1;
    }
    
    
    fn connect_bus(&mut self, bus: Box<Bus>) -> () {
        self.bus = Option::Some(bus);
    }

    fn run(&mut self, program : &[u8]) -> () {
        self.reset();
        loop {
            self.tick();
        }
    }

    pub fn setup(&mut self) {
        let mut metadata_file = File::options()
            .read(true)
            .open("instructions.txt").unwrap();
    
        let reader = BufReader::new(metadata_file);
    
        reader.split(b'\n').for_each(|line| {
            let line = line.unwrap();
            let tokens = line.split(|&c| c == b',')
                .map(std::str::from_utf8)
                .map(|s| s.unwrap().trim())
                .collect::<Vec<&str>>();
            let instruction = Instructions::new(
                tokens[1].to_string(),
                tokens[0].parse::<u8>().unwrap(),
                tokens[3].parse::<u8>().unwrap(),
                AddressMode::from_str(tokens[2])
            );
            self.instruction_set.insert(instruction.opcode, instruction);
        });
    }
}

impl Device for Cpu {
    fn read(&self, addr : u16 ) -> u8 {
        self.bus.as_ref().unwrap().read(addr)
    }
    
    fn write(&mut self, addr : u16, data: u8) -> () {
        match self.bus {
            Some(ref mut bus) => bus.write(addr, data),
            None => panic!("Bus not connected"),
        }
    }
}