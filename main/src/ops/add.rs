use crate::gameboy::Gameboy;

impl Gameboy {
    pub fn add(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }
}
