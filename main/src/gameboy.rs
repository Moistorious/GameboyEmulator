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
    pub fn with_state(running:bool, cpu: Gbz80, memory: GbMemory) -> Self {
        Gameboy {
            running: running,
            cpu: cpu,
            memory: memory,
        }
    }
    pub fn read_u8_increment_pc(&mut self) -> u8 {
        let val = self.memory.read_u8(self.cpu.program_counter);
        self.cpu.program_counter = 
            self.cpu.program_counter.wrapping_add(1);
        val
    }
    pub fn read_u16_increment_pc(&mut self) -> u16 {
        let val = self.memory.read_u16(self.cpu.program_counter);
        self.cpu.program_counter = self.cpu.program_counter.wrapping_add(2);
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

    pub fn decode_opcode(&self, opcode: u8) -> (u8, u8, u8) {
        let x = opcode >> 6;
        let y = (opcode >> 3) & 0x07;
        let z = opcode & 0x07;
        (x, y, z)
    }

    pub fn execute_group_0(&mut self, y:u8, z:u8){
        //0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x3E | 0x36 => self.ld_r_n(opcode),
        match z{
            0x00 => todo!(),
            0x01 => match y {
                        0x00 =>self.ld_rr_nn(Reg16::BC),
                        0x02 =>self.ld_rr_nn(Reg16::DE),
                        0x04 =>self.ld_rr_nn(Reg16::HL),
                        0x06 =>self.ld_rr_nn(Reg16::SP),

                        0x01 =>self.add_hl_rr(Reg16::BC),
                        0x03 =>self.add_hl_rr(Reg16::DE),
                        0x05 =>self.add_hl_rr(Reg16::HL),
                        0x07 =>self.add_hl_rr(Reg16::SP),
                        _ => unreachable!()
            },
            0x02 => match y {
                        0x00 => self.ld_rr_a(Reg16::BC),
                        0x01 => self.ld_a_rr(Reg16::BC),
                        0x02 => self.ld_rr_a(Reg16::DE),
                        0x03 => self.ld_a_rr(Reg16::DE),
                        0x04 => self.ld_hli_a(),
                        0x05 => self.ld_hli_a(),
                        0x06 => self.ld_hld_a(),
                        0x07 => self.ld_a_hld(),
                        _ => unreachable!()
            },
            0x03 => todo!(),
            0x04 => todo!(),
            0x05 => todo!(),
            0x06 => self.ld_r_n(Reg8::from_u8(y)),
            0x07 => todo!(),
            _ => unreachable!()
        }
    }

    pub fn execute_group_1(&mut self, y:u8, z:u8){
        if y == 6 && z == 6 { // 0x76
            self.halt();
            return;
        }
        let dest = Reg8::from_u8(y);
        let source = Reg8::from_u8(z);

        self.ld_r_r(dest, source);
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
    }
}
