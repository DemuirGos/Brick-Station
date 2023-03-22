use crate::hardware::address_mode::AddressMode;
use crate::hardware::interfaces::DeviceOps;
use crate::hardware::bus::*;
use crate::hardware::cpu::*;
use crate::hardware::opcodes::Opcode;
use crate::hardware::ram::*;

use std::borrow::Borrow;
use std::cell;
use std::cell::RefCell;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::rc::Rc;
use std::rc::Weak;
use std::{io, io::Error};

pub struct Disassembler();

impl Disassembler {
    pub fn disassemble(program: &Vec<u8>) -> Vec<String> {
        let mut string_builder = vec![];
        let instruction_set = Cpu::setup_instruction_map();

        for byte in  program {
            if let Some(instruction) = instruction_set.get(byte) {
                let mut instruction_string = String::new();
                instruction_string.push_str(format!("{}", instruction.mnemonic).as_str());
                instruction_string.push_str(" ");

                match instruction.address_mode {
                    AddressMode::Abs => {
                        instruction_string.push_str(format!("${:02X}{:02X}", program[1], program[2]).as_str());
                    },
                    AddressMode::Abx => {
                        instruction_string.push_str(format!("${:02X}{:02X}, X", program[1], program[2]).as_str());
                    },
                    AddressMode::Aby => {
                        instruction_string.push_str(format!("${:02X}{:02X}, Y", program[1], program[2]).as_str());
                    },
                    AddressMode::Imm => {
                        instruction_string.push_str(format!("#${:02X}", program[1]).as_str());
                    },
                    AddressMode::Imp => {
                        instruction_string.push_str("");
                    },
                    AddressMode::Ind => {
                        instruction_string.push_str(format!("(${:02X}{:02X})", program[1], program[2]).as_str());
                    },
                    AddressMode::Izx => {
                        instruction_string.push_str(format!("(${:02X}, X)", program[1]).as_str());
                    },
                    AddressMode::Izy => {
                        instruction_string.push_str(format!("(${:02X}), Y", program[1]).as_str());
                    },
                    AddressMode::Rel => {
                        instruction_string.push_str(format!("${:02X}", program[1]).as_str());
                    },
                    AddressMode::Zp0 => {
                        instruction_string.push_str(format!("${:02X}", program[1]).as_str());
                    },
                    AddressMode::Zpx => {
                        instruction_string.push_str(format!("${:02X}, X", program[1]).as_str());
                    },
                    AddressMode::Zpy => {
                        instruction_string.push_str(format!("${:02X}, Y", program[1]).as_str());
                    },
                }
                string_builder.push(instruction_string);
            }
        }
        string_builder
    }
}

