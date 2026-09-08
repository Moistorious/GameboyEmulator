#[cfg(test)]
mod inc_dec_tests {
    use crate::cpu::Gbz80;
    use crate::gameboy::Gameboy;

    #[test]
    fn test_inc_r8() {
        let mut gb = Gameboy::new();
        gb.cpu.b = 0x0F;
        
        gb.inc(0x04); // INC B
        
        assert_eq!(gb.cpu.b, 0x10);
        assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H != 0); // Half carry
        assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
    }

    #[test]
    fn test_inc_r8_zero() {
        let mut gb = Gameboy::new();
        gb.cpu.b = 0xFF;
        
        gb.inc(0x04); // INC B
        
        assert_eq!(gb.cpu.b, 0x00);
        assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H != 0);
        assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
    }

    #[test]
    fn test_inc_hl_mem() {
        let mut gb = Gameboy::new();
        gb.cpu.set_hl(0xC000);
        gb.memory.write_u8(0xC000, 0x50);
        
        gb.inc(0x34); // INC (HL)
        
        assert_eq!(gb.memory.read_u8(0xC000), 0x51);
        assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
    }

    #[test]
    fn test_inc_r16() {
        let mut gb = Gameboy::new();
        gb.cpu.set_bc(0x0FFF);
        gb.cpu.f = 0xF0; // Set all flags
        
        gb.inc(0x03); // INC BC
        
        assert_eq!(gb.cpu.bc(), 0x1000);
        assert_eq!(gb.cpu.f, 0xF0); // Flags should remain unchanged
    }

    #[test]
    fn test_dec_r8() {
        let mut gb = Gameboy::new();
        gb.cpu.b = 0x10;
        
        gb.dec(0x05); // DEC B
        
        assert_eq!(gb.cpu.b, 0x0F);
        assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H != 0); // Half borrow
        assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
    }

    #[test]
    fn test_dec_r8_zero() {
        let mut gb = Gameboy::new();
        gb.cpu.b = 0x01;
        
        gb.dec(0x05); // DEC B
        
        assert_eq!(gb.cpu.b, 0x00);
        assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
    }

    #[test]
    fn test_dec_r16() {
        let mut gb = Gameboy::new();
        gb.cpu.set_bc(0x1000);
        gb.cpu.f = 0xF0;
        
        gb.dec(0x0B); // DEC BC
        
        assert_eq!(gb.cpu.bc(), 0x0FFF);
        assert_eq!(gb.cpu.f, 0xF0);
    }
}
