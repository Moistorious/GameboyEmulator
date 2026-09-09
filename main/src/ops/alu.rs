use crate::cpu::{AluOp, Reg8, Reg16};
use crate::error::EmulatorError;
use crate::gameboy::Gameboy;

impl Gameboy {
    fn alu_op<F>(&mut self, value: u8, op: AluOp, f: F)
    where
        F: Fn(u8, u8) -> u8,
    {
        let a_before = self.cpu.a;
        let result = f(self.cpu.a, value);
        if let AluOp::Cp = op {
            self.cpu.set_flags(result == 0, false, false, false);
        } else {
            self.cpu.a = result;
            // self.cpu.set_flags()
        }
    }

    pub fn get_alu_operand(&mut self, opcode: u8) -> u8 {
        
        if opcode < 0xC0 && opcode > 0x7F {
            match opcode & 0x07 {
                0 => self.cpu.b,
                1 => self.cpu.c,
                2 => self.cpu.d,
                3 => self.cpu.e,
                4 => self.cpu.h,
                5 => self.cpu.l,
                6 => self.memory.read_u8(self.cpu.hl()),
                7 => self.cpu.a,
                _ => 0 // Can't happen since we're ANDing with 7
            }
        } else {
            self.read_u8_increment_pc() // returns the next byte
        }
    }

    pub fn add(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        let val = self.get_alu_operand(opcode);
        println!("{val}");
        let (z,n,h,c) = self.cpu.flags_from_add(val, self.cpu.a);

        self.cpu.a = self.cpu.a.wrapping_add(val);

        self.cpu.set_flags(z,n,h,c);
        return Ok(());
    }

    pub fn sbc(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        Err(EmulatorError::NotImplementedOpcode(
            opcode,
            self.cpu.program_counter,
        ))
    }
    pub fn sub(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        Err(EmulatorError::NotImplementedOpcode(
            opcode,
            self.cpu.program_counter,
        ))
    }
    pub fn and(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        Err(EmulatorError::NotImplementedOpcode(
            opcode,
            self.cpu.program_counter,
        ))
    }
    pub fn adc(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        Err(EmulatorError::NotImplementedOpcode(
            opcode,
            self.cpu.program_counter,
        ))
    }
    pub fn or(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        let source_value = if opcode & 0xf == 0xE {
            // Value from pointer
            if opcode == 0xEE {
                self.read_u8_increment_pc()
            } else {
                self.memory.read_u8(self.cpu.reg16(Reg16::HL))
            }
        } else {
            self.cpu.reg8(Reg8::from_u8(opcode & 7))
        };

        let value = self.cpu.reg8(Reg8::A) | source_value;

        self.cpu.set_flags(value == 0, false, false, false);

        self.cpu.write_reg8(Reg8::A, value);
        Ok(())
    }
    pub fn xor(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        Err(EmulatorError::NotImplementedOpcode(
            opcode,
            self.cpu.program_counter,
        ))

        // let value = self.cpu.reg8(Reg8::A) ^ source_value;

        // self.cpu.set_flags(value == 0, false, false, false);

        // self.cpu.write_reg8(Reg8::A, value);
        // Ok(())
    }
}
