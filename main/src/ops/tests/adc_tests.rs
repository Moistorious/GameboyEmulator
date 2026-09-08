use crate::cpu::{Reg8, Gbz80};
use crate::gameboy::Gameboy;

#[test]
fn test_adc_a_r8() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x10;
    gb.cpu.b = 0x20;
    gb.cpu.set_flag(Gbz80::FLAG_C, true);

    gb.adc(0x88); // ADC A, B

    assert_eq!(gb.cpu.a, 0x31);
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_adc_a_r8_no_carry_in() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x10;
    gb.cpu.b = 0x20;
    gb.cpu.set_flag(Gbz80::FLAG_C, false);

    gb.adc(0x88); // ADC A, B

    assert_eq!(gb.cpu.a, 0x30);
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_adc_all_r8() {
    let regs = [Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L];
    for &reg in regs.iter() {
        let mut gb = Gameboy::new();
        let opcode = 0x88 + (reg as u8);
        gb.cpu.a = 0x0A;
        gb.cpu.write_reg8(reg, 0x05);
        gb.cpu.set_flag(Gbz80::FLAG_C, false);

        gb.adc(opcode);

        assert_eq!(gb.cpu.a, 0x0F, "ADC A,{:?} failed", reg);
        assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
        assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
    }
}

#[test]
fn test_adc_a_hl_mem() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x0A;
    gb.cpu.set_hl(0xC000);
    gb.memory.write_u8(0xC000, 0x05);
    gb.cpu.set_flag(Gbz80::FLAG_C, false);

    gb.adc(0x8E); // ADC A, (HL)

    assert_eq!(gb.cpu.a, 0x0F);
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_adc_a_n8() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x0A;
    gb.cpu.program_counter = 0x100;
    gb.memory.write_u8(0x101, 0x05);
    gb.cpu.set_flag(Gbz80::FLAG_C, false);

    gb.adc(0xCE); // ADC A, n

    assert_eq!(gb.cpu.a, 0x0F);
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_adc_flags() {
    let mut gb = Gameboy::new();

    // Zero flag: 0xFF + 0x00 + carry 1 = 0x00 -> Z=1, C=1
    gb.cpu.a = 0xFF;
    gb.cpu.b = 0x00;
    gb.cpu.set_flag(Gbz80::FLAG_C, true);
    gb.adc(0x88);
    assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_C != 0);

    // Half carry: 0x0F + 0x00 + carry 1 = 0x10 -> H=1
    gb.cpu.a = 0x0F;
    gb.cpu.b = 0x00;
    gb.cpu.set_flag(Gbz80::FLAG_C, true);
    gb.adc(0x88);
    assert!(gb.cpu.f & Gbz80::FLAG_H != 0);

    // Carry out: 0xFE + 0x02 + carry 0 = 0x00 -> C=1
    gb.cpu.a = 0xFE;
    gb.cpu.b = 0x02;
    gb.cpu.set_flag(Gbz80::FLAG_C, false);
    gb.adc(0x88);
    assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
}
