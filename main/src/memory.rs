use byteorder::{ByteOrder, LittleEndian};

const WRAM_SIZE: usize = 8 * 1024;
const VRAM_SIZE: usize = 8 * 1024;
const OAM_SIZE: usize = 0xFE9F - 0xFE00 + 1;
const IO_REGISTERS_SIZE: usize = 0xFF7F - 0xFF00 + 1;
const HRAM_SIZE: usize = 0xFFFE - 0xFF80 + 1;

pub struct GbMemory {
    wram: [u8; WRAM_SIZE],
    vram: [u8; VRAM_SIZE],
    cartridge: Vec<u8>,
    oam: [u8; OAM_SIZE],
    io_registers: [u8; IO_REGISTERS_SIZE],
    hram: [u8; HRAM_SIZE],
    interrupt_enable: u8,
}

impl GbMemory {
    pub fn new() -> Self {
        GbMemory {
            cartridge: Vec::new(),
            vram: [0; VRAM_SIZE],
            wram: [0; WRAM_SIZE],
            oam: [0; OAM_SIZE],
            io_registers: [0; IO_REGISTERS_SIZE],
            hram: [0; HRAM_SIZE],
            interrupt_enable: 0,
        }
    }

    // pub fn read_u8(&self, address: u16) -> u8 {
    //     return self.memory[address as usize];
    // }

    pub fn read_u16(&self, address: u16) -> u16 {
        let low = self.read_u8(address);
        let high = self.read_u8(address + 1);
        return ((high as u16) << 8) | (low as u16);
    }
    // pub fn read_u16(&self, address: u16) -> u16 {
    //     return LittleEndian::read_u16(&self.memory[address as usize..(address + 2) as usize]);
    // }

    pub fn write_u16(&mut self, address: u16, value: u16) {
        self.write_u8(address, (value & 0xFF) as u8);
        self.write_u8(address + 1, ((value >> 8) & 0xFF) as u8);
    }

    pub fn write_u8(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x3FFF => (), // From cartridge, usually a fixed bank
            0x4000..=0x7FFF => (), // From cartridge, switchable bank via mapper (if any)
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize] = value, // In CGB mode, switchable bank 0/1
            0xA000..=0xBFFF => self.cartridge[address as usize] = value, // External RAM from cartridge, switchable bank via mapper (if any)
            0xC000..=0xCFFF => self.wram[(address - 0xC000) as usize] = value, // working RAM
            0xD000..=0xDFFF => self.wram[(address - 0xC000) as usize] = value, // In CGB mode, switchable bank 1–7
            0xE000..=0xFDFF => self.wram[(address - 0xE000) as usize] = value, // Echo RAM Nintendo says use of this area is prohibited.
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize] = value, // Sprite attribute table (OAM)
            0xFEA0..=0xFEFF => (), // Unusable memory area, writes are ignored
            0xFF00..=0xFF7F => self.io_registers[(address - 0xFF00) as usize] = value, // IO Registers
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize] = value, // High RAM (HRAM)
            0xFFFF => self.interrupt_enable = value, // Interrupt Enable Register
        }
        // self.memory[address as usize] = value;
    }
    pub fn read_u8(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.cartridge[address as usize], // From cartridge, usually a fixed bank
            0x4000..=0x7FFF => self.cartridge[address as usize], // From cartridge, switchable bank via mapper (if any)
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize], // In CGB mode, switchable bank 0/1
            0xA000..=0xBFFF => self.cartridge[address as usize], // External RAM from cartridge, switchable bank via mapper (if any)
            0xC000..=0xCFFF => self.wram[(address - 0xC000) as usize], // working RAM
            0xD000..=0xDFFF => self.wram[(address - 0xC000) as usize], // In CGB mode, switchable bank 1–7
            0xE000..=0xFDFF => self.wram[(address - 0xE000) as usize], // Echo RAM Nintendo says use of this area is prohibited.
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize], // Sprite attribute table (OAM)
            0xFEA0..=0xFEFF => 0xFF, // Unusable memory area, reads typically return 0xFF
            0xFF00..=0xFF7F => self.io_registers[(address - 0xFF00) as usize], // IO Registers
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize], // High RAM (HRAM)
            0xFFFF => self.interrupt_enable, // Interrupt Enable Register
        }
        //return self.map_address(address).unwrap_or(&[0])[0];
        //return self.memory[address as usize];
    }
}
