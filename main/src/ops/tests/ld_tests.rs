use crate::cpu::{Reg16, Reg8, Gbz80};
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
            gameboy.ld(opcode).unwrap();

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

        gb.ld(opcode).unwrap();

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

        gb.ld(opcode).unwrap();

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

        gb.ld(opcode).unwrap();

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

        gb.ld(opcode).unwrap();

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

        gb.ld(opcode).unwrap();

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
    gb.ld(LD_HLI_A).unwrap();

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
    gb.ld(LD_HLD_A).unwrap();

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
    gb.ld(LD_HLI_A).unwrap();

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
    gb.ld(LD_HLD_A).unwrap();

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

        gb.ld(opcode).unwrap();

        assert_eq!(gb.cpu.reg16(pair), 0xABCD, "LD {:?},nn failed", pair);
        assert_eq!(gb.cpu.program_counter, 2, "PC should have incremented by 2");
    }
}

#[test]
fn test_ld_sp_nn() {
    let mut gb = Gameboy::new();
    gb.cpu.program_counter = 0;
    gb.memory.write_u16(0, 0xFFFE);

    gb.ld(0x31).unwrap(); // LD SP, nn

    assert_eq!(gb.cpu.stack_pointer, 0xFFFE, "LD SP,nn failed");
    assert_eq!(gb.cpu.program_counter, 2, "PC should have incremented by 2");
}

#[test]
fn test_ld_a_nn() {
    let mut gb = Gameboy::new();
    gb.cpu.program_counter = 0;
    gb.memory.write_u16(0, 0xC000); // 16-bit address
    gb.memory.write_u8(0xC000, 0x77); // value at that address

    gb.ld(0xFA).unwrap(); // LD A,(nn)

    assert_eq!(gb.cpu.reg8(Reg8::A), 0x77, "LD A,(nn) failed");
    assert_eq!(gb.cpu.program_counter, 2, "PC should have incremented by 2");
}

#[test]
fn test_ld_nn_a() {
    let mut gb = Gameboy::new();
    gb.cpu.program_counter = 0;
    gb.cpu.write_reg8(Reg8::A, 0x88);
    gb.memory.write_u16(0, 0xC000); // destination address

    gb.ld(0xEA).unwrap(); // LD (nn),A

    assert_eq!(gb.memory.read_u8(0xC000), 0x88, "LD (nn),A failed");
    assert_eq!(gb.cpu.program_counter, 2, "PC should have incremented by 2");
}

#[test]
fn test_ld_a_hli() {
    let mut gb = Gameboy::new();
    gb.cpu.write_reg16(Reg16::HL, 0xC000);
    gb.memory.write_u8(0xC000, 0x44);

    gb.ld(0x2A).unwrap(); // LD A,(HL+)

    assert_eq!(gb.cpu.reg8(Reg8::A), 0x44);
    assert_eq!(gb.cpu.hl(), 0xC001);
}

#[test]
fn test_ld_a_hld() {
    let mut gb = Gameboy::new();
    gb.cpu.write_reg16(Reg16::HL, 0xC005);
    gb.memory.write_u8(0xC005, 0x55);

    gb.ld(0x3A).unwrap(); // LD A,(HL-)

    assert_eq!(gb.cpu.reg8(Reg8::A), 0x55);
    assert_eq!(gb.cpu.hl(), 0xC004);
}

#[test]
fn test_ld_a_c() {
    let mut gb = Gameboy::new();
    gb.cpu.write_reg8(Reg8::C, 0x10);
    gb.memory.write_u8(0xFF10, 0x66);

    gb.ld(0xF2).unwrap(); // LD A,(C)

    assert_eq!(gb.cpu.reg8(Reg8::A), 0x66);
}

#[test]
fn test_ld_c_a() {
    let mut gb = Gameboy::new();
    gb.cpu.write_reg8(Reg8::C, 0x20);
    gb.cpu.write_reg8(Reg8::A, 0x77);

    gb.ld(0xE2).unwrap(); // LD (C),A

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

        gb.ld(0xF8).unwrap(); // LD HL,SP+e8

        assert_eq!(gb.cpu.hl(), 0x1010);
        assert_eq!(gb.cpu.program_counter, 1);
        // flags: Z=0, N=0, H=0, C=0
        assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
    }
    // Negative offset
    {
        let mut gb = Gameboy::new();
        gb.cpu.stack_pointer = 0x1000;
        gb.cpu.program_counter = 0;
        gb.memory.write_u8(0, 0xF0); // Offset = -16 (0xF0)

        gb.ld(0xF8).unwrap(); // LD HL,SP+e8

        assert_eq!(gb.cpu.hl(), 0x0FF0);
        assert_eq!(gb.cpu.program_counter, 1);
        assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
    }
}

#[test]
fn test_ld_hl_sp_e8_flags() {
    // Half carry: SP + e8 where low nibbles carry. 0x000F + 0x01 = 0x0010 -> H=1
    let mut gb = Gameboy::new();
    gb.cpu.stack_pointer = 0x000F;
    gb.cpu.program_counter = 0;
    gb.memory.write_u8(0, 0x01);
    gb.ld(0xF8).unwrap();
    assert_eq!(gb.cpu.hl(), 0x0010);
    assert!(gb.cpu.f & Gbz80::FLAG_H != 0);

    // Carry: 0xFFFF + 0x01 = 0x0000 -> C=1
    let mut gb2 = Gameboy::new();
    gb2.cpu.stack_pointer = 0xFFFF;
    gb2.cpu.program_counter = 0;
    gb2.memory.write_u8(0, 0x01);
    gb2.ld(0xF8).unwrap();
    assert_eq!(gb2.cpu.hl(), 0x0000);
    assert!(gb2.cpu.f & Gbz80::FLAG_C != 0);

    // No half carry, no carry: 0x0010 + 0x01 = 0x0011
    let mut gb3 = Gameboy::new();
    gb3.cpu.stack_pointer = 0x0010;
    gb3.cpu.program_counter = 0;
    gb3.memory.write_u8(0, 0x01);
    gb3.ld(0xF8).unwrap();
    assert!(gb3.cpu.f & Gbz80::FLAG_H == 0);
    assert!(gb3.cpu.f & Gbz80::FLAG_C == 0);
    assert!(gb3.cpu.f & Gbz80::FLAG_N == 0);
}

#[test]
fn test_ld_sp_hl() {
    let mut gb = Gameboy::new();
    gb.cpu.write_reg16(Reg16::HL, 0x55AA);

    gb.ld(0xF9).unwrap(); // LD SP,HL

    assert_eq!(gb.cpu.stack_pointer, 0x55AA);
}

#[test]
fn test_ld_nn_sp() {
    let mut gb = Gameboy::new();
    gb.cpu.stack_pointer = 0x9988;
    gb.cpu.program_counter = 0;
    gb.memory.write_u16(0, 0xC500);

    gb.ld(0x08).unwrap(); // LD (nn),SP

    assert_eq!(gb.memory.read_u16(0xC500), 0x9988);
    assert_eq!(gb.cpu.program_counter, 2);
}

#[test]
fn test_ld_hl_mem_n8() {
    let mut gb = Gameboy::new();
    gb.cpu.write_reg16(Reg16::HL, 0xC000);
    gb.cpu.program_counter = 0;
    gb.memory.write_u8(0, 0xBC);

    gb.ld(0x36).unwrap(); // LD (HL),n

    assert_eq!(gb.memory.read_u8(0xC000), 0xBC);
    assert_eq!(gb.cpu.program_counter, 1);
}

//
// HALT via LD (HL),(HL) (0x76)
//

#[test]
fn test_ld_hl_hl_halt() {
    let mut gb = Gameboy::new();
    gb.running = true;

    gb.ld(0x76).unwrap(); // LD (HL),(HL) decodes to HALT

    assert_eq!(gb.running, false, "LD (HL),(HL) should halt the CPU");
}

//
// LD must never modify flags
//

#[test]
fn test_ld_preserves_flags() {
    // Representative LD opcodes across the surface
    let opcodes = [0x40, 0x06, 0x01, 0x0A, 0x02, 0x22, 0xFA, 0xEA, 0x2A, 0xF2, 0xE2, 0xF9];

    for opcode in opcodes {
        let mut gb = Gameboy::new();
        gb.cpu.a = 0x11;
        gb.cpu.b = 0x22;
        gb.cpu.c = 0x10;
        gb.cpu.write_reg16(Reg16::BC, 0xC000);
        gb.cpu.write_reg16(Reg16::DE, 0xC001);
        gb.cpu.write_reg16(Reg16::HL, 0xC000);
        gb.cpu.program_counter = 0x100;
        gb.memory.write_u16(0x100, 0xC000);
        gb.memory.write_u8(0xC000, 0x55);
        gb.memory.write_u8(0xFF10, 0x66);

        gb.cpu.f = 0xF0;
        let flags_before = gb.cpu.f;

        gb.ld(opcode).unwrap();

        assert_eq!(gb.cpu.f, flags_before, "LD (0x{:02X}) modified flags", opcode);
    }
}

//
// LD r,r with (HL) as source and/or destination through the ld_r_r path
//

#[test]
fn test_ld_r_r_hl_as_source() {
    // LD B,(HL) = 0x46, LD C,(HL) = 0x4E, ... LD A,(HL) = 0x7E
    let regs = [Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L, Reg8::A];
    for &dest in &regs {
        let mut gb = Gameboy::new();
        let opcode = 0x46 | ((dest as u8) << 3);
        gb.cpu.write_reg16(Reg16::HL, 0xC000);
        gb.memory.write_u8(0xC000, 0xAB);

        gb.ld(opcode).unwrap();

        assert_eq!(gb.cpu.reg8(dest), 0xAB, "LD {:?},(HL) via ld_r_r failed", dest);
    }
}

#[test]
fn test_ld_r_r_hl_as_destination() {
    // LD (HL),B = 0x70, LD (HL),C = 0x71, ... LD (HL),A = 0x77
    // The H/L source cases are tested separately (see below) because the
    // source value is H or L itself.
    let regs = [Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::A];
    for &src in &regs {
        let mut gb = Gameboy::new();
        let opcode = 0x70 | (src as u8);
        gb.cpu.write_reg16(Reg16::HL, 0xC000);
        gb.cpu.write_reg8(src, 0x5A);

        gb.ld(opcode).unwrap();

        assert_eq!(gb.memory.read_u8(0xC000), 0x5A, "LD (HL),{:?} via ld_r_r failed", src);
    }
}

#[test]
fn test_ld_hl_h() {
    // LD (HL),H = 0x74: writes H's value to address (HL) without changing H
    let mut gb = Gameboy::new();
    gb.cpu.write_reg16(Reg16::HL, 0xC000); // H=0xC0, L=0x00
    gb.ld(0x74).unwrap();
    assert_eq!(gb.memory.read_u8(0xC000), 0xC0);
    assert_eq!(gb.cpu.hl(), 0xC000, "HL must be unchanged");
}

#[test]
fn test_ld_hl_l() {
    // LD (HL),L = 0x75: writes L's value to address (HL) without changing L
    let mut gb = Gameboy::new();
    gb.cpu.write_reg16(Reg16::HL, 0xC012); // H=0xC0, L=0x12
    gb.ld(0x75).unwrap();
    assert_eq!(gb.memory.read_u8(0xC012), 0x12);
    assert_eq!(gb.cpu.hl(), 0xC012, "HL must be unchanged");
}

//
// LD A,(nn) / LD (nn),A / LD (nn),SP little-endian write ordering
//

#[test]
fn test_ld_a_nn_little_endian() {
    let mut gb = Gameboy::new();
    gb.cpu.program_counter = 0x4000;
    gb.memory.write_u8(0x4000, 0x34); // low byte first
    gb.memory.write_u8(0x4001, 0x12); // high byte
    gb.memory.write_u8(0x1234, 0x77);

    gb.ld(0xFA).unwrap(); // LD A,(nn)

    assert_eq!(gb.cpu.reg8(Reg8::A), 0x77);
}

#[test]
fn test_ld_nn_a_little_endian() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x88;
    gb.cpu.program_counter = 0x4000;
    gb.memory.write_u8(0x4000, 0x50);
    gb.memory.write_u8(0x4001, 0xC1);

    gb.ld(0xEA).unwrap(); // LD (nn),A

    assert_eq!(gb.memory.read_u8(0xC150), 0x88);
}

//
// LD A,(HL+) / LD A,(HL-) use wrapping arithmetic
//

#[test]
fn test_ld_a_hli_wrapping() {
    let mut gb = Gameboy::new();
    gb.cpu.write_reg16(Reg16::HL, 0xFFFF);
    gb.memory.write_u8(0xFFFF, 0x7A);

    gb.ld(0x2A).unwrap(); // LD A,(HL+)

    assert_eq!(gb.cpu.reg8(Reg8::A), 0x7A);
    // HL+ wraps 0xFFFF -> 0x0000
    assert_eq!(gb.cpu.hl(), 0x0000);
}

#[test]
fn test_ld_a_hld_wrapping() {
    let mut gb = Gameboy::new();
    gb.cpu.write_reg16(Reg16::HL, 0x0000);
    gb.memory.write_u8(0x0000, 0x39);

    gb.ld(0x3A).unwrap(); // LD A,(HL-)

    assert_eq!(gb.cpu.reg8(Reg8::A), 0x39);
    // HL- wraps 0x0000 -> 0xFFFF
    assert_eq!(gb.cpu.hl(), 0xFFFF);
}

#[test]
fn test_ld_a_hli_edge() {
    let mut gb = Gameboy::new();
    gb.cpu.write_reg16(Reg16::HL, 0xFFFF - 1);
    gb.memory.write_u8(0xFFFE, 0x7A);

    gb.ld(0x2A).unwrap(); // LD A,(HL+)

    assert_eq!(gb.cpu.reg8(Reg8::A), 0x7A);
    assert_eq!(gb.cpu.hl(), 0xFFFF);
}

#[test]
fn test_ld_a_hld_edge() {
    let mut gb = Gameboy::new();
    gb.cpu.write_reg16(Reg16::HL, 0x0001);
    gb.memory.write_u8(0x0001, 0x39);

    gb.ld(0x3A).unwrap(); // LD A,(HL-)

    assert_eq!(gb.cpu.reg8(Reg8::A), 0x39);
    assert_eq!(gb.cpu.hl(), 0x0000);
}

//
// LD (HL+),A / LD (HL-),A write before increment (correct operand order)
//

#[test]
fn test_ld_hli_a_uses_original_address() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x12;
    gb.cpu.write_reg16(Reg16::HL, 0xC100);

    gb.ld(0x22).unwrap(); // LD (HL+),A

    // Value must be written at ORIGINAL HL, then HL incremented
    assert_eq!(gb.memory.read_u8(0xC100), 0x12);
    assert_eq!(gb.cpu.hl(), 0xC101);
}

#[test]
fn test_ld_hld_a_uses_original_address() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x34;
    gb.cpu.write_reg16(Reg16::HL, 0xC100);

    gb.ld(0x32).unwrap(); // LD (HL-),A

    assert_eq!(gb.memory.read_u8(0xC100), 0x34);
    assert_eq!(gb.cpu.hl(), 0xC0FF);
}

//
// LD HL,SP+e8 (0xF8) — full flag + wrap coverage
//

#[test]
fn test_ld_hl_sp_e8_extremes() {
    // offset 0: HL == SP exactly
    {
        let mut gb = Gameboy::new();
        gb.cpu.stack_pointer = 0x1234;
        gb.cpu.program_counter = 0;
        gb.memory.write_u8(0, 0x00);

        gb.ld(0xF8).unwrap();

        assert_eq!(gb.cpu.hl(), 0x1234);
        assert_eq!(gb.cpu.program_counter, 1);
    }
    // max positive offset (+127)
    {
        let mut gb = Gameboy::new();
        gb.cpu.stack_pointer = 0x1000;
        gb.cpu.program_counter = 0;
        gb.memory.write_u8(0, 0x7F);

        gb.ld(0xF8).unwrap();

        assert_eq!(gb.cpu.hl(), 0x107F);
    }
    // max negative offset (-128)
    {
        let mut gb = Gameboy::new();
        gb.cpu.stack_pointer = 0x1000;
        gb.cpu.program_counter = 0;
        gb.memory.write_u8(0, 0x80);

        gb.ld(0xF8).unwrap();

        assert_eq!(gb.cpu.hl(), 0x0F80);
    }
}

//
// LD rr,nn / LD SP,nn set the full 16-bit register from a little-endian immediate
//

#[test]
fn test_ld_rr_nn_little_endian() {
    let mut gb = Gameboy::new();
    gb.cpu.program_counter = 0x4000;
    gb.memory.write_u8(0x4000, 0xEF); // low byte
    gb.memory.write_u8(0x4001, 0xBE); // high byte

    gb.ld(0x21).unwrap(); // LD HL,nn

    assert_eq!(gb.cpu.hl(), 0xBEEF);
}
