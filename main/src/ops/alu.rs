use crate::cpu::Gbz80;
use crate::gameboy::Gameboy;

impl Gameboy {
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
                _ => unreachable!()
            }
        } else {
            self.read_u8_increment_pc() // returns the next byte
        }
    }

    pub fn add_16(&mut self, opcode: u8) {
        let source = match opcode >> 4 {
            0 => self.cpu.bc(),
            1 => self.cpu.de(),
            2 => self.cpu.hl(),
            3=> self.cpu.stack_pointer,
            _ => unreachable!()
        };
        let (z,n,h,c) = self.cpu.flags_from_16bit_add(source, self.cpu.hl());

        self.cpu.set_hl(self.cpu.hl().wrapping_add(source));

        self.cpu.set_flags(z,n,h,c);
    }

    pub fn add_sp(&mut self) {
        let imm = self.read_u8_increment_pc();

        // H/C use the LOW byte of SP and the UNSIGNED immediate.
        let low = (self.cpu.stack_pointer & 0xFF) as u8;
        let (_, _, h, c) = self.cpu.flags_from_add(low, imm);

        // Sign-extend the immediate only for the actual addition.
        let offset = (imm as i8) as i16 as u16;
        self.cpu.stack_pointer = self.cpu.stack_pointer.wrapping_add(offset);

        self.cpu.set_flags(false, false, h, c); // Z = 0, N = 0
    }

    pub fn add(&mut self, opcode: u8) {
        if opcode < 0x80 {
            self.add_16(opcode);
            return;
        } else if opcode == 0xE8 {
            self.add_sp();
            return;
        }
        let val = self.get_alu_operand(opcode);
        let (z,n,h,c) = self.cpu.flags_from_add(val, self.cpu.a);

        self.cpu.a = self.cpu.a.wrapping_add(val);

        self.cpu.set_flags(z,n,h,c);
    }

    pub fn adc(&mut self, opcode: u8) {
        let val = self.get_alu_operand(opcode);
        let (z,n,h,c) = self.cpu.flags_from_adc(val, self.cpu.a);

        self.cpu.a = self.cpu.a.wrapping_add(val).wrapping_add(self.cpu.get_flag(Gbz80::FLAG_C) as u8);

        self.cpu.set_flags(z,n,h,c);
    }

    pub fn sub(&mut self, opcode: u8) {
        let val = self.get_alu_operand(opcode);
        let (z,n,h,c) = self.cpu.flags_from_sub(self.cpu.a, val);

        self.cpu.a = self.cpu.a.wrapping_sub(val);

        self.cpu.set_flags(z,n,h,c);
    }

    pub fn cp(&mut self, opcode: u8) {
        let val = self.get_alu_operand(opcode);
        let (z,n,h,c) = self.cpu.flags_from_sub(self.cpu.a, val);
        self.cpu.set_flags(z,n,h,c);
    }

    pub fn sbc(&mut self, opcode: u8) {
        let val = self.get_alu_operand(opcode);
        let (z,n,h,c) = self.cpu.flags_from_sbc(self.cpu.a, val);

        self.cpu.a = self.cpu.a.wrapping_sub(val)
                        .wrapping_sub(self.cpu.get_flag(Gbz80::FLAG_C) as u8);

        self.cpu.set_flags(z,n,h,c);
    }

    pub fn and(&mut self, opcode: u8) {
        let val = self.get_alu_operand(opcode);
        self.cpu.a = self.cpu.a & val;
        self.cpu.set_flags(self.cpu.a == 0, false, true, false);
    }

    pub fn or(&mut self, opcode: u8) {
        let val = self.get_alu_operand(opcode);

        self.cpu.a = self.cpu.a | val;
        self.cpu.set_flags(self.cpu.a == 0, false, false, false);
    }

    pub fn xor(&mut self, opcode: u8) {
        let val = self.get_alu_operand(opcode);
        self.cpu.a = self.cpu.a ^ val;
        self.cpu.set_flags(self.cpu.a == 0, false, false, false);
    }
}
