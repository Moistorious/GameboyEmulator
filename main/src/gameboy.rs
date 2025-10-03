use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use crate::cpu::{Gbz80, Reg8, Reg16};
use crate::memory::GbMemory;

pub struct Gameboy {
    pub running: bool,
    pub cpu: Gbz80,
    pub memory: GbMemory,
}
#[allow(dead_code)]
#[allow(unused)]
impl Gameboy {
    pub fn new() -> Self {
        Gameboy {
            running: true,
            cpu: Gbz80::new(),
            memory: GbMemory::new(),
        }
    }

    pub fn read_u8_increment_pc(&mut self) -> u8 {
        let val = self.memory.read_u8(self.cpu.program_counter);
        self.cpu.program_counter += 1;
        return val;
    }
    pub fn read_u16_increment_pc(&mut self) -> u16 {
        let val = self.memory.read_u16(self.cpu.program_counter);
        self.cpu.program_counter += 2;
        return val;
    }

    pub fn load_rom(&mut self, address: u16, filename: &str) -> u16 {
        let my_buf = BufReader::new(File::open(filename).unwrap());
        let mut i = 0;

        for byte_or_error in my_buf.bytes() {
            let byte = byte_or_error.unwrap();

            self.memory.write_u8(address + i, byte);

            i = i + 1;
        }
        return i;
    }

    pub fn ld_n_n(&self, opcode: u8) {
        println!("{}", opcode);
    }

    pub fn ld_16_16(&self, source: u16, dest: &mut u16) {
        *dest = source;
    }

    pub fn ld_sp_u16(&mut self, program_counter: &mut u16, stack_pointer: &mut u16) {
        self.ld_16_16(self.memory.read_u16(*program_counter + 1), stack_pointer);
        *program_counter = *program_counter + 3;
    }

    pub fn not_implemented(&mut self, opcode: u8, instruction: &str) {
        println!(
            "instruction not implemented: {} opcode {:0X}",
            instruction, opcode
        );
    }

    pub fn nop(&mut self, _opcode: u8) {
        self.cpu.program_counter += 1;
    }

    fn opcode_dest_register(opcode: u8) -> u8 {
        return opcode >> 3 & 0x07;
    }

    pub fn ld(&mut self, opcode: u8) {
        //B=0, C=1, D=2, E=3, H=4, L=5, F=6, A=7
        let msb4: u8 = opcode >> 4;
        let lsb4: u8 = opcode & 0x0F;

        let mut dest_register = Reg8::try_from(Self::opcode_dest_register(opcode)).unwrap();
        let mut source: u8 = 0;

        if lsb4 == 0x06 || lsb4 == 0x0E {
            if opcode > 0x40 {
                source = self.memory.read_u8(self.cpu.reg16(Reg16::HL));
            } else {
                // opcode < 0x40
                source = self.read_u8_increment_pc();
                if opcode == 0x36 {
                    self.memory.write_u8(self.cpu.reg16(Reg16::HL), source);
                    return; // exit early
                }
            }
        } else if opcode >= 0x40 {
            let source_register = Reg8::try_from(opcode & 7).unwrap();
            source = self.cpu.reg8(source_register);
            if opcode >= 0x70 && lsb4 < 8  {
                self.memory.write_u8(self.cpu.reg16(Reg16::HL), source);
                return;
            }
        } else if lsb4 == 0x0A {
            dest_register = Reg8::A;
            match msb4 {
                0x00 => source = self.memory.read_u8(self.cpu.reg16(Reg16::BC)),
                0x01 => source = self.memory.read_u8(self.cpu.reg16(Reg16::DE)),
                0x02 => {
                    source = self.memory.read_u8(self.cpu.reg16(Reg16::HL));
                    self.cpu
                        .write_reg16(Reg16::HL, self.cpu.reg16(Reg16::HL) + 1);
                }
                0x03 => {
                    source = self.memory.read_u8(self.cpu.reg16(Reg16::HL));
                    self.cpu
                        .write_reg16(Reg16::HL, self.cpu.reg16(Reg16::HL) - 1);
                }
                _ => println!("something's fucked"),
            }
        } else if lsb4 == 0x02 {
            source = self.cpu.reg8(Reg8::A);
            match msb4 {
                0x00 => self.memory.write_u8(self.cpu.reg16(Reg16::BC), source),
                0x01 => self.memory.write_u8(self.cpu.reg16(Reg16::DE), source),
                0x02 => {
                    self.memory.write_u8(self.cpu.reg16(Reg16::HL), source);
                    self.cpu
                        .write_reg16(Reg16::HL, self.cpu.reg16(Reg16::HL) + 1);
                }
                0x03 => {
                    self.memory.write_u8(self.cpu.reg16(Reg16::HL), source);
                    self.cpu
                        .write_reg16(Reg16::HL, self.cpu.reg16(Reg16::HL) - 1);
                }
                _ => println!("something's fucked"),
            }
            return;
        }

        self.cpu.write_reg8(dest_register, source);
        //self.not_implemented(opcode, "ld");
    }

    pub fn inc(&mut self, opcode: u8) {
        self.not_implemented(opcode, "inc");
    }
    pub fn dec(&mut self, opcode: u8) {
        self.not_implemented(opcode, "dec");
    }
    pub fn rlca(&mut self, opcode: u8) {
        self.not_implemented(opcode, "rlca");
    }
    pub fn add(&mut self, opcode: u8) {
        self.not_implemented(opcode, "add");
    }
    pub fn rrca(&mut self, opcode: u8) {
        self.not_implemented(opcode, "rrca");
    }
    pub fn stop(&mut self, opcode: u8) {
        self.not_implemented(opcode, "stop");
    }
    pub fn rla(&mut self, opcode: u8) {
        self.not_implemented(opcode, "rla");
    }
    pub fn jr(&mut self, opcode: u8) {
        self.not_implemented(opcode, "jr");
    }
    pub fn rra(&mut self, opcode: u8) {
        self.not_implemented(opcode, "rra");
    }
    pub fn daa(&mut self, opcode: u8) {
        self.not_implemented(opcode, "daa");
    }
    pub fn cpl(&mut self, opcode: u8) {
        self.not_implemented(opcode, "cpl");
    }
    pub fn scf(&mut self, opcode: u8) {
        self.not_implemented(opcode, "scf");
    }
    pub fn ccf(&mut self, opcode: u8) {
        self.not_implemented(opcode, "ccf");
    }
    pub fn halt(&mut self, opcode: u8) {
        self.not_implemented(opcode, "halt");
    }
    pub fn adc(&mut self, opcode: u8) {
        self.not_implemented(opcode, "adc");
    }
    pub fn sub(&mut self, opcode: u8) {
        self.not_implemented(opcode, "sub");
    }
    pub fn sbc(&mut self, opcode: u8) {
        self.not_implemented(opcode, "sbc");
    }
    pub fn and(&mut self, opcode: u8) {
        self.not_implemented(opcode, "and");
    }
    pub fn xor(&mut self, opcode: u8) {
        self.not_implemented(opcode, "xor");
    }
    pub fn or(&mut self, opcode: u8) {
        self.not_implemented(opcode, "or");
    }
    pub fn cp(&mut self, opcode: u8) {
        self.not_implemented(opcode, "cp");
    }
    pub fn ret(&mut self, opcode: u8) {
        self.not_implemented(opcode, "ret");
    }
    pub fn pop(&mut self, opcode: u8) {
        self.not_implemented(opcode, "pop");
    }
    pub fn jp(&mut self, opcode: u8) {
        self.not_implemented(opcode, "jp");
    }
    pub fn call(&mut self, opcode: u8) {
        self.not_implemented(opcode, "call");
    }
    pub fn push(&mut self, opcode: u8) {
        self.not_implemented(opcode, "push");
    }
    pub fn rst(&mut self, opcode: u8) {
        self.not_implemented(opcode, "rst");
    }
    pub fn cb(&mut self, opcode: u8) {
        self.not_implemented(opcode, "cb");
    }
    pub fn unused(&mut self, opcode: u8) {
        self.not_implemented(opcode, "unused");
    }
    pub fn reti(&mut self, opcode: u8) {
        self.not_implemented(opcode, "reti");
    }
    pub fn di(&mut self, opcode: u8) {
        self.not_implemented(opcode, "di");
    }
    pub fn ei(&mut self, opcode: u8) {
        self.not_implemented(opcode, "ei");
    }
    pub fn rlc(&mut self, opcode: u8) {
        self.not_implemented(opcode, "rlc");
    }
    pub fn rrc(&mut self, opcode: u8) {
        self.not_implemented(opcode, "rrc");
    }
    pub fn rl(&mut self, opcode: u8) {
        self.not_implemented(opcode, "rl");
    }
    pub fn rr(&mut self, opcode: u8) {
        self.not_implemented(opcode, "rr");
    }
    pub fn sla(&mut self, opcode: u8) {
        self.not_implemented(opcode, "sla");
    }
    pub fn sra(&mut self, opcode: u8) {
        self.not_implemented(opcode, "sra");
    }
    pub fn swap(&mut self, opcode: u8) {
        self.not_implemented(opcode, "swap");
    }
    pub fn srl(&mut self, opcode: u8) {
        self.not_implemented(opcode, "srl");
    }
    pub fn bit(&mut self, opcode: u8) {
        self.not_implemented(opcode, "bit");
    }
    pub fn res(&mut self, opcode: u8) {
        self.not_implemented(opcode, "res");
    }
    pub fn set(&mut self, opcode: u8) {
        self.not_implemented(opcode, "set");
    }

    pub fn execute_next(&mut self) {
        let opcode = self.read_u8_increment_pc();
        let msb4: u8 = opcode >> 4;
        let lsb4: u8 = opcode & 0x0F;

        match opcode {
            0x0 => self.nop(opcode),
            0x01..=0x3f if lsb4 != 0x07 && lsb4 != 0x08 && lsb4 != 0x00 && lsb4 != 0x0F => {
                match lsb4 {
                    0x01 | 0x02 | 0x06 | 0x0A | 0x0E => self.ld(opcode),
                    0x03 | 0x04 | 0x0C => self.inc(opcode),
                    0x05 | 0x0B | 0x0D => self.dec(opcode),
                    0x09 => self.add(opcode),
                    _ => println!("lsb4 match failed, opcode {}", opcode),
                }
            }

            0x40..=0x7f if opcode != 0x76 => self.ld(opcode),
            0x76 => self.halt(opcode),

            0x80..=0x87 => self.add(opcode),

            0x88..=0x8F => self.adc(opcode),

            0x90..=0x97 => self.sub(opcode),

            0x98..=0x9F => self.sbc(opcode),

            0xA0..=0xA7 => self.and(opcode),

            0xA8..=0xAF => self.xor(opcode),

            0xB0..=0xB7 => self.or(opcode),

            0xB8..=0xBF => self.cp(opcode),

            0xC0 | 0xD0 | 0xD8 => self.ret(opcode),

            0xCE => self.adc(opcode),
            0xC6 | 0xE8 => self.add(opcode),

            0xE6 => self.and(opcode),
            0xD4 | 0xDC | 0xC4 | 0xCD | 0xCC => self.call(opcode),
            0xCB => self.cb(opcode),
            0xFE => self.cp(opcode),
            0xF3 => self.di(opcode),
            0xFB => self.ei(opcode),

            0xC2 | 0xC3 | 0xCA | 0xD2 | 0xDA | 0xE9 => self.jp(opcode),
            0xE0 | 0xE2 | 0xEA | 0xF0 | 0xF2 | 0xF8 | 0xF9 | 0xFA => self.ld(opcode),
            0xF6 => self.or(opcode),

            0xC1 | 0xD1 | 0xE1 | 0xF1 => self.pop(opcode),

            0xC5 | 0xD5 | 0xE5 | 0xF5 => self.push(opcode),
            0xC8 | 0xC9 => self.ret(opcode),
            0xD9 => self.reti(opcode),

            0xC7 | 0xCF | 0xD7 | 0xDF | 0xE7 | 0xEF | 0xF7 | 0xFF => self.rst(opcode),
            0xDE => self.sbc(opcode),
            0xD6 => self.sub(opcode),
            0xEE => self.xor(opcode),
            _ => self.unused(opcode),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn ld_opcode(dest: Reg8, src: Reg8) -> u8 {
        0x40 | ((dest as u8) << 3) | (src as u8)
    }

    #[test]
    fn test_all_ld_reg8_to_reg8_instructions() {
        let regs = [
            Reg8::B,
            Reg8::C,
            Reg8::D,
            Reg8::E,
            Reg8::H,
            Reg8::L,
            Reg8::A,
        ];

        for &dest in &regs {
            for &src in &regs {
                // Skip LD (HL),(HL) because opcode 0x76 is HALT
                if dest == Reg8::L && src == Reg8::L || dest == src {
                    continue;
                }

                let opcode = ld_opcode(dest, src);
                if opcode == 0x46 {
                    assert_eq!(1, 2);
                }

                let mut gameboy = Gameboy::new();

                // initialize dest with 0x00 and src with 0xAB
                gameboy.cpu.write_reg8(dest, 0x00);
                gameboy.cpu.write_reg8(src, 0xAB);

                // execute LD dest,src
                gameboy.ld(opcode);

                // dest should equal src after the LD
                assert_eq!(
                    gameboy.cpu.reg8(dest),
                    0xAB,
                    "LD {:?},{:?} (opcode 0x{:02X}) failed",
                    dest,
                    src,
                    opcode
                );
            }
        }
    }

    //
    // LD r,(HL)
    //
    #[test]
    fn test_ld_r_from_hl() {
        let regs = [
            Reg8::B,
            Reg8::C,
            Reg8::D,
            Reg8::E,
            Reg8::H,
            Reg8::L,
            Reg8::A,
        ];

        for &dest in &regs {
            let opcode = 0x46 | ((dest as u8) << 3); // LD r,(HL)

            let mut gb = Gameboy::new();
            gb.cpu.write_reg16(Reg16::HL, 0x1234);
            gb.memory.write_u8(0x1234, 0x5A);

            gb.ld(opcode);

            assert_eq!(gb.cpu.reg8(dest), 0x5A, "LD {:?},(HL) failed", dest);
        }
    }

    //
    // LD (HL),r
    //
    #[test]
    fn test_ld_hl_from_r() {
        let regs = [
            Reg8::B,
            Reg8::C,
            Reg8::D,
            Reg8::E,
            Reg8::H,
            Reg8::L,
            Reg8::A,
        ];

        for &src in &regs {
            let opcode = 0x70 | (src as u8); // LD (HL),r

            let mut gb = Gameboy::new();
            gb.cpu.write_reg16(Reg16::HL, 0x1F1F);
            gb.cpu.write_reg8(src, 0x1F);

            gb.ld(opcode);

            assert_eq!(gb.memory.read_u8(0x1F1F), 0x1F, "LD (HL),{:?} failed", src);
        }
    }

    //
    // LD r,n (immediate)
    //
    #[test]
    fn test_ld_r_n() {
        let tests = [
            (Reg8::B, 0x06),
            (Reg8::C, 0x0E),
            (Reg8::D, 0x16),
            (Reg8::E, 0x1E),
            (Reg8::H, 0x26),
            (Reg8::L, 0x2E),
            (Reg8::A, 0x3E),
        ];

        for &(reg, opcode) in &tests {
            let mut gb = Gameboy::new();
            gb.memory.write_u8(0, 0x99); // pretend immediate at PC+1

            gb.ld(opcode);

            assert_eq!(gb.cpu.reg8(reg), 0x99, "LD {:?},n failed", reg);
        }
    }

    //
    // LD A,(BC) and LD A,(DE)
    //
    #[test]
    fn test_ld_a_from_bc_de() {
        let tests = [(0x0A, Reg16::BC), (0x1A, Reg16::DE)];

        for &(opcode, pair) in &tests {
            let mut gb = Gameboy::new();
            gb.cpu.write_reg16(pair, 0x1FFF);
            gb.memory.write_u8(0x1FFF, 0x55);

            gb.ld(opcode);

            assert_eq!(gb.cpu.reg8(Reg8::A), 0x55, "LD A,({:?}) failed", pair);
        }
    }

    //
    // LD (BC),A and LD (DE),A
    //
    #[test]
    fn test_ld_bc_de_from_a() {
        let tests = [(0x02, Reg16::BC), (0x12, Reg16::DE)];

        for &(opcode, pair) in &tests {
            let mut gb = Gameboy::new();
            gb.cpu.write_reg16(pair, 0x1FFF);
            gb.cpu.write_reg8(Reg8::A, 0x66);

            gb.ld(opcode);

            assert_eq!(gb.memory.read_u8(0x1FFF), 0x66, "LD ({:?}),A failed", pair);
        }
    }

    // You’d then continue with LD A,(nn), LD (nn),A, LD A,(C), LD (C),A, etc.
}
