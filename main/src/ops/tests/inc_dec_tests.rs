use crate::cpu::{Reg8, Reg16, Gbz80};
use crate::gameboy::Gameboy;

// INC r   = 0x04 + r (r = 0..7)
// INC (HL) = 0x34
// INC rr  = 0x03, 0x13, 0x23, 0x33 (BC, DE, HL, SP) — no flags
// DEC r   = 0x05 + r
// DEC (HL) = 0x35
// DEC rr  = 0x0B, 0x1B, 0x2B, 0x3B — no flags
// 8-bit INC/DEC affect Z, N, H (not C)

#[test]
fn test_inc_r8() {
    let mut gb = Gameboy::new();
    gb.cpu.b = 0x0F;
    gb.inc(0x04).unwrap(); // INC B
    assert_eq!(gb.cpu.b, 0x10);
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H != 0);
}

#[test]
fn test_inc_all_r8() {
    let regs = [Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L, Reg8::A];
    for &reg in regs.iter() {
        let mut gb = Gameboy::new();
        let opcode = 0x04 + (reg as u8);
        gb.cpu.write_reg8(reg, 0x25);
        gb.inc(opcode).unwrap();
        assert_eq!(gb.cpu.reg8(reg), 0x26, "INC {:?} failed", reg);
        assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
    }
}

#[test]
fn test_inc_zero() {
    let mut gb = Gameboy::new();
    gb.cpu.b = 0xFF;
    gb.inc(0x04).unwrap(); // INC B -> 0x00
    assert_eq!(gb.cpu.b, 0x00);
    assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
}

#[test]
fn test_inc_carry_untouched() {
    // 8-bit INC does not modify carry
    let mut gb = Gameboy::new();
    gb.cpu.b = 0x01;
    gb.cpu.set_flag(Gbz80::FLAG_C, true);
    gb.inc(0x04).unwrap();
    assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
}

#[test]
fn test_inc_hl_mem() {
    let mut gb = Gameboy::new();
    gb.cpu.set_hl(0xC000);
    gb.memory.write_u8(0xC000, 0x50);
    gb.inc(0x34).unwrap(); // INC (HL)
    assert_eq!(gb.memory.read_u8(0xC000), 0x51);
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
}

#[test]
fn test_inc_r16() {
    let cases = [(0x03, Reg16::BC), (0x13, Reg16::DE), (0x23, Reg16::HL)];
    for &(opcode, reg) in cases.iter() {
        let mut gb = Gameboy::new();
        gb.cpu.write_reg16(reg, 0x0FFF);
        gb.cpu.f = 0xF0;
        gb.inc(opcode).unwrap();
        assert_eq!(gb.cpu.reg16(reg), 0x1000, "INC {:?} failed", reg);
        assert_eq!(gb.cpu.f, 0xF0, "INC 16-bit must not modify flags");
    }
}

#[test]
fn test_inc_sp() {
    let mut gb = Gameboy::new();
    gb.cpu.stack_pointer = 0xFFFF;
    gb.inc(0x33).unwrap(); // INC SP
    assert_eq!(gb.cpu.stack_pointer, 0x0000);
}

#[test]
fn test_dec_r8() {
    let mut gb = Gameboy::new();
    gb.cpu.b = 0x10;
    gb.dec(0x05).unwrap(); // DEC B
    assert_eq!(gb.cpu.b, 0x0F);
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H != 0); // half borrow
}

#[test]
fn test_dec_all_r8() {
    let regs = [Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L, Reg8::A];
    for &reg in regs.iter() {
        let mut gb = Gameboy::new();
        let opcode = 0x05 + (reg as u8);
        gb.cpu.write_reg8(reg, 0x26);
        gb.dec(opcode).unwrap();
        assert_eq!(gb.cpu.reg8(reg), 0x25, "DEC {:?} failed", reg);
        assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
    }
}

#[test]
fn test_dec_zero() {
    let mut gb = Gameboy::new();
    gb.cpu.b = 0x01;
    gb.dec(0x05).unwrap(); // DEC B -> 0x00
    assert_eq!(gb.cpu.b, 0x00);
    assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
}

#[test]
fn test_dec_half_borrow() {
    // 0x00 - 1 = 0xFF -> H set (borrow from bit 4)
    let mut gb = Gameboy::new();
    gb.cpu.b = 0x00;
    gb.dec(0x05).unwrap();
    assert!(gb.cpu.f & Gbz80::FLAG_H != 0);

    // 0x10 - 1 = 0x0F -> H clear
    let mut gb2 = Gameboy::new();
    gb2.cpu.b = 0x10;
    gb2.dec(0x05).unwrap();
    assert!(gb2.cpu.f & Gbz80::FLAG_H == 0);
}

#[test]
fn test_dec_carry_untouched() {
    let mut gb = Gameboy::new();
    gb.cpu.b = 0x01;
    gb.cpu.set_flag(Gbz80::FLAG_C, true);
    gb.dec(0x05).unwrap();
    assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
}

#[test]
fn test_dec_hl_mem() {
    let mut gb = Gameboy::new();
    gb.cpu.set_hl(0xC000);
    gb.memory.write_u8(0xC000, 0x50);
    gb.dec(0x35).unwrap(); // DEC (HL)
    assert_eq!(gb.memory.read_u8(0xC000), 0x4F);
    assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
}

#[test]
fn test_dec_r16() {
    let cases = [(0x0B, Reg16::BC), (0x1B, Reg16::DE), (0x2B, Reg16::HL)];
    for &(opcode, reg) in cases.iter() {
        let mut gb = Gameboy::new();
        gb.cpu.write_reg16(reg, 0x1000);
        gb.cpu.f = 0xF0;
        gb.dec(opcode).unwrap();
        assert_eq!(gb.cpu.reg16(reg), 0x0FFF, "DEC {:?} failed", reg);
        assert_eq!(gb.cpu.f, 0xF0, "DEC 16-bit must not modify flags");
    }
}
