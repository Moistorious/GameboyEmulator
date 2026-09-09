use crate::cpu::{Reg8, Gbz80};
use crate::gameboy::Gameboy;

// AND A,r = 0xA0 + r
// AND A,(HL) = 0xA6
// AND A,n  = 0xE6
// AND always: Z (result==0), N=0, H=1, C=0

#[test]
fn test_and_a_r8() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0xF0;
    gb.cpu.b = 0x01;

    gb.and(0xA0).unwrap();

    assert_eq!(gb.cpu.a, 0x00);
    assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_and_all_r8() {
    let regs = [Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L];
    for &reg in regs.iter() {
        let mut gb = Gameboy::new();
        let opcode = 0xA0 + (reg as u8);
        gb.cpu.a = 0xFF;
        gb.cpu.write_reg8(reg, 0x0F);

        gb.and(opcode).unwrap();

        assert_eq!(gb.cpu.a, 0x0F, "AND A,{:?} failed", reg);
        assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H != 0);
        assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
    }
}

#[test]
fn test_and_a_hl_mem() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0xFF;
    gb.cpu.set_hl(0xC000);
    gb.memory.write_u8(0xC000, 0x0F);

    gb.and(0xA6).unwrap();

    assert_eq!(gb.cpu.a, 0x0F);
    assert!(gb.cpu.f & Gbz80::FLAG_H != 0);
}

#[test]
fn test_and_a_n8() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0xFF;
    gb.cpu.program_counter = 0x100;
    gb.memory.write_u8(0x101, 0x0F);

    gb.and(0xE6).unwrap();

    assert_eq!(gb.cpu.a, 0x0F);
    assert!(gb.cpu.f & Gbz80::FLAG_H != 0);
}

#[test]
fn test_and_zero_result() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0xF0;
    gb.cpu.b = 0x0F;

    gb.and(0xA0).unwrap();

    assert_eq!(gb.cpu.a, 0x00);
    assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
}
