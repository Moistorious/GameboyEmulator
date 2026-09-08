use crate::gameboy::Gameboy;

impl Gameboy {
    pub fn set(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }

    pub fn res(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }
}
