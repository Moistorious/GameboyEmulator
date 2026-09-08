use crate::gameboy::Gameboy;
use crate::error::EmulatorError;

impl Gameboy {
    pub fn get_alu_operand(&mut self, opcode: u8) -> Result<u8, EmulatorError> {
        if opcode < 0xC0 && opcode > 0x7F {
            match opcode & 0x07 {
                0 => Ok(self.cpu.b),
                1 => Ok(self.cpu.c),
                2 => Ok(self.cpu.d),
                3 => Ok(self.cpu.e),
                4 => Ok(self.cpu.h),
                5 => Ok(self.cpu.l),
                6 => Ok(self.read_u8_increment_pc()),
                7 => Ok(self.cpu.a),
                _ => Err(EmulatorError::InvalidOperand(opcode))
            }
        }else{
            Ok(0)
        }
    }
    //     let source_value = if opcode & 0xf == 0xE {
    //         // Value from pointer
    //         if opcode == 0xEE{
    //             self.read_u8_increment_pc()
    //         }else{
    //             self.memory.read_u8(self.cpu.reg16(Reg16::HL))
    //         }
    //     }else{
    //         self.cpu.reg8(Reg8::from_u8(opcode & 7))
    //     };
    // }

    pub fn xor(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        

        Err(EmulatorError::NotImplementedOpcode(opcode, self.cpu.program_counter))

        // let value = self.cpu.reg8(Reg8::A) ^ source_value;
        
        // self.cpu.set_flags(value == 0, false, false, false);

        // self.cpu.write_reg8(Reg8::A, value);
        // Ok(())
    }
}