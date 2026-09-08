use crate::gameboy::Gameboy;

impl Gameboy {
    pub fn sbc(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }
}
