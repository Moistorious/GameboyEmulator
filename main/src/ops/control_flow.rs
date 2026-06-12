use crate::gameboy::Gameboy;

impl Gameboy {
    pub fn jp(&mut self, opcode: u8) {
        // TODO: Implement JP (Absolute Jump)
    }

    pub fn jr(&mut self, opcode: u8) {
        // TODO: Implement JR (Relative Jump)
    }

    pub fn call(&mut self, opcode: u8) {
        // TODO: Implement CALL
    }

    pub fn ret(&mut self, opcode: u8) {
        // TODO: Implement RET
    }

    pub fn reti(&mut self, opcode: u8) {
        // TODO: Implement RETI
    }

    pub fn rst(&mut self, opcode: u8) {
        // TODO: Implement RST
    }
}
