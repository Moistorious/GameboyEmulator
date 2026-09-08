use crate::gameboy::Gameboy;

impl Gameboy {
    pub fn inc(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }

    pub fn dec(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }
}
