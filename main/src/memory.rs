use byteorder::{BigEndian, ByteOrder};

pub struct GbMemory {
    memory: [u8; 8 * 1024],
}

impl GbMemory {
    pub fn new() -> Self {
        GbMemory {
            memory: [0; 8 * 1024],
        }
    }
    pub fn read_u8(&self, address: u16) -> u8 {
        return self.memory[address as usize];
    }
    pub fn read_u16(&self, address: u16) -> u16 {
        return BigEndian::read_u16(&self.memory[address as usize..(address + 2) as usize]);
    }

    pub fn write_u16(&mut self, address: u16, value: u16) {
        BigEndian::write_u16(
            &mut self.memory[address as usize..(address + 2) as usize],
            value,
        );
    }

    pub fn write_u8(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}
