use crate::cpu::{Reg8, Gbz80};
use crate::gameboy::Gameboy;

#[test]
fn test_sub_a_r8() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x30;
    gb.cpu.b = 0x10;

    gb.sub(0x90); // SUB A, B

    assert_eq!(gb.cpu.a, 0x20);
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_sub_all_r8() {
    let regs = [Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L, Reg8::A];
    for &reg in regs.iter() {
        let mut gb = Gameboy::new();
        let opcode = 0x90 + (reg as u8);
        gb.cpu.a = 0x30;
        gb.cpu.write_reg8(reg, 0x10);

        gb.sub(opcode);

        assert_eq!(gb.cpu.a, 0x20, "SUB A,{:?} failed", reg);
        assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
    }
}

#[test]
fn test_sub_a_hl_mem() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x30;
    gb.cpu.set_hl(0xC000);
    gb.memory.write_u8(0xC000, 0x10);

    gb.sub(0x96); // SUB A, (HL)

    assert_eq!(gb.cpu.a, 0x20);
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_sub_a_n8() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x30;
    gb.cpu.program_counter = 0x100;
    gb.memory.write_u8(0x101, 0x10);

    gb.sub(0xD6); // SUB A, n

    assert_eq!(gb.cpu.a, 0x20);
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_sub_flags() {
    let mut gb = Gameboy::new();

    // Zero flag
    gb.cpu.a = 0x10;
    gb.cpu.b = 0x10;
    gb.sub(0x90);
    assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N != 0);

    // Half carry (borrow from bit 4)
    gb.cpu.a = 0x10;
    gb.cpu.b = 0x01;
    gb.sub(0x90);
    assert!(gb.cpu.f & Gbz80::FLAG_H != 0);

    // Carry flag (borrow)
    gb.cpu.a = 0x00;
    gb.cpu.b = 0x01;
    gb.sub(0x90);
    assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
}
