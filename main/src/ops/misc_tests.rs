#[cfg(test)]
mod misc_tests {
    use crate::gameboy::Gameboy;
    use crate::cpu::Gbz80;

    #[test]
    fn test_daa() {
        let mut gb = Gameboy::new();
        
        // Addition examples
        // 0x15 + 0x27 = 0x3C -> 0x42
        gb.cpu.a = 0x3C;
        gb.cpu.set_flags(false, false, false, false);
        gb.daa(1 as u8);
        assert_eq!(gb.cpu.a, 0x42);
        assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H == 0);

        // 0x45 + 0x55 = 0x9A -> 0x00, C=1
        gb.cpu.a = 0x9A;
        gb.cpu.set_flags(false, false, false, false);
        gb.daa(1 as u8);
        assert_eq!(gb.cpu.a, 0x00);
        assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
        assert!(gb.cpu.f & Gbz80::FLAG_C != 0);

        // Subtraction examples
        // 0x42 - 0x27 = 0x1B -> 0x15 (H=1, N=1)
        gb.cpu.a = 0x1B;
        gb.cpu.set_flags(false, true, true, false); // Z=0, N=1, H=1, C=0
        gb.daa(1 as u8);
        assert_eq!(gb.cpu.a, 0x15);
    }

    #[test]
    fn test_cpl() {
        let mut gb = Gameboy::new();
        gb.cpu.a = 0x35;
        gb.cpl(1 as u8);
        assert_eq!(gb.cpu.a, 0xCA);
        assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H != 0);
    }

    #[test]
    fn test_scf() {
        let mut gb = Gameboy::new();
        gb.cpu.set_flag(Gbz80::FLAG_C, false);
        gb.cpu.set_flag(Gbz80::FLAG_N, true);
        gb.cpu.set_flag(Gbz80::FLAG_H, true);
        gb.scf(1 as u8);
        assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
        assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
    }

    #[test]
    fn test_ccf() {
        let mut gb = Gameboy::new();
        gb.cpu.set_flag(Gbz80::FLAG_C, true);
        gb.cpu.set_flag(Gbz80::FLAG_N, true);
        gb.cpu.set_flag(Gbz80::FLAG_H, true);
        gb.ccf(1 as u8);
        assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H == 0);

        gb.ccf(1 as u8);
        assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
    }

    #[test]
    fn test_di_ei() {
        let mut gb = Gameboy::new();
        gb.di(1 as u8);
        // Since IME is not implemented in Gbz80 struct, we just call the method
        // to ensure it doesn't crash and is available.
        gb.ei(1 as u8);
    }

    #[test]
    fn test_halt() {
        let mut gb = Gameboy::new();
        gb.running = true;
        gb.halt();
        assert_eq!(gb.running, false);
    }

    #[test]
    fn test_stop() {
        let mut gb = Gameboy::new();
        gb.stop(1 as u8);
        // STOP usually also halts the CPU until a button is pressed.
        // For now, we just ensure it exists.
    }
}
