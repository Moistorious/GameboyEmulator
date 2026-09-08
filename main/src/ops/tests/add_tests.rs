use crate::cpu::{Reg8, Reg16, Gbz80};
use crate::gameboy::Gameboy;

#[test]
fn test_add_a_r8() {
    let regs = [Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L, Reg8::A];
    for &reg in regs.iter() {
        let mut gb = Gameboy::new();
        let opcode = 0x80 + (reg as u8);

        gb.cpu.a = 0x10;
        gb.cpu.write_reg8(reg, 0x20);
        if reg == Reg8::A {
            gb.add(opcode);
            assert_eq!(gb.cpu.a, 0x20);
        } else {
            gb.add(opcode);
            assert_eq!(gb.cpu.a, 0x30);
        }
        assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
    }
}

#[test]
fn test_add_a_hl_mem() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x12;
    gb.cpu.set_hl(0x1234);
    gb.memory.write_u8(0x1234, 0x08);

    gb.add(0x86); // ADD A, (HL)

    assert_eq!(gb.cpu.a, 0x1A);
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_add_a_n8() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x25;
    gb.cpu.program_counter = 0x100;
    gb.memory.write_u8(0x101, 0x10);

    gb.add(0xC6); // ADD A, n

    assert_eq!(gb.cpu.a, 0x35);
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_add_flags() {
    let mut gb = Gameboy::new();

    // Zero flag
    gb.cpu.a = 0x00;
    gb.cpu.b = 0x00;
    gb.add(0x80); // ADD A, B
    assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N == 0);

    // Half carry flag (0x0F + 0x01 = 0x10)
    gb.cpu.a = 0x0F;
    gb.cpu.b = 0x01;
    gb.add(0x80);
    assert!(gb.cpu.f & Gbz80::FLAG_H != 0);

    // Carry flag (0xFF + 0x01 = 0x00 with carry)
    gb.cpu.a = 0xFF;
    gb.cpu.b = 0x01;
    gb.add(0x80);
    assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
}

#[test]
fn test_add_hl_rr() {
    let mut gb = Gameboy::new();
    gb.cpu.set_hl(0x1000);
    gb.cpu.set_bc(0x0234);

    gb.add(0x09); // ADD HL, BC

    assert_eq!(gb.cpu.hl(), 0x1234);
    assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_add_hl_all_rr() {
    let cases = [(0x09, Reg16::BC), (0x19, Reg16::DE), (0x29, Reg16::HL), (0x39, Reg16::HL)];
    for &(opcode, reg) in cases.iter() {
        let mut gb = Gameboy::new();
        // HL gets added to itself for 0x29/0x39 is trickier; use a distinct HL setup.
        gb.cpu.set_hl(0x1000);
        match reg {
            Reg16::BC => gb.cpu.set_bc(0x0234),
            Reg16::DE => gb.cpu.set_de(0x0234),
            _ => {}
        }
        let expect = if matches!(reg, Reg16::HL) { 0x2000 } else { 0x1234 };
        gb.add(opcode);
        assert_eq!(gb.cpu.hl(), expect, "ADD HL,{:?} (0x{:02X}) failed", reg, opcode);
        assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
    }
}

#[test]
fn test_add_sp_n() {
    let mut gb = Gameboy::new();
    gb.cpu.stack_pointer = 0x1000;
    gb.cpu.program_counter = 0x200;
    gb.memory.write_u8(0x201, 0x02); // immediate n = 2

    gb.add(0xE8); // ADD SP, n

    assert_eq!(gb.cpu.stack_pointer, 0x1002);
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_add_sp_negative() {
    let mut gb = Gameboy::new();
    gb.cpu.stack_pointer = 0x1000;
    gb.cpu.program_counter = 0x200;
    gb.memory.write_u8(0x201, 0xFF); // immediate n = -1

    gb.add(0xE8); // ADD SP, n

    assert_eq!(gb.cpu.stack_pointer, 0x0FFF);
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
}
