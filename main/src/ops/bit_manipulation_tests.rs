#[cfg(test)]
mod bit_manipulation_tests {
    use crate::gameboy::Gameboy;
    use crate::cpu::Gbz80;

    #[test]
    fn test_bit() {
        let mut gb = Gameboy::new();
        // BIT 0, B (0x40)
        gb.cpu.b = 0x01;
        gb.bit(0x40);
        assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H != 0);

        gb.cpu.b = 0xFE;
        gb.bit(0x40);
        assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);

        // BIT 7, (HL) (0x7E)
        gb.cpu.set_hl(0xC000);
        gb.memory.write_u8(0xC000, 0x80);
        gb.bit(0x7E);
        assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);

        gb.memory.write_u8(0xC000, 0x00);
        gb.bit(0x7E);
        assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
    }

    #[test]
    fn test_set() {
        let mut gb = Gameboy::new();
        // SET 0, B (0xC0)
        gb.cpu.b = 0x00;
        gb.set(0xC0);
        assert_eq!(gb.cpu.b, 0x01);

        // SET 7, A (0xFF)
        gb.cpu.a = 0x00;
        gb.set(0xFF);
        assert_eq!(gb.cpu.a, 0x80);

        // SET 3, (HL) (0xDE)
        gb.cpu.set_hl(0xC000);
        gb.memory.write_u8(0xC000, 0x00);
        gb.set(0xDE);
        assert_eq!(gb.memory.read_u8(0xC000), 0x08);
    }

    #[test]
    fn test_res() {
        let mut gb = Gameboy::new();
        // RES 0, B (0x80)
        gb.cpu.b = 0x01;
        gb.res(0x80);
        assert_eq!(gb.cpu.b, 0x00);

        // RES 7, A (0xBF)
        gb.cpu.a = 0xFF;
        gb.res(0xBF);
        assert_eq!(gb.cpu.a, 0x7F);

        // RES 3, (HL) (0x9E)
        gb.cpu.set_hl(0xC000);
        gb.memory.write_u8(0xC000, 0xFF);
        gb.res(0x9E);
        assert_eq!(gb.memory.read_u8(0xC000), 0xF7);
    }
}
