use crate::cpu::{Reg8, Gbz80};
use crate::gameboy::Gameboy;

// SUB A,r   = 0x90 + r
// SUB A,(HL) = 0x96
// SUB A,n    = 0xD6

#[test]
fn test_sub_a_r8() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x30;
    gb.cpu.b = 0x10;

    gb.sub(0x90);

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
        assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
    }
}

#[test]
fn test_sub_a_hl_mem() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x30;
    gb.cpu.set_hl(0xC000);
    gb.memory.write_u8(0xC000, 0x10);

    gb.sub(0x96);

    assert_eq!(gb.cpu.a, 0x20);
    assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
}

#[test]
fn test_sub_a_n8() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x30;
    gb.cpu.program_counter = 0x100;
    gb.memory.write_u8(0x101, 0x10);

    gb.sub(0xD6);

    assert_eq!(gb.cpu.a, 0x20);
    assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
}

#[test]
fn test_sub_zero_flag() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x10;
    gb.cpu.b = 0x10;
    gb.sub(0x90);
    assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
}

#[test]
fn test_sub_half_borrow() {
    // 0x10 - 0x01 = 0x0F -> H set
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x10;
    gb.cpu.b = 0x01;
    gb.sub(0x90);
    assert!(gb.cpu.f & Gbz80::FLAG_H != 0);

    // 0x20 - 0x01 = 0x1F -> H clear
    let mut gb2 = Gameboy::new();
    gb2.cpu.a = 0x20;
    gb2.cpu.b = 0x01;
    gb2.sub(0x90);
    assert!(gb2.cpu.f & Gbz80::FLAG_H == 0);
}

#[test]
fn test_sub_borrow() {
    // 0x00 - 0x01 = 0xFF -> C set
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x00;
    gb.cpu.b = 0x01;
    gb.sub(0x90);
    assert!(gb.cpu.f & Gbz80::FLAG_C != 0);

    // 0x10 - 0x01 = 0x0F -> C clear
    let mut gb2 = Gameboy::new();
    gb2.cpu.a = 0x10;
    gb2.cpu.b = 0x01;
    gb2.sub(0x90);
    assert!(gb2.cpu.f & Gbz80::FLAG_C == 0);
}
