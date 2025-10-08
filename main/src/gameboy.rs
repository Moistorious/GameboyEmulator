use crate::cpu::Gbz80;
use crate::memory::GbMemory;
use once_cell::sync::Lazy;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub struct Gameboy {
    pub running: bool,
    pub cpu: Gbz80,
    pub memory: GbMemory,
}

type GameboyInstruction = fn(&mut Gameboy, u8);

static DISPATCH: Lazy<[GameboyInstruction; 256]> = Lazy::new(|| {
    let mut table: [GameboyInstruction; 256] = [Gameboy::not_implemented; 256];
    table[0x00] = Gameboy::nop;

    for i in 0x01..=0x3f {
        let lsb4 = i & 0x0f;
        match lsb4 {
            0x01 | 0x02 | 0x06 | 0x0A | 0x0E => table[i] = Gameboy::ld,
            //0x03 | 0x04 | 0x0C => table[i] = Gameboy::inc,
            //0x05 | 0x0B | 0x0D => table[i] = Gameboy::dec,
            //0x09 => table[i] = Gameboy::add,
            _ => table[i] = Gameboy::not_implemented,
        }
    }

    table[0x40..=0x7f].fill(Gameboy::ld);
    for i in [0xE0, 0xE2, 0xEA, 0xF0, 0xF2, 0xF8, 0xF9, 0xFA] {
        table[i] = Gameboy::ld;
    }
    //table[0x07] = Gameboy::RLCA;
    //table[0x17] = Gameboy::RLA;
    //table[0x27] = Gameboy::DAA;
    //table[0x37] = Gameboy::SCF;

    table[0x76] = Gameboy::not_implemented; // HALT

    //table[0x80..=0x87].fill(Gameboy::add);
    //table[0x88..=0x8F].fill(Gameboy::adc);
    //table[0x90..=0x97].fill(Gameboy::sub);
    //table[0x98..=0x9F].fill(Gameboy::sbc);
    //table[0xA0..=0xA7].fill(Gameboy::and);
    table[0xA8..=0xAF].fill(Gameboy::xor);
    //table[0xB0..=0xB7].fill(Gameboy::or);
    //table[0xB8..=0xBF].fill(Gameboy::cp);

    //table[0xC0] = Gameboy::ret;
    //table[0xD0] = Gameboy::ret;
    //table[0xD8] = Gameboy::ret;

    //table[0xCE] Gameboy::adc;
    //table[0xE6] = Gameboy::and;
    //table[0xCB] = Gameboy::cb;
    //table[0xFE] = Gameboy::cp;
    //table[0xF3] = Gameboy::di;
    //table[0xFB] = Gameboy::ei;
    //table[0xF6] = Gameboy::or;
    //table[0xD9] = Gameboy::reti;
    //table[0xDE] = Gameboy::sbc;
    //table[0xD6] = Gameboy::sub;
    table[0xEE] = Gameboy::xor;

    // for i in [0xD4,0xDC,0xC4,0xCD,0xCC] {
    //     table[i] = Gameboy::call;
    // }

    //table[0xC6] = Gameboy::add;
    //table[0xE8] = Gameboy::add;

    // for i in [0xC2,0xC3,0xCA,0xD2,0xDA,0xE9] {
    //     table[i] = Gameboy::jp;
    // }

    //table[] = Gameboy::ld;
    //table[] = Gameboy::ld;
    //table[] = Gameboy::ld;
    //table[] = Gameboy::ld;
    //table[] = Gameboy::ld;
    //table[] = Gameboy::ld;
    //table[] = Gameboy::ld;
    //table[] = Gameboy::ld;

    //table[0xC1] = Gameboy::pop;
    //table[0xD1] = Gameboy::pop;
    //table[0xE1] = Gameboy::pop;
    //table[0xF1] = Gameboy::pop;

    //table[0xC5] = Gameboy::push;
    //table[0xD5] = Gameboy::push;
    //table[0xE5] = Gameboy::push;
    //table[0xF5] = Gameboy::push;

    // for _i in [0xC7, 0xCF, 0xD7, 0xDF, 0xE7, 0xEF, 0xF7, 0xFF]{
    //     table[i] = Gameboy::rst;
    // }

    //table[0xC8] = Gameboy::ret;
    //table[0xC9] = Gameboy::ret;

    table
});

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

    pub fn load_rom(&mut self, address: u16, filename: &str) -> u16 {
        let my_buf = BufReader::new(File::open(filename).unwrap());
        let mut i = 0;

        for byte_or_error in my_buf.bytes() {
            let byte = byte_or_error.unwrap();

            self.memory.write_u8(address + i, byte);

            i = i + 1;
        }
        i
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

    pub fn not_implemented(&mut self, opcode: u8) {
        println!("instruction not implemented opcode {:0X}", opcode);
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

    pub fn execute_next(&mut self) {
        let opcode = self.read_u8_increment_pc();
        DISPATCH[opcode as usize](self, opcode);
    }
}
