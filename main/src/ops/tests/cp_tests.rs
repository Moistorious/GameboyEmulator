use crate::cpu::{Reg8, Gbz80};
use crate::gameboy::Gameboy;

#[test]
fn test_cp_r8() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x30;
    gb.cpu.b = 0x30;

    gb.cp(0xB8); // CP B

    assert_eq!(gb.cpu.a, 0x30); // A should remain unchanged
    assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_cp_all_r8() {
    let regs = [Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L];
    for &reg in regs.iter() {
        let mut gb = Gameboy::new();
        let opcode = 0xB8 + (reg as u8);
        gb.cpu.a = 0x30;
        gb.cpu.write_reg8(reg, 0x10);

        gb.cp(opcode);

        assert_eq!(gb.cpu.a, 0x30, "CP {:?} must not modify A", reg);
        assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
    }
}

#[test]
fn test_cp_a_hl_mem() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x30;
    gb.cpu.set_hl(0xC000);
    gb.memory.write_u8(0xC000, 0x30);

    gb.cp(0xBE); // CP (HL)

    assert_eq!(gb.cpu.a, 0x30);
    assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_cp_a_n8() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x30;
    gb.cpu.program_counter = 0x100;
    gb.memory.write_u8(0x101, 0x10);

    gb.cp(0xFE); // CP n

    assert_eq!(gb.cpu.a, 0x30);
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_cp_flags() {
    let mut gb = Gameboy::new();

    // Half borrow: 0x10 - 0x01 = 0x0F -> H=1
    gb.cpu.a = 0x10;
    gb.cpu.b = 0x01;
    gb.cp(0xB8);
    assert!(gb.cpu.f & Gbz80::FLAG_H != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);

    // Borrow: 0x00 - 0x01 -> C=1
    gb.cpu.a = 0x00;
    gb.cpu.b = 0x01;
    gb.cp(0xB8);
    assert!(gb.cpu.f & Gbz80::FLAG_C != 0);

    // N must always be set by CP
    assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
}
