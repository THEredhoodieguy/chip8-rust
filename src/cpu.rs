use crate::ram::Ram;

pub const PROGRAM_START : u16 = 0x200;

pub struct CPU {
    registers: [u8; 16],
    i: u16,
    sound: u8,
    delay: u8,
    pc: u16,
    sp: u8,
    stack: [u16; 16],
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: [0; 16],
            i: 0,
            sound: 0,
            delay: 0,
            pc: PROGRAM_START,
            sp: 0,
            stack: [0; 16],
        }
        
    }

    pub fn run_instruction(&mut self, ram: &mut Ram) {

    }
}