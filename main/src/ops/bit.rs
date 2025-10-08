use crate::gameboy::Gameboy;
use crate::cpu::{Reg8, Reg16, Gbz80};

impl Gameboy {

    pub fn bit(&mut self, opcode: u8) {
        let source_register = Reg8::from_u8(opcode & 7);
        let source_value = self.cpu.reg8(source_register);
        let bit = (opcode >> 3) & 0x07;

        self.cpu.set_flags((source_value >> bit) & 0x01 == 0, false, true);
    }
}