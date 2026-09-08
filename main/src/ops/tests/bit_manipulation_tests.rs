use crate::gameboy::Gameboy;
use crate::cpu::{Gbz80, Reg8};

fn bit_opcode(bit: u8, reg: Reg8) -> u8 {
    0x40 | ((bit & 0x07) << 3) | (reg as u8)
}

#[test]
fn test_bit_all_registers() {
    let regs = [Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L, Reg8::A];
    for &reg in regs.iter() {
        // BIT 0 = 0x40 | reg
        let mut gb = Gameboy::new();
        gb.cpu.write_reg8(reg, 0x00);
        gb.bit(bit_opcode(0, reg));
        assert!(gb.cpu.f & Gbz80::FLAG_Z != 0, "BIT 0,{:?} (clear bit) should set Z", reg);
        assert!(gb.cpu.f & Gbz80::FLAG_N == 0, "BIT must clear N");
        assert!(gb.cpu.f & Gbz80::FLAG_H != 0, "BIT must set H");

        let mut gb2 = Gameboy::new();
        gb2.cpu.write_reg8(reg, 0x01);
        gb2.bit(bit_opcode(0, reg));
        assert!(gb2.cpu.f & Gbz80::FLAG_Z == 0, "BIT 0,{:?} (set bit) should clear Z", reg);
    }
}

#[test]
fn test_bit_all_bits() {
    for bit in 0..8u8 {
        let mut gb = Gameboy::new();
        gb.cpu.b = 1 << bit;
        gb.bit(bit_opcode(bit, Reg8::B));
        assert!(gb.cpu.f & Gbz80::FLAG_Z == 0, "BIT {},B (set) should clear Z", bit);

        let mut gb2 = Gameboy::new();
        gb2.cpu.b = !(1u8 << bit);
        gb2.bit(bit_opcode(bit, Reg8::B));
        assert!(gb2.cpu.f & Gbz80::FLAG_Z != 0, "BIT {},B (clear) should set Z", bit);
    }
}

#[test]
fn test_bit_preserves_carry() {
    // BIT sets and clears carry per spec: C is unaffected.
    let mut gb = Gameboy::new();
    gb.cpu.b = 0x01;
    gb.cpu.set_flag(Gbz80::FLAG_C, true);
    gb.bit(bit_opcode(0, Reg8::B));
    assert!(gb.cpu.f & Gbz80::FLAG_C != 0, "BIT must preserve C when set");

    gb.cpu.set_flag(Gbz80::FLAG_C, false);
    gb.bit(bit_opcode(0, Reg8::B));
    assert!(gb.cpu.f & Gbz80::FLAG_C == 0, "BIT must preserve C when clear");
}

#[test]
fn test_bit_hl_mem() {
    let mut gb = Gameboy::new();
    gb.cpu.set_hl(0xC000);
    gb.memory.write_u8(0xC000, 0x80);
    gb.bit(bit_opcode(7, Reg8::F)); // BIT 7,(HL) = 0x7E? BIT b,(HL) = 0xCB low nibble 6
    // Actually BIT 7,(HL) opcode low bits are 6 (for (HL)); here we test the memory path via reg=F (6)
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);

    gb.memory.write_u8(0xC000, 0x00);
    gb.bit(bit_opcode(7, Reg8::F));
    assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
}

fn set_opcode(bit: u8, reg: Reg8) -> u8 {
    0xC0 | ((bit & 0x07) << 3) | (reg as u8)
}

fn res_opcode(bit: u8, reg: Reg8) -> u8 {
    0x80 | ((bit & 0x07) << 3) | (reg as u8)
}

#[test]
fn test_set_all_registers() {
    let regs = [Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L, Reg8::A];
    for &reg in regs.iter() {
        let mut gb = Gameboy::new();
        gb.cpu.write_reg8(reg, 0x00);
        gb.set(set_opcode(3, reg)); // SET 3, r
        assert_eq!(gb.cpu.reg8(reg), 0x08, "SET 3,{:?} failed", reg);
    }
}

#[test]
fn test_set_all_bits() {
    for bit in 0..8u8 {
        let mut gb = Gameboy::new();
        gb.cpu.b = 0x00;
        gb.set(set_opcode(bit, Reg8::B));
        assert_eq!(gb.cpu.b, 1 << bit, "SET {},B failed", bit);
    }
}

#[test]
fn test_set_hl_mem() {
    let mut gb = Gameboy::new();
    gb.cpu.set_hl(0xC000);
    gb.memory.write_u8(0xC000, 0x00);
    gb.set(set_opcode(3, Reg8::F)); // SET 3,(HL)
    assert_eq!(gb.memory.read_u8(0xC000), 0x08);
}

#[test]
fn test_set_does_not_modify_flags() {
    let mut gb = Gameboy::new();
    gb.cpu.b = 0x00;
    gb.cpu.f = 0xF0;
    gb.set(set_opcode(0, Reg8::B));
    assert_eq!(gb.cpu.f, 0xF0, "SET must not modify flags");
}

#[test]
fn test_res_all_registers() {
    let regs = [Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L, Reg8::A];
    for &reg in regs.iter() {
        let mut gb = Gameboy::new();
        gb.cpu.write_reg8(reg, 0xFF);
        gb.res(res_opcode(3, reg)); // RES 3, r
        assert_eq!(gb.cpu.reg8(reg), 0xF7, "RES 3,{:?} failed", reg);
    }
}

#[test]
fn test_res_all_bits() {
    for bit in 0..8u8 {
        let mut gb = Gameboy::new();
        gb.cpu.b = 0xFF;
        gb.res(res_opcode(bit, Reg8::B));
        assert_eq!(gb.cpu.b, 0xFF ^ (1 << bit), "RES {},B failed", bit);
    }
}

#[test]
fn test_res_hl_mem() {
    let mut gb = Gameboy::new();
    gb.cpu.set_hl(0xC000);
    gb.memory.write_u8(0xC000, 0xFF);
    gb.res(res_opcode(3, Reg8::F)); // RES 3,(HL)
    assert_eq!(gb.memory.read_u8(0xC000), 0xF7);
}

#[test]
fn test_res_does_not_modify_flags() {
    let mut gb = Gameboy::new();
    gb.cpu.b = 0xFF;
    gb.cpu.f = 0xF0;
    gb.res(res_opcode(0, Reg8::B));
    assert_eq!(gb.cpu.f, 0xF0, "RES must not modify flags");
}
