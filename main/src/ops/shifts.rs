use crate::gameboy::Gameboy;

impl Gameboy {
    pub fn rlc(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }

    pub fn rrc(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }

    pub fn rl(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }

    pub fn rr(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }

    pub fn sla(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }

    pub fn sra(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }

    pub fn swap(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }

    pub fn srl(&mut self, opcode: u8) {
        self.not_implemented(opcode);
    }
}
