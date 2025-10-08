use crate::cpu::{Reg8, Reg16};
use crate::gameboy::Gameboy;

impl Gameboy {
    pub fn ld(&mut self, opcode: u8) {
        match opcode {
            // LD r,(HL)
            0x46 | 0x4E | 0x56 | 0x5E | 0x66 | 0x6E | 0x7E => self.ld_r_hl(opcode),
            // LD (HL),r
            0x70..=0x75 | 0x77 => self.ld_hl_r(opcode),
            // LD r,r
            0x40..=0x7F => self.ld_r_r(opcode),

            // LD r,n
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x3E => self.ld_r_n(opcode),

            // LD A,(rr)
            0x0A => self.ld_a_rr(Reg16::BC),
            0x1A => self.ld_a_rr(Reg16::DE),

            // LD (rr),A
            0x02 => self.ld_rr_a(Reg16::BC),
            0x12 => self.ld_rr_a(Reg16::DE),

            0x01 => self.ld_rr_nn(Reg16::BC),
            0x11 => self.ld_rr_nn(Reg16::DE),
            0x21 => self.ld_rr_nn(Reg16::HL),
            0x31 => self.ld_sp_nn(),

            // LD A,(nn)
            0xFA => self.ld_a_nn(),

            // LD (nn),A
            0xEA => self.ld_nn_a(),

            _ => self.not_implemented(opcode),
        }
    }

    fn ld_sp_nn(&mut self)
    {
        self.cpu.stack_pointer = self.read_u16_increment_pc();
    }
    
    fn ld_rr_nn(&mut self, rr: Reg16)
    {
        let imm = self.read_u16_increment_pc();
        self.cpu.write_reg16(rr, imm);
    }

    fn ld_r_r(&mut self, opcode: u8) {
        let dest = ((opcode >> 3) & 0x07) as u8;
        let src = (opcode & 0x07) as u8;

        if dest == 6 && src == 6 {
            // LD (HL),(HL) is actually HALT, not LD
            self.halt();
            return;
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
        }
    }

    fn ld_r_n(&mut self, opcode: u8) {
        let dest = ((opcode >> 3) & 0x07) as u8;
        let imm = self.read_u8_increment_pc();

        if dest == 6 {
            let addr = self.cpu.reg16(Reg16::HL);
            self.memory.write_u8(addr, imm);
        } else {
            self.cpu.write_reg8(Reg8::from_u8(dest), imm);
        }
    }

    fn ld_r_hl(&mut self, opcode: u8) {
        let dest = ((opcode >> 3) & 0x07) as u8;
        let addr = self.cpu.hl();
        let value = self.memory.read_u8(addr);

        self.cpu.write_reg8(Reg8::from_u8(dest), value);
    }

    fn ld_hl_r(&mut self, opcode: u8) {
        let src = (opcode & 0x07) as u8;
        let addr = self.cpu.reg16(Reg16::HL);

        let value = self.cpu.reg8(Reg8::from_u8(src));
        self.memory.write_u8(addr, value);
    }

    fn ld_a_rr(&mut self, rr: Reg16) {
        let addr = self.cpu.reg16(rr);
        let value = self.memory.read_u8(addr);
        self.cpu.write_reg8(Reg8::A, value);
    }

    fn ld_rr_a(&mut self, rr: Reg16) {
        let addr = self.cpu.reg16(rr);
        let value = self.cpu.reg8(Reg8::A);
        self.memory.write_u8(addr, value);
    }

    fn ld_a_nn(&mut self) {
        let addr = self.read_u16_increment_pc();
        let value = self.memory.read_u8(addr);
        self.cpu.write_reg8(Reg8::A, value);
    }

    fn ld_nn_a(&mut self) {
        let addr = self.read_u16_increment_pc();
        let value = self.cpu.reg8(Reg8::A);
        self.memory.write_u8(addr, value);
    }
}
