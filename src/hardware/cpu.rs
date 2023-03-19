use std::{collections::HashMap, fs::File, io::{BufReader, BufRead}};

use super::{
    registers::{Registers, Flag}, 
    instructions::Instructions, 
    address_mode::{
        AddressMode, 
        AddressingData,
    }, 
    interfaces::Device,
    bus::Bus, opcodes::Opcode
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

    pub fn reset(&mut self) -> () {
        self.registers = Registers::new();
        self.address_mode = AddressingData::new();
        self.cycle = 8;
        self.opcode = 0;
        self.registers.sp = 0xFD;
        self.registers.set_flag(Flag::U, true);
        self.address_mode.address_abs = 0xFFFC;
        let hi = self.read(self.address_mode.address_abs + 1) as u16;
        let lo =  self.read(self.address_mode.address_abs) as u16;
        self.registers.pc = hi << 8 + lo;
    }

    pub fn interrupt(&mut self, is_non_maskable: bool) -> () {
        if(!self.registers.get_flag(Flag::I) || is_non_maskable) {
            self.write(0x0100 + self.registers.sp as u16 + 0 , (self.registers.pc >> 8) as u8);
            self.write(0x0100 + self.registers.sp as u16 - 1 , self.registers.pc  as u8);
            self.registers.sp -= 2;
            
            self.registers.set_flag(Flag::B, false);
            self.registers.set_flag(Flag::U, true);
            self.registers.set_flag(Flag::I, true);
            
            self.write(0x0100 + self.registers.sp as u16 , self.registers.status  as u8);
            self.registers.sp -= 1;

            self.address_mode.address_abs = if is_non_maskable { 0xFFFA } else { 0xFFFE };
            let lo = self.read(self.address_mode.address_abs + 0) as u16;
            let hi = self.read(self.address_mode.address_abs + 1) as u16;
            self.registers.pc = (hi << 8) + lo;

            self.cycle = if is_non_maskable { 8 } else { 7 } ;
        }
    }

    pub fn fetch(&mut self) -> u8 {
        let opcode_metadata = self.instruction_set.get(&self.opcode);
        if let Some(metadata) = opcode_metadata {
            if metadata.address_mode == AddressMode::Imm {
                self.registers.fetched = self.read(self.address_mode.address_abs);
            }
        }
        self.registers.fetched
    }

    pub fn tick(&mut self) -> () {
        if(self.cycle == 0) {
            self.opcode = self.read(self.registers.pc as u16);
            self.registers.pc += 1;

            let instruction_data = self.instruction_set.get(&self.opcode).unwrap().to_owned();

            self.cycle = instruction_data.cycles as i32;
            
            let additional_cycles1 = instruction_data.address_mode.handle(self);
            let additional_cycles2 = instruction_data.operation(self);

            self.cycle += (additional_cycles1 && additional_cycles2) as i32;
        }
        self.cycle -= 1;
    }
    
    
    pub fn connect_bus(&mut self, bus: Box<Bus>) -> () {
        self.bus = Option::Some(bus);
    }

    pub fn setup(&mut self) {
        let mut metadata_file: File = File::options()
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
                Opcode::from_str(tokens[1]),
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