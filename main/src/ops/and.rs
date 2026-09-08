use crate::gameboy::Gameboy;

impl Gameboy {
    pub fn and(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }
}
