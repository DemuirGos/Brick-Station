use crate::hardware::address_mode::AddressMode;
use crate::hardware::cpu::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Disassembler{
    pub program: Vec<String>,
    pub counters: HashMap<i32, usize> 
}

impl Disassembler {
    pub fn new() -> Disassembler {
        Disassembler {
            program: Vec::new(),
            counters: HashMap::new()
        }
    }

    pub fn disassemble(program: &Vec<u8>) -> Disassembler {
        let memory_region_start : i32 = 0x8000;
        let mut string_builder = Vec::new();
        let mut wires_builder = HashMap::new();
        let instruction_set = Cpu::setup_instruction_map();

        let mut i = 0;
        while i < program.len() {
            let byte = program[i];
            if let Some(instruction) = instruction_set.get(&byte) {
                wires_builder.insert(memory_region_start + (i as i32), string_builder.len());
                let mut instruction_string = String::new();
                instruction_string.push_str(format!("{}", instruction.mnemonic).as_str());
                instruction_string.push_str(" ");

                match instruction.address_mode {
                    AddressMode::Abs => {
                        instruction_string.push_str(format!("${:02X}{:02X}", program[1], program[2]).as_str());
                        i += 2;
                    },
                    AddressMode::Abx => {
                        instruction_string.push_str(format!("${:02X}{:02X}, X", program[1], program[2]).as_str());
                        i += 2;
                    },
                    AddressMode::Aby => {
                        instruction_string.push_str(format!("${:02X}{:02X}, Y", program[1], program[2]).as_str());
                        i += 2;
                    },
                    AddressMode::Imm => {
                        instruction_string.push_str(format!("#${:02X}", program[1]).as_str());
                        i += 1;
                    },
                    AddressMode::Imp => {
                        instruction_string.push_str("");
                    },
                    AddressMode::Ind => {
                        instruction_string.push_str(format!("(${:02X}{:02X})", program[1], program[2]).as_str());
                        i += 2;
                    },
                    AddressMode::Izx => {
                        instruction_string.push_str(format!("(${:02X}, X)", program[1]).as_str());
                        i += 1;
                    },
                    AddressMode::Izy => {
                        instruction_string.push_str(format!("(${:02X}), Y", program[1]).as_str());
                        i += 1;
                    },
                    AddressMode::Rel => {
                        instruction_string.push_str(format!("${:02X}", program[1]).as_str());
                        i += 1;
                    },
                    AddressMode::Zp0 => {
                        instruction_string.push_str(format!("${:02X}", program[1]).as_str());
                        i += 1;
                    },
                    AddressMode::Zpx => {
                        instruction_string.push_str(format!("${:02X}, X", program[1]).as_str());
                        i += 1;
                    },
                    AddressMode::Zpy => {
                        instruction_string.push_str(format!("${:02X}, Y", program[1]).as_str());
                        i += 1;
                    },
                }
                string_builder.push(instruction_string);
                i += 1;
            }
        }
        
        Disassembler {
            program: string_builder,
            counters: wires_builder
        }
    }
}

