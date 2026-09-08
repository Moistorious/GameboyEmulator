use crate::gameboy::Gameboy;

impl Gameboy {
    pub fn daa(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }

    pub fn cpl(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }

    pub fn scf(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }

    pub fn ccf(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }

    pub fn di(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }

    pub fn ei(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }

    pub fn stop(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }
}
