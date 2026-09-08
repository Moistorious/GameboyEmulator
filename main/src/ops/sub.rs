use crate::gameboy::Gameboy;

impl Gameboy {
    pub fn sub(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }
}
