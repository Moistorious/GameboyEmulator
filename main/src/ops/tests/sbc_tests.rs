use crate::cpu::{Reg8, Gbz80};
use crate::gameboy::Gameboy;

#[test]
fn test_sbc_a_r8() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x30;
    gb.cpu.b = 0x10;
    gb.cpu.set_flag(Gbz80::FLAG_C, true);

    gb.sbc(0x98); // SBC A, B

    assert_eq!(gb.cpu.a, 0x1F);
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_sbc_a_r8_no_carry_in() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x30;
    gb.cpu.b = 0x10;
    gb.cpu.set_flag(Gbz80::FLAG_C, false);

    gb.sbc(0x98); // SBC A, B

    assert_eq!(gb.cpu.a, 0x20);
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_sbc_all_r8() {
    let regs = [Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L];
    for &reg in regs.iter() {
        let mut gb = Gameboy::new();
        let opcode = 0x98 + (reg as u8);
        gb.cpu.a = 0x30;
        gb.cpu.write_reg8(reg, 0x10);
        gb.cpu.set_flag(Gbz80::FLAG_C, false);

        gb.sbc(opcode);

        assert_eq!(gb.cpu.a, 0x20, "SBC A,{:?} failed", reg);
        assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
    }
}

#[test]
fn test_sbc_a_hl_mem() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x30;
    gb.cpu.set_hl(0xC000);
    gb.memory.write_u8(0xC000, 0x10);
    gb.cpu.set_flag(Gbz80::FLAG_C, false);

    gb.sbc(0x9E); // SBC A, (HL)

    assert_eq!(gb.cpu.a, 0x20);
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_sbc_a_n8() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x30;
    gb.cpu.program_counter = 0x100;
    gb.memory.write_u8(0x101, 0x10);
    gb.cpu.set_flag(Gbz80::FLAG_C, false);

    gb.sbc(0xDE); // SBC A, n

    assert_eq!(gb.cpu.a, 0x20);
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_sbc_flags() {
    let mut gb = Gameboy::new();

    // Zero flag: 0x01 - 0x00 - carry 1 = 0x00 -> Z=1, N=1
    gb.cpu.a = 0x01;
    gb.cpu.b = 0x00;
    gb.cpu.set_flag(Gbz80::FLAG_C, true);
    gb.sbc(0x98);
    assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N != 0);

    // Half borrow: 0x10 - 0x01 - carry 0 = 0x0F -> H=1
    gb.cpu.a = 0x10;
    gb.cpu.b = 0x01;
    gb.cpu.set_flag(Gbz80::FLAG_C, false);
    gb.sbc(0x98);
    assert!(gb.cpu.f & Gbz80::FLAG_H != 0);

    // Carry out: 0x00 - 0x01 - carry 0 -> C=1
    gb.cpu.a = 0x00;
    gb.cpu.b = 0x01;
    gb.cpu.set_flag(Gbz80::FLAG_C, false);
    gb.sbc(0x98);
    assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
}
