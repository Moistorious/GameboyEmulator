use crate::gameboy::Gameboy;

impl Gameboy {
    pub fn jp(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }

    pub fn jr(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }

    pub fn call(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }

    pub fn ret(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }

    pub fn reti(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }

    pub fn rst(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }
}
