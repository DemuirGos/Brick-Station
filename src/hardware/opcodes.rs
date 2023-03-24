use std::{fmt::Display};


#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    ADC, // Add register $0 with $1 and store result in $0 with carry  
    SBC, // Subtract register $0 with $1 and store result in $0 with carry 
    AND, // AND register $0 with $1 and store result in $0 
    OR, // OR register $0 with $1 and store result in $0 
    SL, // Arithmetic shift left
    SR, // Arithmetic shift right 
    ROL, // Rotate left 
    ROR, // Rotate right 
    BRC, // Branch on register clear
    BRS, // Branch on register set
    CMP, // Compare with Register $0 with register $1
    DEC, // Decrement register $0
    INC, // Increment register $0
    EOR, // Exclusive OR register $0 with register $1 and store result in $0
    JMP, // Jump to address 
    LDR, // Load register local with value $1
    NOP, // No operation
    MOV, // Move register $0 to register $1
    PLR, // Pull register from stack
    PHR, // Push register to stack
    STR, // Store register in memory
    BRK, // Break
    JSR, // Jump to subroutine
    RTS, // Return from subroutine
    RTI, // Return from interrupt    
}

impl Opcode {
    pub fn from_str(word: &str) -> Opcode {
        match word {
            "ADC" => Opcode::ADC,
            "SBC" => Opcode::SBC,
            "AND" => Opcode::AND,
            "OR" => Opcode::OR,
            "SL" => Opcode::SL,
            "SR" => Opcode::SR,
            "ROL" => Opcode::ROL,
            "ROR" => Opcode::ROR,
            "BRC" => Opcode::BRC,
            "BRS" => Opcode::BRS,
            "CMP" => Opcode::CMP,
            "DEC" => Opcode::DEC,
            "INC" => Opcode::INC,
            "EOR" => Opcode::EOR,
            "JMP" => Opcode::JMP,
            "LDR" => Opcode::LDR,
            "NOP" => Opcode::NOP,
            "MOV" => Opcode::MOV,
            "PLR" => Opcode::PLR,
            "PHR" => Opcode::PHR,
            "STR" => Opcode::STR,
            "BRK" => Opcode::BRK,
            "JSR" => Opcode::JSR,
            "RTS" => Opcode::RTS,
            "RTI" => Opcode::RTI,
            _ => panic!("Invalid opcode: {}", word),
        }
    }
}

impl Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "{:?}", self)
        }
    }
}