#[cfg(test)]
mod or_tests {
    use crate::cpu::{Reg8, Gbz80};
    use crate::gameboy::Gameboy;

    fn or_opcode(src: Reg8) -> u8 {
        0xB0 | (src as u8)
    }

    #[test]
    fn test_all_or_reg8() {
        let regs = [Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L];

        for &src in &regs {
            let opcode = or_opcode(src);

            let mut gameboy = Gameboy::new();

            gameboy.cpu.write_reg8(Reg8::A, 0x0F);
            gameboy.cpu.write_reg8(src, 0xF0);

            gameboy.or(opcode);

            assert_eq!(
                gameboy.cpu.reg8(Reg8::A),
                0xFF,
                "OR {:?},{:?} (opcode 0x{:02X}) failed",
                Reg8::A,
                src,
                opcode
            );
            // 0x0F | 0xF0 = 0xFF: no zero, no carry
            assert!(gameboy.cpu.f & Gbz80::FLAG_Z == 0);
            assert!(gameboy.cpu.f & Gbz80::FLAG_N == 0);
            assert!(gameboy.cpu.f & Gbz80::FLAG_H == 0);
            assert!(gameboy.cpu.f & Gbz80::FLAG_C == 0);
        }
    }

    #[test]
    fn test_or_zero_result() {
        let mut gameboy = Gameboy::new();

        gameboy.cpu.write_reg8(Reg8::A, 0x00);
        gameboy.cpu.write_reg8(Reg8::B, 0x00);

        gameboy.or(or_opcode(Reg8::B));

        assert_eq!(gameboy.cpu.reg8(Reg8::A), 0x00);
        assert!(gameboy.cpu.f & Gbz80::FLAG_Z != 0);
        assert!(gameboy.cpu.f & Gbz80::FLAG_N == 0);
        assert!(gameboy.cpu.f & Gbz80::FLAG_H == 0);
        assert!(gameboy.cpu.f & Gbz80::FLAG_C == 0);
    }

    #[test]
    fn test_or_a_a() {
        let mut gameboy = Gameboy::new();

        gameboy.cpu.write_reg8(Reg8::A, 0x01);
        gameboy.or(or_opcode(Reg8::A));

        assert_eq!(gameboy.cpu.reg8(Reg8::A), 0x01);
        assert!(gameboy.cpu.f & Gbz80::FLAG_Z == 0);
        assert!(gameboy.cpu.f & Gbz80::FLAG_N == 0);
        assert!(gameboy.cpu.f & Gbz80::FLAG_H == 0);
        assert!(gameboy.cpu.f & Gbz80::FLAG_C == 0);
    }
}
