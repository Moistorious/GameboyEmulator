use crate::gameboy::Gameboy;
use crate::error::EmulatorError;

impl Gameboy {
    pub fn rlc(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        Err(EmulatorError::NotImplementedOpcode(opcode, self.cpu.program_counter))
    }

    pub fn rrc(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        Err(EmulatorError::NotImplementedOpcode(opcode, self.cpu.program_counter))
    }

    pub fn rl(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        Err(EmulatorError::NotImplementedOpcode(opcode, self.cpu.program_counter))
    }

    pub fn rr(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        Err(EmulatorError::NotImplementedOpcode(opcode, self.cpu.program_counter))
    }

    pub fn sla(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        Err(EmulatorError::NotImplementedOpcode(opcode, self.cpu.program_counter))
    }

    pub fn sra(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        Err(EmulatorError::NotImplementedOpcode(opcode, self.cpu.program_counter))
    }

    pub fn swap(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        Err(EmulatorError::NotImplementedOpcode(opcode, self.cpu.program_counter))
    }

    pub fn srl(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        Err(EmulatorError::NotImplementedOpcode(opcode, self.cpu.program_counter))
    }
}
