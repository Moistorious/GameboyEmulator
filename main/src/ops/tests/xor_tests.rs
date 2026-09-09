use crate::cpu::{Reg8, Reg16, Gbz80};
use crate::gameboy::Gameboy;

// XOR A,r = 0xA8 + r
// XOR A,(HL) = 0xAE
// XOR A,n  = 0xEE
// XOR always: Z=(result==0), N=0, H=0, C=0

#[test]
fn test_xor_a_a() {
    let mut gameboy = Gameboy::new();
    gameboy.cpu.write_reg8(Reg8::A, 0xAE);
    gameboy.xor(0xAF).unwrap(); // XOR A,A
    assert_eq!(gameboy.cpu.reg8(Reg8::A), 0x00);
    assert!(gameboy.cpu.f & Gbz80::FLAG_Z != 0);
    assert!(gameboy.cpu.f & Gbz80::FLAG_N == 0);
    assert!(gameboy.cpu.f & Gbz80::FLAG_H == 0);
    assert!(gameboy.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_xor_all_reg8() {
    let regs = [Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L];
    for &src in &regs {
        let opcode = 0xA8 | (src as u8);
        let mut gameboy = Gameboy::new();
        gameboy.cpu.write_reg8(Reg8::A, 0x01);
        gameboy.cpu.write_reg8(src, 0xAB);
        gameboy.xor(opcode).unwrap();
        assert_eq!(gameboy.cpu.reg8(Reg8::A), 0xAA, "XOR A,{:?} failed", src);
        assert!(gameboy.cpu.f & Gbz80::FLAG_Z == 0);
        assert!(gameboy.cpu.f & Gbz80::FLAG_N == 0);
        assert!(gameboy.cpu.f & Gbz80::FLAG_H == 0);
        assert!(gameboy.cpu.f & Gbz80::FLAG_C == 0);
    }
}

#[test]
fn test_xor_a_from_hl() {
    let mut gameboy = Gameboy::new();
    gameboy.cpu.write_reg8(Reg8::A, 0x01);
    gameboy.cpu.write_reg16(Reg16::HL, 0xC000);
    gameboy.memory.write_u8(0xC000, 0xAB);
    gameboy.xor(0xAE).unwrap(); // XOR A,(HL)
    assert_eq!(gameboy.cpu.reg8(Reg8::A), 0xAA);
    assert!(gameboy.cpu.f & Gbz80::FLAG_Z == 0);
    assert!(gameboy.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_xor_a_n8() {
    let mut gameboy = Gameboy::new();
    gameboy.cpu.write_reg8(Reg8::A, 0x01);
    gameboy.memory.write_u8(0x00, 0xAB); // immediate at PC 0
    gameboy.xor(0xEE).unwrap(); // XOR A,n
    assert_eq!(gameboy.cpu.reg8(Reg8::A), 0xAA);
    assert!(gameboy.cpu.f & Gbz80::FLAG_Z == 0);
    assert!(gameboy.cpu.f & Gbz80::FLAG_N == 0);
    assert!(gameboy.cpu.f & Gbz80::FLAG_H == 0);
    assert!(gameboy.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_xor_zero_result() {
    let mut gameboy = Gameboy::new();
    gameboy.cpu.write_reg8(Reg8::A, 0xF0);
    gameboy.cpu.write_reg8(Reg8::B, 0xF0);
    gameboy.xor(0xA8).unwrap(); // XOR A,B
    assert_eq!(gameboy.cpu.reg8(Reg8::A), 0x00);
    assert!(gameboy.cpu.f & Gbz80::FLAG_Z != 0);
    assert!(gameboy.cpu.f & Gbz80::FLAG_N == 0);
    assert!(gameboy.cpu.f & Gbz80::FLAG_H == 0);
    assert!(gameboy.cpu.f & Gbz80::FLAG_C == 0);
}
