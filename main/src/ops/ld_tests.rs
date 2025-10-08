
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
                // Skip LD (HL),(HL) because opcode 0x76 is HALT
                if dest == Reg8::L && src == Reg8::L || dest == src {
                    continue;
                }

                let opcode = ld_opcode(dest, src);
                if opcode == 0x46 {
                    assert_eq!(1, 2);
                }

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
            gb.memory.write_u8(0, 0x99); // pretend immediate at PC+1

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

    // You’d then continue with LD A,(nn), LD (nn),A, LD A,(C), LD (C),A, etc.
}