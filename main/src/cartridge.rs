
const ROM_SIZE: usize = 32 * 1024; // 32KB for the first bank, can be larger with mappers
pub struct Cartridge {
    rom: [u8; ROM_SIZE],
}

impl Cartridge {
    pub fn new() -> Self {
        Self {
            rom: [0; ROM_SIZE],
        }
    }
    pub fn load_rom(&mut self, data: &[u8]) {
        let len = data.len().min(ROM_SIZE);
        self.rom[..len].copy_from_slice(&data[..len]);
    }
    pub fn read(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }
    pub fn write(&mut self, address: u16, value: u8) {
        self.rom[address as usize] = value;
    }
}