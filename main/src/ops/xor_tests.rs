#[cfg(test)]
mod ld_tests {
    use crate::cpu::{Reg8, Reg16};
    use crate::gameboy::Gameboy;
    fn xor_opcode(src: Reg8) -> u8 {
        0xA8 | (src as u8)
    }

    #[test]
    fn test_xor_a_a() {
        let mut gameboy = Gameboy::new();

        gameboy.cpu.write_reg8(Reg8::A, 0xAE);

        // XOR A with A
        gameboy.xor(0xAF);

        // dest should equal src after the LD
        assert_eq!(
            gameboy.cpu.reg8(Reg8::A),
            0x00,
            "XOR A,A (opcode 0xAF) failed"
        );
    }
    #[test]
    fn test_xor_a_u8() {
        let mut gameboy = Gameboy::new();

        gameboy.cpu.write_reg8(Reg8::A, 0x01);
        // xor is going to read the next 1 byte, which is at 0x00 right now.
        gameboy.memory.write_u8(0x00, 0xAB);

        gameboy.xor(0xEE);

        // dest should equal src after the LD
        assert_eq!(
            gameboy.cpu.reg8(Reg8::A),
            0xAA,
            "XOR A,(HL) (opcode 0xEE) failed"
        );
    }

    #[test]
    fn test_xor_a_from_hl() {
        let mut gameboy = Gameboy::new();

        gameboy.cpu.write_reg8(Reg8::A, 0x01);
        gameboy.cpu.write_reg16(Reg16::HL, 0x0012);
        gameboy.memory.write_u8(0x0012, 0xAB);

        // XOR A with A
        gameboy.xor(0xAE);

        // dest should equal src after the LD
        assert_eq!(
            gameboy.cpu.reg8(Reg8::A),
            0xAA,
            "XOR A,(HL) (opcode 0xAE) failed"
        );
    }

    #[test]
    fn test_all_xor_reg8() {
        let regs = [Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L];

        for &src in &regs {
            let opcode = xor_opcode(src);

            let mut gameboy = Gameboy::new();

            // initialize dest with 0x01 and src with 0xAB
            gameboy.cpu.write_reg8(Reg8::A, 0x01);
            gameboy.cpu.write_reg8(src, 0xAB);

            gameboy.xor(opcode);

            assert_eq!(
                gameboy.cpu.reg8(Reg8::A),
                0xAA,
                "XOR {:?},{:?} (opcode 0x{:02X}) failed",
                Reg8::A,
                src,
                opcode
            );
        }
    }
}
