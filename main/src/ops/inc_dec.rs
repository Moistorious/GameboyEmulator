use crate::cpu::{Reg8};
use crate::gameboy::Gameboy;
use crate::error::EmulatorError;

impl Gameboy {
    //fn get_inc_dec_reg
    pub fn inc(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        // let reg = match opcode & 0xF0 {
        //     0x40 => Reg8::B,
        //     0x50 => Reg8::D,
        //     0x60 => Reg8::H
        // };
        // Ok(())
        Err(EmulatorError::NotImplementedOpcode(opcode, self.cpu.program_counter))
    }

    pub fn dec(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        Err(EmulatorError::NotImplementedOpcode(opcode, self.cpu.program_counter))
    }
}
