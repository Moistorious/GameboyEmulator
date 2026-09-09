use crate::gameboy::Gameboy;
use crate::cpu::Gbz80;

// DAA = 0x27
// CPL = 0x2F: A = ~A; N=1, H=1; Z,C preserved
// SCF = 0x37: C=1, N=0, H=0; Z preserved
// CCF = 0x3F: C = ~C, N=0, H=0; Z preserved

#[test]
fn test_daa_addition() {
    // 0x15 + 0x27 = 0x3C -> 0x42
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x3C;
    gb.cpu.set_flags(false, false, false, false);
    gb.daa(0x27).unwrap();
    assert_eq!(gb.cpu.a, 0x42);
    assert!(gb.cpu.f & Gbz80::FLAG_Z == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
}

#[test]
fn test_daa_addition_carry() {
    // 0x45 + 0x55 = 0x9A -> 0x00, C=1
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x9A;
    gb.cpu.set_flags(false, false, false, false);
    gb.daa(0x27).unwrap();
    assert_eq!(gb.cpu.a, 0x00);
    assert!(gb.cpu.f & Gbz80::FLAG_Z != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
}

#[test]
fn test_daa_subtraction() {
    // 0x42 - 0x27 = 0x1B -> 0x15 (N=1, H=1)
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x1B;
    gb.cpu.set_flags(false, true, true, false);
    gb.daa(0x27).unwrap();
    assert_eq!(gb.cpu.a, 0x15);
    assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
}

#[test]
fn test_cpl() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x35;
    gb.cpl(0x2F).unwrap();
    assert_eq!(gb.cpu.a, 0xCA);
    assert!(gb.cpu.f & Gbz80::FLAG_N != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H != 0);

    // Z and C preserved
    let mut gb2 = Gameboy::new();
    gb2.cpu.set_flag(Gbz80::FLAG_Z, true);
    gb2.cpu.set_flag(Gbz80::FLAG_C, true);
    gb2.cpl(0x2F).unwrap();
    assert!(gb2.cpu.f & Gbz80::FLAG_Z != 0);
    assert!(gb2.cpu.f & Gbz80::FLAG_C != 0);
}

#[test]
fn test_scf() {
    let mut gb = Gameboy::new();
    gb.cpu.set_flag(Gbz80::FLAG_C, false);
    gb.cpu.set_flag(Gbz80::FLAG_N, true);
    gb.cpu.set_flag(Gbz80::FLAG_H, true);
    gb.scf(0x37).unwrap();
    assert!(gb.cpu.f & Gbz80::FLAG_C != 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);

    // Z preserved
    let mut gb2 = Gameboy::new();
    gb2.cpu.set_flag(Gbz80::FLAG_Z, true);
    gb2.scf(0x37).unwrap();
    assert!(gb2.cpu.f & Gbz80::FLAG_Z != 0);
}

#[test]
fn test_ccf() {
    let mut gb = Gameboy::new();
    gb.cpu.set_flag(Gbz80::FLAG_C, true);
    gb.cpu.set_flag(Gbz80::FLAG_N, true);
    gb.cpu.set_flag(Gbz80::FLAG_H, true);
    gb.ccf(0x3F).unwrap();
    assert!(gb.cpu.f & Gbz80::FLAG_C == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_N == 0);
    assert!(gb.cpu.f & Gbz80::FLAG_H == 0);

    gb.ccf(0x3F).unwrap();
    assert!(gb.cpu.f & Gbz80::FLAG_C != 0);

    // Z preserved
    let mut gb2 = Gameboy::new();
    gb2.cpu.set_flag(Gbz80::FLAG_Z, true);
    gb2.ccf(0x3F).unwrap();
    assert!(gb2.cpu.f & Gbz80::FLAG_Z != 0);
}

#[test]
fn test_di_ei() {
    let mut gb = Gameboy::new();
    gb.di(0xF3).unwrap();
    gb.ei(0xFB).unwrap();
}

#[test]
fn test_halt() {
    let mut gb = Gameboy::new();
    gb.running = true;
    gb.halt().unwrap();
    assert_eq!(gb.running, false);
}

#[test]
fn test_stop() {
    let mut gb = Gameboy::new();
    gb.stop(0x10).unwrap();
}
