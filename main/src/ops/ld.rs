use crate::cpu::{Reg8, Reg16};
use crate::gameboy::Gameboy;
use crate::error::EmulatorError;

impl Gameboy {
    pub fn ld(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        match opcode {
            // LD r,(HL)
            0x46 | 0x4E | 0x56 | 0x5E | 0x66 | 0x6E | 0x7E => self.ld_r_hl(opcode),
            // LD (HL),r
            0x70..=0x75 | 0x77 => self.ld_hl_r(opcode),
            // LD r,r
            0x40..=0x7F => self.ld_r_r(opcode),

            // LD r,n
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x3E | 0x36 => self.ld_r_n(opcode),

            // LD A,(rr)
            0x0A => self.ld_a_rr(Reg16::BC),
            0x1A => self.ld_a_rr(Reg16::DE),
            0x2A => self.ld_a_hli(),
            0x3A => self.ld_a_hld(),
            0xF2 => self.ld_a_c(),

            // LD (rr),A
            0x02 => self.ld_rr_a(Reg16::BC),
            0x12 => self.ld_rr_a(Reg16::DE),
            0x22 => self.ld_hli(Reg16::HL),
            0x32 => self.ld_hld(Reg16::HL),
            0xE2 => self.ld_c_a(),

            0x01 => self.ld_rr_nn(Reg16::BC),
            0x11 => self.ld_rr_nn(Reg16::DE),
            0x21 => self.ld_rr_nn(Reg16::HL),
            0x31 => self.ld_sp_nn(),
            0xF9 => self.ld_sp_hl(),
            0xF8 => self.ld_hl_sp_e8(),
            0x08 => self.ld_nn_sp(),

            // LD A,(nn)
            0xFA => self.ld_a_nn(),

            // LD (nn),A
            0xEA => self.ld_nn_a(),

            _ => Err(EmulatorError::NotImplementedOpcode(opcode, self.cpu.program_counter)),
        }
    }

    fn ld_sp_nn(&mut self) -> Result<(), EmulatorError> {
        self.cpu.stack_pointer = self.read_u16_increment_pc();
        Ok(())
    }

    fn ld_rr_nn(&mut self, rr: Reg16) -> Result<(), EmulatorError> {
        let imm = self.read_u16_increment_pc();
        self.cpu.write_reg16(rr, imm);
        Ok(())
    }

    fn ld_r_r(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        let (_, dest, src) = self.decode_opcode(opcode);

        if dest == 6 && src == 6 {
            // LD (HL),(HL) is actually HALT, not LD
            return self.halt();
        }

        let value = if src == 6 {
            let addr = self.cpu.reg16(Reg16::HL);
            self.memory.read_u8(addr)
        } else {
            self.cpu.reg8(Reg8::from_u8(src))
        };

        if dest == 6 {
            let addr = self.cpu.reg16(Reg16::HL);
            self.memory.write_u8(addr, value);
        } else {
            self.cpu.write_reg8(Reg8::from_u8(dest), value);
        };
        Ok(())
    }

    fn ld_r_n(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        let (_, dest, _) = self.decode_opcode(opcode);
        let imm = self.read_u8_increment_pc();

        if dest == 6 {
            self.memory.write_u8(self.cpu.reg16(Reg16::HL), imm);
        } else {
            self.cpu.write_reg8(Reg8::from_u8(dest), imm);
        };
        Ok(())
    }

    fn ld_r_hl(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        // opcode format: 01ddd110, where ddd is the destination register
        let (_, dest, _) = self.decode_opcode(opcode);
        self.cpu.write_reg8(
            Reg8::from_u8(dest), 
            self.memory.read_u8(self.cpu.hl()));
        Ok(())
    }

    fn ld_hl_r(&mut self, opcode: u8) -> Result<(), EmulatorError> {
        let (_, _, source) = self.decode_opcode(opcode);
        self.memory.write_u8(
            self.cpu.reg16(Reg16::HL),
            self.cpu.reg8(Reg8::from_u8(source)),
        );
        Ok(())
    }

    fn ld_a_rr(&mut self, rr: Reg16) -> Result<(), EmulatorError> {
        let addr = self.cpu.reg16(rr);
        let value = self.memory.read_u8(addr);
        self.cpu.write_reg8(Reg8::A, value);
        Ok(())
    }

    fn ld_rr_a(&mut self, rr: Reg16) -> Result<(), EmulatorError> {
        let addr = self.cpu.reg16(rr);
        let value = self.cpu.reg8(Reg8::A);
        self.memory.write_u8(addr, value);
        Ok(())
    }
    fn ld_hli(&mut self, rr: Reg16) -> Result<(), EmulatorError> {
        let addr = self.cpu.reg16(rr);
        let value = self.cpu.reg8(Reg8::A);
        self.cpu.write_reg16(rr, self.cpu.reg16(rr).wrapping_add(1));
        self.memory.write_u8(addr, value);
        Ok(())
    }

    fn ld_hld(&mut self, rr: Reg16) -> Result<(), EmulatorError> {
        let addr = self.cpu.reg16(rr);
        let value = self.cpu.reg8(Reg8::A);
        self.cpu.write_reg16(rr, self.cpu.reg16(rr).wrapping_sub(1));
        self.memory.write_u8(addr, value);
        Ok(())
    }

    fn ld_a_nn(&mut self) -> Result<(), EmulatorError> {
        let addr = self.read_u16_increment_pc();
        let value = self.memory.read_u8(addr);
        self.cpu.write_reg8(Reg8::A, value);
        Ok(())
    }

    fn ld_nn_a(&mut self) -> Result<(), EmulatorError> {
        let addr = self.read_u16_increment_pc();
        let value = self.cpu.reg8(Reg8::A);
        self.memory.write_u8(addr, value);
        Ok(())
    }

    fn ld_a_hli(&mut self) -> Result<(), EmulatorError> {
        let addr = self.cpu.hl();
        self.cpu.set_hl(addr.wrapping_add(1));
        self.cpu.write_reg8(Reg8::A, self.memory.read_u8(addr));
        Ok(())
    }

    fn ld_a_hld(&mut self) -> Result<(), EmulatorError> {
        let addr = self.cpu.hl();
        self.cpu.set_hl(addr.wrapping_sub(1));
        self.cpu.write_reg8(Reg8::A, self.memory.read_u8(addr));
        Ok(())
    }
    fn ld_a_c(&mut self) -> Result<(), EmulatorError> {
        let addr: u16 = 0xFF00 + self.cpu.reg8(Reg8::C) as u16;
        self.cpu.write_reg8(Reg8::A, self.memory.read_u8(addr));
        Ok(())
    }

    fn ld_c_a(&mut self) -> Result<(), EmulatorError> {
        let addr: u16 = 0xFF00 + self.cpu.reg8(Reg8::C) as u16;
        self.memory.write_u8(addr, self.cpu.reg8(Reg8::A));
        Ok(())
    }

    fn ld_sp_hl(&mut self) -> Result<(), EmulatorError> {
        self.cpu.stack_pointer = self.cpu.hl();
        Ok(())
    }

    fn ld_hl_sp_e8(&mut self) -> Result<(), EmulatorError> {
        let imm = self.read_u8_increment_pc();
        let sign_shifted = imm as i8 as i16 as u16;

        let (z,n,h,c) = self.cpu.flags_from_add(imm, (self.cpu.stack_pointer & 0xFF) as u8);

        let addr: u16 = self.cpu.stack_pointer.wrapping_add(sign_shifted);
        self.cpu.set_hl(addr);
        self.cpu.set_flags(z,n,h,c);
        
        Ok(())
    }

    fn ld_nn_sp(&mut self) -> Result<(), EmulatorError> {
        let imm = self.read_u16_increment_pc();
        self.memory.write_u16(imm, self.cpu.stack_pointer);
        Ok(())
    }
}
