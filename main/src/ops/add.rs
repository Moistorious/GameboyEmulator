use crate::gameboy::Gameboy;
use crate::error::EmulatorError;

impl Gameboy {
    pub fn add(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        Err(EmulatorError::NotImplementedOpcode(opcode, self.cpu.program_counter))
    }
}
