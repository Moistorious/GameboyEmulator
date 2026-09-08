
#[cfg(test)]
mod ld_tests {
    use crate::cpu::{Reg16, Reg8};
    use crate::gameboy::Gameboy;

    fn ld_opcode(dest: Reg8, src: Reg8) -> u8 {
        0x40 | ((dest as u8) << 3) | (src as u8)
    }

    #[test]
    fn test_all_ld_reg8_to_reg8_instructions() {
        let regs = [
            Reg8::B,
            Reg8::C,
            Reg8::D,
            Reg8::E,
            Reg8::H,
            Reg8::L,
            Reg8::A,
        ];

        for &dest in &regs {
            for &src in &regs {
                if dest == src {
                    continue;
                }

                let opcode = ld_opcode(dest, src);
                let mut gameboy = Gameboy::new();

                // initialize dest with 0x00 and src with 0xAB
                gameboy.cpu.write_reg8(dest, 0x00);
                gameboy.cpu.write_reg8(src, 0xAB);

                // execute LD dest,src
                gameboy.ld(opcode);

                // dest should equal src after the LD
                assert_eq!(
                    gameboy.cpu.reg8(dest),
                    0xAB,
                    "LD {:?},{:?} (opcode 0x{:02X}) failed",
                    dest,
                    src,
                    opcode
                );
            }
        }
    }

    //
    // LD r,(HL)
    //
    #[test]
    fn test_ld_r_from_hl() {
        let regs = [
            Reg8::B,
            Reg8::C,
            Reg8::D,
            Reg8::E,
            Reg8::H,
            Reg8::L,
            Reg8::A,
        ];

        for &dest in &regs {
            let opcode = 0x46 | ((dest as u8) << 3); // LD r,(HL)

            let mut gb = Gameboy::new();
            gb.cpu.write_reg16(Reg16::HL, 0x1234);
            gb.memory.write_u8(0x1234, 0x5A);

            gb.ld(opcode);

            assert_eq!(gb.cpu.reg8(dest), 0x5A, "LD {:?},(HL) failed", dest);
        }
    }

    //
    // LD (HL),r
    //
    #[test]
    fn test_ld_hl_from_r() {
        let regs = [
            Reg8::B,
            Reg8::C,
            Reg8::D,
            Reg8::E,
            Reg8::H,
            Reg8::L,
            Reg8::A,
        ];

        for &src in &regs {
            let opcode = 0x70 | (src as u8); // LD (HL),r

            let mut gb = Gameboy::new();
            gb.cpu.write_reg16(Reg16::HL, 0x1F1F);
            gb.cpu.write_reg8(src, 0x1F);

            gb.ld(opcode);

            assert_eq!(gb.memory.read_u8(0x1F1F), 0x1F, "LD (HL),{:?} failed", src);
        }
    }

    //
    // LD r,n (immediate)
    //
    #[test]
    fn test_ld_r_n() {
        let tests = [
            (Reg8::B, 0x06),
            (Reg8::C, 0x0E),
            (Reg8::D, 0x16),
            (Reg8::E, 0x1E),
            (Reg8::H, 0x26),
            (Reg8::L, 0x2E),
            (Reg8::A, 0x3E),
        ];

        for &(reg, opcode) in &tests {
            let mut gb = Gameboy::new();
            gb.cpu.program_counter = 0;
            gb.memory.write_u8(0, 0x99); // pretend immediate at PC

            gb.ld(opcode);

            assert_eq!(gb.cpu.reg8(reg), 0x99, "LD {:?},n failed", reg);
        }
    }

    //
    // LD A,(BC) and LD A,(DE)
    //
    #[test]
    fn test_ld_a_from_bc_de() {
        let tests = [(0x0A, Reg16::BC), (0x1A, Reg16::DE)];

        for &(opcode, pair) in &tests {
            let mut gb = Gameboy::new();
            gb.cpu.write_reg16(pair, 0x1FFF);
            gb.memory.write_u8(0x1FFF, 0x55);

            gb.ld(opcode);

            assert_eq!(gb.cpu.reg8(Reg8::A), 0x55, "LD A,({:?}) failed", pair);
        }
    }

    //
    // LD (BC),A and LD (DE),A
    //
    #[test]
    fn test_ld_bc_de_from_a() {
        let tests = [(0x02, Reg16::BC), (0x12, Reg16::DE)];

        for &(opcode, pair) in &tests {
            let mut gb = Gameboy::new();
            gb.cpu.write_reg16(pair, 0x1FFF);
            gb.cpu.write_reg8(Reg8::A, 0x66);

            gb.ld(opcode);

            assert_eq!(gb.memory.read_u8(0x1FFF), 0x66, "LD ({:?}),A failed", pair);
        }
    }

    const LD_HLI_A: u8 = 0x22; // LD (HL+), A
    const LD_HLD_A: u8 = 0x32; // LD (HL-), A

    #[test]
    fn test_ld_hli_a_stores_a_and_increments_hl() {
        let mut gb = Gameboy::new();

        // Setup registers
        gb.cpu.a = 0xAB;
        gb.cpu.write_reg16(Reg16::HL, 0xC000);
        let hl_before = gb.cpu.hl();

        // Execute
        gb.ld(LD_HLI_A);

        // Check that A was written to memory at address HL
        let written = gb.memory.read_u8(hl_before);
        assert_eq!(written, 0xAB, "Memory at HL should contain A’s value");

        // Check that HL incremented
        assert_eq!(gb.cpu.hl(), hl_before + 1, "HL should have incremented by 1");
    }

    #[test]
    fn test_ld_hld_a_stores_a_and_decrements_hl() {
        let mut gb = Gameboy::new();

        // Setup registers
        gb.cpu.a = 0xAB;
        gb.cpu.write_reg16(Reg16::HL, 0xC010);
        let hl_before = gb.cpu.hl();

        // Execute
        gb.ld(LD_HLD_A);

        // Check that A was written to memory at address HL
        let written = gb.memory.read_u8(hl_before);
        assert_eq!(written, 0xAB, "Memory at HL should contain A’s value");

        // Check that HL decremented
        assert_eq!(gb.cpu.hl(), hl_before - 1, "HL should have decremented by 1");
    }

    //
    // New coverage & edge case tests
    //

    #[test]
    fn test_ld_hli_a_wrapping() {
        let mut gb = Gameboy::new();
        gb.cpu.a = 0xAB;
        gb.cpu.write_reg16(Reg16::HL, 0xFFFF);

        // Execute LD (HL+), A
        gb.ld(LD_HLI_A);

        // Check value and wrapping
        assert_eq!(gb.memory.read_u8(0xFFFF), 0xAB);
        assert_eq!(gb.cpu.hl(), 0x0000);
    }

    #[test]
    fn test_ld_hld_a_wrapping() {
        let mut gb = Gameboy::new();
        gb.cpu.a = 0xAB;
        gb.cpu.write_reg16(Reg16::HL, 0x0000);

        // Execute LD (HL-), A
        gb.ld(LD_HLD_A);

        // Check value and wrapping
        assert_eq!(gb.memory.read_u8(0x0000), 0xAB);
        assert_eq!(gb.cpu.hl(), 0xFFFF);
    }

    #[test]
    fn test_ld_rr_nn() {
        let tests = [
            (0x01, Reg16::BC),
            (0x11, Reg16::DE),
            (0x21, Reg16::HL),
        ];

        for &(opcode, pair) in &tests {
            let mut gb = Gameboy::new();
            gb.cpu.program_counter = 0;
            // Write 0xABCD as the 16-bit immediate value at PC
            gb.memory.write_u16(0, 0xABCD);

            gb.ld(opcode);

            assert_eq!(gb.cpu.reg16(pair), 0xABCD, "LD {:?},nn failed", pair);
            assert_eq!(gb.cpu.program_counter, 2, "PC should have incremented by 2");
        }
    }

    #[test]
    fn test_ld_sp_nn() {
        let mut gb = Gameboy::new();
        gb.cpu.program_counter = 0;
        gb.memory.write_u16(0, 0xFFFE);

        gb.ld(0x31); // LD SP, nn

        assert_eq!(gb.cpu.stack_pointer, 0xFFFE, "LD SP,nn failed");
        assert_eq!(gb.cpu.program_counter, 2, "PC should have incremented by 2");
    }

    #[test]
    fn test_ld_a_nn() {
        let mut gb = Gameboy::new();
        gb.cpu.program_counter = 0;
        gb.memory.write_u16(0, 0xC000); // 16-bit address
        gb.memory.write_u8(0xC000, 0x77); // value at that address

        gb.ld(0xFA); // LD A,(nn)

        assert_eq!(gb.cpu.reg8(Reg8::A), 0x77, "LD A,(nn) failed");
        assert_eq!(gb.cpu.program_counter, 2, "PC should have incremented by 2");
    }

    #[test]
    fn test_ld_nn_a() {
        let mut gb = Gameboy::new();
        gb.cpu.program_counter = 0;
        gb.cpu.write_reg8(Reg8::A, 0x88);
        gb.memory.write_u16(0, 0xC000); // destination address

        gb.ld(0xEA); // LD (nn),A

        assert_eq!(gb.memory.read_u8(0xC000), 0x88, "LD (nn),A failed");
        assert_eq!(gb.cpu.program_counter, 2, "PC should have incremented by 2");
    }

    #[test]
    fn test_ld_a_hli() {
        let mut gb = Gameboy::new();
        gb.cpu.write_reg16(Reg16::HL, 0xC000);
        gb.memory.write_u8(0xC000, 0x44);

        gb.ld(0x2A); // LD A,(HL+)

        assert_eq!(gb.cpu.reg8(Reg8::A), 0x44);
        assert_eq!(gb.cpu.hl(), 0xC001);
    }

    #[test]
    fn test_ld_a_hld() {
        let mut gb = Gameboy::new();
        gb.cpu.write_reg16(Reg16::HL, 0xC005);
        gb.memory.write_u8(0xC005, 0x55);

        gb.ld(0x3A); // LD A,(HL-)

        assert_eq!(gb.cpu.reg8(Reg8::A), 0x55);
        assert_eq!(gb.cpu.hl(), 0xC004);
    }

    #[test]
    fn test_ld_a_c() {
        let mut gb = Gameboy::new();
        gb.cpu.write_reg8(Reg8::C, 0x10);
        gb.memory.write_u8(0xFF10, 0x66);

        gb.ld(0xF2); // LD A,(C)

        assert_eq!(gb.cpu.reg8(Reg8::A), 0x66);
    }

    #[test]
    fn test_ld_c_a() {
        let mut gb = Gameboy::new();
        gb.cpu.write_reg8(Reg8::C, 0x20);
        gb.cpu.write_reg8(Reg8::A, 0x77);

        gb.ld(0xE2); // LD (C),A

        assert_eq!(gb.memory.read_u8(0xFF20), 0x77);
    }

    #[test]
    fn test_ld_hl_sp_e8() {
        // Positive offset
        {
            let mut gb = Gameboy::new();
            gb.cpu.stack_pointer = 0x1000;
            gb.cpu.program_counter = 0;
            gb.memory.write_u8(0, 0x10); // Offset = 16

            gb.ld(0xF8); // LD HL,SP+e8

            assert_eq!(gb.cpu.hl(), 0x1010);
            assert_eq!(gb.cpu.program_counter, 1);
            // check flags (Z=0, N=0, H=0, C=0)
            assert_eq!(gb.cpu.reg8(Reg8::F) & 0xF0, 0);
        }
        // Negative offset
        {
            let mut gb = Gameboy::new();
            gb.cpu.stack_pointer = 0x1000;
            gb.cpu.program_counter = 0;
            gb.memory.write_u8(0, 0xF0); // Offset = -16 (0xF0)

            gb.ld(0xF8); // LD HL,SP+e8

            assert_eq!(gb.cpu.hl(), 0x0FF0);
            assert_eq!(gb.cpu.program_counter, 1);
        }
    }

    #[test]
    fn test_ld_sp_hl() {
        let mut gb = Gameboy::new();
        gb.cpu.write_reg16(Reg16::HL, 0x55AA);

        gb.ld(0xF9); // LD SP,HL

        assert_eq!(gb.cpu.stack_pointer, 0x55AA);
    }

    #[test]
    fn test_ld_nn_sp() {
        let mut gb = Gameboy::new();
        gb.cpu.stack_pointer = 0x9988;
        gb.cpu.program_counter = 0;
        gb.memory.write_u16(0, 0xC500);

        gb.ld(0x08); // LD (nn),SP

        assert_eq!(gb.memory.read_u16(0xC500), 0x9988);
        assert_eq!(gb.cpu.program_counter, 2);
    }

    #[test]
    fn test_ld_hl_mem_n8() {
        let mut gb = Gameboy::new();
        gb.cpu.write_reg16(Reg16::HL, 0xC000);
        gb.cpu.program_counter = 0;
        gb.memory.write_u8(0, 0xBC);

        gb.ld(0x36); // LD (HL),n

        assert_eq!(gb.memory.read_u8(0xC000), 0xBC);
        assert_eq!(gb.cpu.program_counter, 1);
    }
}