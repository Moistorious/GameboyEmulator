use crate::cpu::{Flag, Reg8};
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
    assert!(!gb.cpu.get_flag(Flag::Z));
    assert!(gb.cpu.get_flag(Flag::N));
    assert!(!gb.cpu.get_flag(Flag::H));
    assert!(!gb.cpu.get_flag(Flag::C));
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

        if reg == Reg8::A {
            assert_eq!(gb.cpu.a, 0x00, "SUB A,{:?} failed", reg); // A - A = 0
        } else {
            assert_eq!(gb.cpu.a, 0x20, "SUB A,{:?} failed", reg);
        }
        assert!(gb.cpu.get_flag(Flag::N));
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
    assert!(gb.cpu.get_flag(Flag::N));
}

#[test]
fn test_sub_a_n8() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x30;
    gb.cpu.program_counter = 0x100;
    gb.memory.write_u8(0x100, 0x10);

    gb.sub(0xD6);

    assert_eq!(gb.cpu.a, 0x20);
    assert!(gb.cpu.get_flag(Flag::N));
}

#[test]
fn test_sub_zero_flag() {
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x10;
    gb.cpu.b = 0x10;
    gb.sub(0x90);
    assert!(gb.cpu.get_flag(Flag::Z));
    assert!(gb.cpu.get_flag(Flag::N));
}

#[test]
fn test_sub_half_borrow() {
    // 0x10 - 0x01 = 0x0F -> H set
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x10;
    gb.cpu.b = 0x01;
    gb.sub(0x90);
    assert!(gb.cpu.get_flag(Flag::H));

    // 0x11 - 0x01 = 0x10 -> H clear
    let mut gb2 = Gameboy::new();
    gb2.cpu.a = 0x11;
    gb2.cpu.b = 0x01;
    gb2.sub(0x90);
    assert!(!gb2.cpu.get_flag(Flag::H));
}

#[test]
fn test_sub_borrow() {
    // 0x00 - 0x01 = 0xFF -> C set
    let mut gb = Gameboy::new();
    gb.cpu.a = 0x00;
    gb.cpu.b = 0x01;
    gb.sub(0x90);
    assert!(gb.cpu.get_flag(Flag::C));

    // 0x10 - 0x01 = 0x0F -> C clear
    let mut gb2 = Gameboy::new();
    gb2.cpu.a = 0x10;
    gb2.cpu.b = 0x01;
    gb2.sub(0x90);
    assert!(!gb2.cpu.get_flag(Flag::C));
}
