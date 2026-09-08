use crate::gameboy::Gameboy;
use crate::error::EmulatorError;

impl Gameboy {
    pub fn jp(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        Err(EmulatorError::NotImplementedOpcode(opcode, self.cpu.program_counter))
    }

    pub fn jr(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        Err(EmulatorError::NotImplementedOpcode(opcode, self.cpu.program_counter))
    }

    pub fn call(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        Err(EmulatorError::NotImplementedOpcode(opcode, self.cpu.program_counter))
    }

    pub fn ret(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        Err(EmulatorError::NotImplementedOpcode(opcode, self.cpu.program_counter))
    }

    pub fn reti(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        Err(EmulatorError::NotImplementedOpcode(opcode, self.cpu.program_counter))
    }

    pub fn rst(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        Err(EmulatorError::NotImplementedOpcode(opcode, self.cpu.program_counter))
    }
}
