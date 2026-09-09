use crate::cpu::{Reg8, Reg16, Gbz80};
use crate::gameboy::Gameboy;

// OR A,r = 0xB0 + r
// OR A,(HL) = 0xB6
// OR A,n  = 0xF6
// OR always: Z=(result==0), N=0, H=0, C=0

#[test]
fn test_or_all_reg8() {
    let regs = [Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L];
    for &src in &regs {
        let opcode = 0xB0 | (src as u8);
        let mut gameboy = Gameboy::new();
        gameboy.cpu.write_reg8(Reg8::A, 0x0F);
        gameboy.cpu.write_reg8(src, 0xF0);
        gameboy.or(opcode).unwrap();
        assert_eq!(gameboy.cpu.reg8(Reg8::A), 0xFF, "OR A,{:?} failed", src);
        assert!(gameboy.cpu.f & Gbz80::FLAG_Z == 0);
        assert!(gameboy.cpu.f & Gbz80::FLAG_N == 0);
        assert!(gameboy.cpu.f & Gbz80::FLAG_H == 0);
        assert!(gameboy.cpu.f & Gbz80::FLAG_C == 0);
    }
}

#[test]
fn test_or_a_a() {
    let mut gameboy = Gameboy::new();
    gameboy.cpu.write_reg8(Reg8::A, 0x01);
    gameboy.or(0xB7).unwrap(); // OR A,A
    assert_eq!(gameboy.cpu.reg8(Reg8::A), 0x01);
    assert!(gameboy.cpu.f & Gbz80::FLAG_Z == 0);
}

#[test]
fn test_or_a_hl() {
    let mut gameboy = Gameboy::new();
    gameboy.cpu.write_reg8(Reg8::A, 0x0F);
    gameboy.cpu.write_reg16(Reg16::HL, 0xC000);
    gameboy.memory.write_u8(0xC000, 0xF0);
    gameboy.or(0xB6).unwrap(); // OR A,(HL)
    assert_eq!(gameboy.cpu.reg8(Reg8::A), 0xFF);
    assert!(gameboy.cpu.f & Gbz80::FLAG_Z == 0);
}

#[test]
fn test_or_a_n8() {
    let mut gameboy = Gameboy::new();
    gameboy.cpu.write_reg8(Reg8::A, 0x0F);
    gameboy.memory.write_u8(0x00, 0xF0); // immediate
    gameboy.or(0xF6).unwrap(); // OR A,n
    assert_eq!(gameboy.cpu.reg8(Reg8::A), 0xFF);
    assert!(gameboy.cpu.f & Gbz80::FLAG_Z == 0);
}

#[test]
fn test_or_zero_result() {
    let mut gameboy = Gameboy::new();
    gameboy.cpu.write_reg8(Reg8::A, 0x00);
    gameboy.cpu.write_reg8(Reg8::B, 0x00);
    gameboy.or(0xB0).unwrap(); // OR A,B
    assert_eq!(gameboy.cpu.reg8(Reg8::A), 0x00);
    assert!(gameboy.cpu.f & Gbz80::FLAG_Z != 0);
    assert!(gameboy.cpu.f & Gbz80::FLAG_N == 0);
    assert!(gameboy.cpu.f & Gbz80::FLAG_H == 0);
    assert!(gameboy.cpu.f & Gbz80::FLAG_C == 0);
}
