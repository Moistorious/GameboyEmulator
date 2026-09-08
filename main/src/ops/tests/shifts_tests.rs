use crate::gameboy::Gameboy;
use crate::cpu::{Gbz80, Reg8};

// CB-prefixed rotates/shifts. Low nibble selects register: 0..7 = B..(HL)
// All clear N and H. Z = (result == 0). C = shifted-out bit.
// SWAP clears C. SRA/others set C from the bit shifted out.

fn reg_val(opcode: u8) -> u8 {
    opcode & 0x07
}

#[test]
fn test_rlc() {
    // RLC A (0x07)
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x85; // 1000 0101
    gb.rlc(0x07);
    assert_eq!(gb.cpu.a, 0x0B); // 0000 1011
    assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);

    // RLC of 0x00 -> 0x00, Z=1, C=0
    let mut gb2 = Gameboy::new();
    gb2.cpu.a = 0x00;
    gb2.rlc(0x07);
    assert_eq!(gb2.cpu.a, 0x00);
    assert!(gb2.cpu.f & Gbz80::FLAG_Z != 0);
    assert!(gb2.cpu.f & Gbz80::FLAG_C == 0);

    // RLC of 0x80 -> 0x01, C=1
    let mut gb3 = Gameboy::new();
    gb3.cpu.a = 0x80;
    gb3.rlc(0x07);
    assert_eq!(gb3.cpu.a, 0x01);
    assert!(gb3.cpu.f & Gbz80::FLAG_C != 0);
    assert!(gb3.cpu.f & Gbz80::FLAG_Z == 0);
}

#[test]
fn test_rlc_all_regs() {
    let regs = [Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L, Reg8::A];
    for &reg in regs.iter() {
        let mut gb = Gameboy::new();
        let opcode = reg as u8;
        gb.cpu.write_reg8(reg, 0x40);
        gb.rlc(opcode);
        assert_eq!(gb.cpu.reg8(reg), 0x80, "RLC {:?} failed", reg);
        assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
    }
}

#[test]
fn test_rlc_hl_mem() {
    let mut gb = Gameboy::new();
    gb.cpu.set_hl(0xC000);
    gb.memory.write_u8(0xC000, 0x80);
    gb.rlc(0x06); // RLC (HL)
    assert_eq!(gb.memory.read_u8(0xC000), 0x01);
    assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
}

#[test]
fn test_rrc() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x01;
    gb.rrc(0x0F);
    assert_eq!(gb.cpu.a, 0x80);
    assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);

    // RRC of 0x00 -> 0x00, Z=1
    let mut gb2 = Gameboy::new();
    gb2.cpu.a = 0x00;
    gb2.rrc(0x0F);
    assert!(gb2.cpu.f & Gbz80::FLAG_Z != 0);
    assert!(gb2.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_rrc_hl_mem() {
    let mut gb = Gameboy::new();
    gb.cpu.set_hl(0xC000);
    gb.memory.write_u8(0xC000, 0x80);
    gb.rrc(0x0E); // RRC (HL)
    assert_eq!(gb.memory.read_u8(0xC000), 0x40);
    assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_rl() {
    // RL A, carry 0, bit7=1 -> result 0x00, C=1
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x80;
    gb.cpu.set_flag(Gbz80::FLAG_C, false);
    gb.rl(0x17);
    assert_eq!(gb.cpu.a, 0x00);
    assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);

    // RL with carry=1 into empty -> result 0x01
    let mut gb2 = Gameboy::new();
    gb2.cpu.a = 0x00;
    gb2.cpu.set_flag(Gbz80::FLAG_C, true);
    gb2.rl(0x17);
    assert_eq!(gb2.cpu.a, 0x01);
    assert!(gb2.cpu.f & Gbz80::FLAG_C == 0);
    assert!(gb2.cpu.f & Gbz80::FLAG_Z == 0);
}

#[test]
fn test_rl_hl_mem() {
    let mut gb = Gameboy::new();
    gb.cpu.set_hl(0xC000);
    gb.memory.write_u8(0xC000, 0x11);
    gb.cpu.set_flag(Gbz80::FLAG_C, true);
    gb.rl(0x16); // RL (HL)
    assert_eq!(gb.memory.read_u8(0xC000), 0x23);
    assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_rr() {
    // RR A, carry 0, bit0=1 -> result 0x00, C=1
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x01;
    gb.cpu.set_flag(Gbz80::FLAG_C, false);
    gb.rr(0x1F);
    assert_eq!(gb.cpu.a, 0x00);
    assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);

    // RR with carry=1 into empty -> result 0x80
    let mut gb2 = Gameboy::new();
    gb2.cpu.a = 0x00;
    gb2.cpu.set_flag(Gbz80::FLAG_C, true);
    gb2.rr(0x1F);
    assert_eq!(gb2.cpu.a, 0x80);
    assert!(gb2.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_rr_hl_mem() {
    let mut gb = Gameboy::new();
    gb.cpu.set_hl(0xC000);
    gb.memory.write_u8(0xC000, 0x88);
    gb.cpu.set_flag(Gbz80::FLAG_C, true);
    gb.rr(0x1E); // RR (HL)
    assert_eq!(gb.memory.read_u8(0xC000), 0xC4);
    assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_sla() {
    // SLA A: shifts left into carry, bit0=0
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x80;
    gb.sla(0x27);
    assert_eq!(gb.cpu.a, 0x00);
    assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);

    // SLA of 0x01 -> 0x02, C=0
    let mut gb2 = Gameboy::new();
    gb2.cpu.a = 0x01;
    gb2.sla(0x27);
    assert_eq!(gb2.cpu.a, 0x02);
    assert!(gb2.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_sla_hl_mem() {
    let mut gb = Gameboy::new();
    gb.cpu.set_hl(0xC000);
    gb.memory.write_u8(0xC000, 0xFF);
    gb.sla(0x26); // SLA (HL)
    assert_eq!(gb.memory.read_u8(0xC000), 0xFE);
    assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
}

#[test]
fn test_sra() {
    // SRA A: arithmetic shift right, bit7 preserved
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x81; // 1000 0001
    gb.sra(0x2F);
    assert_eq!(gb.cpu.a, 0xC0); // 1100 0000
    assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);

    // SRA of positive 0x01 -> 0x00
    let mut gb2 = Gameboy::new();
    gb2.cpu.a = 0x01;
    gb2.sra(0x2F);
    assert_eq!(gb2.cpu.a, 0x00);
    assert!(gb2.cpu.f & Gbz80::FLAG_C != 0);
    assert!(gb2.cpu.f & Gbz80::FLAG_Z != 0);
}

#[test]
fn test_sra_hl_mem() {
    let mut gb = Gameboy::new();
    gb.cpu.set_hl(0xC000);
    gb.memory.write_u8(0xC000, 0x01);
    gb.sra(0x2E); // SRA (HL)
    assert_eq!(gb.memory.read_u8(0xC000), 0x00);
    assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
}

#[test]
fn test_swap() {
    // SWAP A: exchange high/low nibbles, C reset
    let mut gb = Gameboy::new();
    gb.cpu.a = 0xF0;
    gb.swap(0x37);
    assert_eq!(gb.cpu.a, 0x0F);
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_C == 0);

    // SWAP producing zero -> Z=1
    let mut gb2 = Gameboy::new();
    gb2.cpu.a = 0x00;
    gb2.swap(0x37);
    assert!(gb2.cpu.f & Gbz80::FLAG_Z != 0);
    assert!(gb2.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_swap_hl_mem() {
    let mut gb = Gameboy::new();
    gb.cpu.set_hl(0xC000);
    gb.memory.write_u8(0xC000, 0x12);
    gb.swap(0x36); // SWAP (HL)
    assert_eq!(gb.memory.read_u8(0xC000), 0x21);
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
}

#[test]
fn test_srl() {
    // SRL A: logical shift right, bit7=0
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x81; // 1000 0001
    gb.srl(0x3F);
    assert_eq!(gb.cpu.a, 0x40); // 0100 0000
    assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);

    // SRL of 0x01 -> 0x00, C=1, Z=1
    let mut gb2 = Gameboy::new();
    gb2.cpu.a = 0x01;
    gb2.srl(0x3F);
    assert_eq!(gb2.cpu.a, 0x00);
    assert!(gb2.cpu.f & Gbz80::FLAG_C != 0);
    assert!(gb2.cpu.f & Gbz80::FLAG_Z != 0);
}

#[test]
fn test_srl_hl_mem() {
    let mut gb = Gameboy::new();
    gb.cpu.set_hl(0xC000);
    gb.memory.write_u8(0xC000, 0x01);
    gb.srl(0x3E); // SRL (HL)
    assert_eq!(gb.memory.read_u8(0xC000), 0x00);
    assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
}
