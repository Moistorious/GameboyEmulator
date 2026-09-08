use crate::gameboy::Gameboy;

impl Gameboy {
    pub fn add(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }

    pub fn adc(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }

    pub fn sub(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }

    pub fn sbc(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }

    pub fn cp(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }
}
