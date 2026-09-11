use crate::cpu::{Gbz80, Reg8, Reg16};
use crate::memory::GbMemory;
use crate::cartridge::Cartridge;
use std::fs::File;
use std::io::Read;

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
        val
    }
    pub fn read_u16_increment_pc(&mut self) -> u16 {
        let val = self.memory.read_u16(self.cpu.program_counter);
        self.cpu.program_counter += 2;
        val
    }

    pub fn load_rom(&mut self, _address: u16, filename: &str) -> u16 {
        let mut file = File::open(filename).unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        let len = data.len();

        if len == 256 {
            let mut boot = [0u8; 256];
            boot.copy_from_slice(&data[..256]);
            self.memory.boot_rom = Some(boot);
            self.memory.boot_rom_enabled = true;
        } else {
            self.memory.cartridge = Cartridge::new();
        }
        len as u16
    }

    pub fn nop(&mut self, _opcode: u8) {
        self.cpu.program_counter += 1;
    }

    pub fn halt(&mut self) {
        self.running = false;
    }

    pub fn opcode_dest_register(opcode: u8) -> u8 {
        opcode >> 3 & 0x07
    }

    pub fn decode_opcode(&self, opcode: u8) -> (u8, u8, u8) {
        let x = opcode >> 6;
        let y = (opcode >> 3) & 0x07;
        let z = opcode & 0x07;
        (x, y, z)
    }

    pub fn execute_group_0(&mut self, y:u8, z:u8){
        match z{
            0x00 => todo!(),
            0x01 => todo!(),
            0x02 => match y {
                        0x00 => self.ld_rr_a(Reg16::BC),
                        0x01 => self.ld_rr_a(Reg16::DE),
                        0x02 => self.ld_hli(Reg16::HL),
                        0x03 => self.ld_hld(Reg16::HL),
                        _ => unreachable!()
            },
            0x03 => todo!(),
            0x04 => todo!(),
            0x05 => todo!(),
            0x06 => todo!(),
            0x07 => todo!(),
            _ => unreachable!()
        }
    }

    pub fn execute_group_1(&mut self, y:u8, z:u8){
        let dest = Reg8::from_u8(y);
        let source = Reg8::from_u8(z);
        if dest == source && source == Reg8::HLIndirect {
            self.halt();
        }else{
            self.ld_r_r(dest, source);
        }
    }
    
    pub fn execute_group_2(&mut self, y:u8, z:u8){

    }
    
    pub fn execute_group_3(&mut self, opcode:u8, y:u8, z:u8){

    }

    pub fn step(&mut self) {
        // Opcode Byte: [ Bit 7 | Bit 6 ] [ Bit 5 | Bit 4 | Bit 3 ] [ Bit 2 | Bit 1 | Bit 0 ]
        //                Group (x)         Destination (y)            Source (z)
        let opcode = self.read_u8_increment_pc();
        let (x,y,z) = self.decode_opcode(opcode);

        match x {
            0 => self.execute_group_0(y, z),
            1 => self.execute_group_1(y, z),
            2 => self.execute_group_2(y, z),
            3 => self.execute_group_3(opcode, y, z),
            _ => unreachable!()
        }

        match opcode {
            0x00 => self.nop(opcode),

            // <LD
            0x40..=0x7F | 
            0x01 | 0x11 | 0x21 | 0x31 | // reg16
            0xE0 | 0xF0 | 0x08 |
            0x06 | 0x16 | 0x26 | 0x36 |
            0x0E | 0x1E | 0x2E | 0x3E |
            0x02 | 0x12 | 0x22 | 0x32 | 0xE2 | 0xF2 |
            0xEA | 0xFA | 0x0A | 0x1A | 0x2A | 0x3A |
            0xF8 | 0xF9 => self.ld(opcode),
            // LD>
            // <Arithmetic
            0x80..=0x87 | 0xC6 | 0x09 | 0x19 | 0x29 | 0x39 | 0xE8 => self.add(opcode),
            0x88..=0x8F | 0xCE => self.adc(opcode),
            0x90..=0x97 | 0xD6 => self.sub(opcode),
            0x98..=0x9F | 0xDE => self.sbc(opcode),
            0xA0..=0xA7 | 0xE6 => self.and(opcode),
            0xA8..=0xAF | 0xEE => self.xor(opcode),
            0xB0..=0xB7 | 0xF6 => self.or(opcode),
            0xB8..=0xBF | 0xFE => self.cp(opcode),
            
            // Arithmetic>
            
            0xCB => {
                let opcode = self.read_u8_increment_pc(); 
                match opcode {
                    0x00..=0x07 => self.rlc(opcode),
                    0x08..=0x0f => self.rrc(opcode),

                    0x10..=0x17 => self.rl(opcode),
                    0x18..=0x1f => self.rr(opcode),
                    
                    0x20..=0x27 => self.sla(opcode),
                    0x28..=0x2f => self.sra (opcode),
                    
                    0x30..=0x37 => self.swap(opcode),
                    0x38..=0x3F => self.srl(opcode),
                    0x40..=0x7F => self.bit(opcode),
                    0x80..=0xBF => self.res(opcode),
                    0xA0..=0xFF => self.set(opcode)
                }
            },

            _ => todo!()
        }





        //DISPATCH[opcode as usize](self, opcode);
    }
}
