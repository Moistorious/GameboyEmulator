use crate::gameboy::Gameboy;
use crate::cpu::{Reg8, Reg16, Gbz80};

impl Gameboy {
    pub fn xor(&mut self, opcode: u8) {
        let source_value = if opcode & 0xf == 0xE {
            // Value from pointer
            if opcode == 0xEE{
                self.read_u8_increment_pc()
            }else{
                self.memory.read_u8(self.cpu.reg16(Reg16::HL))
            }
        }else{
            self.cpu.reg8(Reg8::from_u8(opcode & 7))
        };

        let value = self.cpu.reg8(Reg8::A) ^ source_value;
        
        self.cpu.set_flags(value == 0, false, false);

        self.cpu.write_reg8(Reg8::A, value);
    }
}