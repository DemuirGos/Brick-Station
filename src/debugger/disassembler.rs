use crate::hardware::interfaces::DeviceOps;
use crate::hardware::bus::*;
use crate::hardware::cpu::*;
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
        string_builder.push("test".to_string());
        string_builder
    }
}

