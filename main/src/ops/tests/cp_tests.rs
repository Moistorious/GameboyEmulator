use crate::cpu::{Reg8, Gbz80};
use crate::gameboy::Gameboy;

// CP r     = 0xB8 + r
// CP (HL)  = 0xBE
// CP n     = 0xFE
// CP = A - n without storing result; always sets N=1

#[test]
fn test_cp_r8() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x30;
    gb.cpu.b = 0x30;

    gb.cp(0xB8).unwrap();

    assert_eq!(gb.cpu.a, 0x30, "CP must not modify A");
    assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_cp_all_r8() {
    let regs = [Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L, Reg8::A];
    for &reg in regs.iter() {
        let mut gb = Gameboy::new();
        let opcode = 0xB8 + (reg as u8);
        gb.cpu.a = 0x30;
        gb.cpu.write_reg8(reg, 0x10);

        gb.cp(opcode).unwrap();

        assert_eq!(gb.cpu.a, 0x30, "CP {:?} must not modify A", reg);
        assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
        if reg == Reg8::A {
            assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
        } else {
            assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
        }
    }
}

#[test]
fn test_cp_a_hl_mem() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x30;
    gb.cpu.set_hl(0xC000);
    gb.memory.write_u8(0xC000, 0x30);

    gb.cp(0xBE).unwrap();

    assert_eq!(gb.cpu.a, 0x30);
    assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
}

#[test]
fn test_cp_a_n8() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x30;
    gb.cpu.program_counter = 0x100;
    gb.memory.write_u8(0x101, 0x10);

    gb.cp(0xFE).unwrap();

    assert_eq!(gb.cpu.a, 0x30);
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
}

#[test]
fn test_cp_half_borrow() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x10;
    gb.cpu.b = 0x01;
    gb.cp(0xB8).unwrap();
    assert!(gb.cpu.f & Gbz80::FLAG_H != 0);
}

#[test]
fn test_cp_borrow() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x00;
    gb.cpu.b = 0x01;
    gb.cp(0xB8).unwrap();
    assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
}
