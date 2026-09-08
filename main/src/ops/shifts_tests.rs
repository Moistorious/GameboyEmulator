#[cfg(test)]
mod shifts_tests {
    use crate::gameboy::Gameboy;
    use crate::cpu::Gbz80;

    #[test]
    fn test_rlc() {
        let mut gb = Gameboy::new();
        // RLC A (0x07)
        gb.cpu.a = 0x85; // 1000 0101
        gb.rlc(0x07);
        // Bit 7 (1) moves to bit 0 and Carry. Result: 0x0B (0000 1011), C=1
        assert_eq!(gb.cpu.a, 0x0B);
        assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
        assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
        // RLC B (0x00)
        gb.cpu.b = 0x00;
        gb.rlc(0x00);
        assert_eq!(gb.cpu.b, 0x00);
        assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
        assert!(gb.cpu.f & Gbz80::FLAG_C == 0);

        // RLC (HL) (0x06)
        gb.cpu.set_hl(0xC000);
        gb.memory.write_u8(0xC000, 0x80);
        gb.rlc(0x06);
        assert_eq!(gb.memory.read_u8(0xC000), 0x01);
        assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
    }

    #[test]
    fn test_rrc() {
        let mut gb = Gameboy::new();
        // RRC A (0x0F)
        gb.cpu.a = 0x01; // 0000 0001
        gb.rrc(0x0F);
        // Bit 0 (1) moves to bit 7 and Carry. Result: 0x80 (1000 0000), C=1
        assert_eq!(gb.cpu.a, 0x80);
        assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
        assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H == 0);

        // RRC (HL) (0x0E)
        gb.cpu.set_hl(0xC000);
        gb.memory.write_u8(0xC000, 0x80);
        gb.rrc(0x0E);
        assert_eq!(gb.memory.read_u8(0xC000), 0x40);
        assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
    }

    #[test]
    fn test_rl() {
        let mut gb = Gameboy::new();
        // RL A (0x17)
        gb.cpu.a = 0x80;
        gb.cpu.set_flag(Gbz80::FLAG_C, false);
        gb.rl(0x17);
        // Bit 7 (1) moves to Carry, Carry (0) moves to bit 0. Result: 0x00, C=1
        assert_eq!(gb.cpu.a, 0x00);
        assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
        assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
        assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H == 0);

        // RL (HL) (0x16)
        gb.cpu.set_hl(0xC000);
        gb.memory.write_u8(0xC000, 0x11);
        gb.cpu.set_flag(Gbz80::FLAG_C, true);
        gb.rl(0x16);
        assert_eq!(gb.memory.read_u8(0xC000), 0x23);
        assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
    }

    #[test]
    fn test_rr() {
        let mut gb = Gameboy::new();
        // RR A (0x1F)
        gb.cpu.a = 0x01;
        gb.cpu.set_flag(Gbz80::FLAG_C, false);
        gb.rr(0x1F);
        // Bit 0 (1) moves to Carry, Carry (0) moves to bit 7. Result: 0x00, C=1
        assert_eq!(gb.cpu.a, 0x00);
        assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
        assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
        assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H == 0);

        // RR (HL) (0x1E)
        gb.cpu.set_hl(0xC000);
        gb.memory.write_u8(0xC000, 0x88);
        gb.cpu.set_flag(Gbz80::FLAG_C, true);
        gb.rr(0x1E);
        assert_eq!(gb.memory.read_u8(0xC000), 0xC4);
        assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
    }

    #[test]
    fn test_sla() {
        let mut gb = Gameboy::new();
        // SLA A (0x27)
        gb.cpu.a = 0x80;
        gb.sla(0x27);
        assert_eq!(gb.cpu.a, 0x00);
        assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
        assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
        assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H == 0);

        // SLA (HL) (0x26)
        gb.cpu.set_hl(0xC000);
        gb.memory.write_u8(0xC000, 0xFF);
        gb.sla(0x26);
        assert_eq!(gb.memory.read_u8(0xC000), 0xFE);
        assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
        assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
    }

    #[test]
    fn test_sra() {
        let mut gb = Gameboy::new();
        // SRA A (0x2F)
        gb.cpu.a = 0x81;
        gb.sra(0x2F);
        assert_eq!(gb.cpu.a, 0xC0);
        assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
        assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H == 0);

        // SRA (HL) (0x2E)
        gb.cpu.set_hl(0xC000);
        gb.memory.write_u8(0xC000, 0x01);
        gb.sra(0x2E);
        assert_eq!(gb.memory.read_u8(0xC000), 0x00);
        assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
        assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
        assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
    }

    #[test]
    fn test_swap() {
        let mut gb = Gameboy::new();
        // SWAP A (0x37)
        gb.cpu.a = 0xF0;
        gb.swap(0x37);
        assert_eq!(gb.cpu.a, 0x0F);
        assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
        
        // SWAP (HL) (0x36)
        gb.cpu.set_hl(0xC000);
        gb.memory.write_u8(0xC000, 0x12);
        gb.swap(0x36);
        assert_eq!(gb.memory.read_u8(0xC000), 0x21);
        assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);

        // SWAP producing zero
        gb.cpu.a = 0x00;
        gb.swap(0x37);
        assert_eq!(gb.cpu.a, 0x00);
        assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
    }

    #[test]
    fn test_srl() {
        let mut gb = Gameboy::new();
        // SRL A (0x3F)
        gb.cpu.a = 0x81;
        gb.srl(0x3F);
        assert_eq!(gb.cpu.a, 0x40);
        assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
        assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H == 0);

        // SRL (HL) (0x3E)
        gb.cpu.set_hl(0xC000);
        gb.memory.write_u8(0xC000, 0x01);
        gb.srl(0x3E);
        assert_eq!(gb.memory.read_u8(0xC000), 0x00);
        assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
        assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
        assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
    }
}
