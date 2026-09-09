use crate::cpu::Gbz80;
use crate::gameboy::Gameboy;

// JP nn     = 0xC3
// JP cc,nn  = 0xC2 NZ, 0xCA Z, 0xD2 NC, 0xDA C
// JP HL     = 0xE9
// JR n      = 0x18
// JR cc,n   = 0x20 NZ, 0x28 Z, 0x30 NC, 0x38 C
// CALL nn   = 0xCD
// CALL cc,nn = 0xC4 NZ, 0xCC Z, 0xD4 NC, 0xDC C
// RET       = 0xC9
// RET cc    = 0xC0 NZ, 0xC8 Z, 0xD0 NC, 0xD8 C
// RETI      = 0xD9
// RST       = 0xC7, 0xCF, 0xD7, 0xDF, 0xE7, 0xEF, 0xF7, 0xFF

#[test]
fn test_jp_nn() {
    let mut gb = Gameboy::new();
    gb.cpu.program_counter = 0x100;
    gb.memory.write_u16(0x101, 0x1234);
    gb.jp(0xC3).unwrap();
    assert_eq!(gb.cpu.program_counter, 0x1234);
}

#[test]
fn test_jp_cc_nn_taken() {
    // NZ taken when Z clear
    let mut gb = Gameboy::new();
    gb.cpu.program_counter = 0x100;
    gb.memory.write_u16(0x101, 0x1234);
    gb.cpu.set_flag(Gbz80::FLAG_Z, false);
    gb.jp(0xC2).unwrap();
    assert_eq!(gb.cpu.program_counter, 0x1234);
}

#[test]
fn test_jp_cc_nn_not_taken() {
    // NZ not taken when Z set; PC advances past 3-byte operand
    let mut gb = Gameboy::new();
    gb.cpu.program_counter = 0x100;
    gb.memory.write_u16(0x101, 0x1234);
    gb.cpu.set_flag(Gbz80::FLAG_Z, true);
    gb.jp(0xC2).unwrap();
    assert_eq!(gb.cpu.program_counter, 0x103);
}

#[test]
fn test_jp_z_c_nc_conditions() {
    // Z: taken when Z set
    let mut gb = Gameboy::new();
    gb.cpu.program_counter = 0x100;
    gb.memory.write_u16(0x101, 0x2222);
    gb.cpu.set_flag(Gbz80::FLAG_Z, true);
    gb.jp(0xCA).unwrap();
    assert_eq!(gb.cpu.program_counter, 0x2222);

    // C: taken when C set
    let mut gb2 = Gameboy::new();
    gb2.cpu.program_counter = 0x100;
    gb2.memory.write_u16(0x101, 0x3333);
    gb2.cpu.set_flag(Gbz80::FLAG_C, true);
    gb2.jp(0xDA).unwrap();
    assert_eq!(gb2.cpu.program_counter, 0x3333);

    // NC: taken when C clear
    let mut gb3 = Gameboy::new();
    gb3.cpu.program_counter = 0x100;
    gb3.memory.write_u16(0x101, 0x4444);
    gb3.cpu.set_flag(Gbz80::FLAG_C, false);
    gb3.jp(0xD2).unwrap();
    assert_eq!(gb3.cpu.program_counter, 0x4444);
}

#[test]
fn test_jp_hl() {
    let mut gb = Gameboy::new();
    gb.cpu.set_hl(0xABCD);
    gb.jp(0xE9).unwrap();
    assert_eq!(gb.cpu.program_counter, 0xABCD);
}

#[test]
fn test_jr_n() {
    let mut gb = Gameboy::new();
    gb.cpu.program_counter = 0x100;
    gb.memory.write_u8(0x101, 0x05);
    gb.jr(0x18).unwrap();
    assert_eq!(gb.cpu.program_counter, 0x107);
}

#[test]
fn test_jr_n_negative() {
    let mut gb = Gameboy::new();
    gb.cpu.program_counter = 0x100;
    gb.memory.write_u8(0x101, 0xFB); // -5
    gb.jr(0x18).unwrap();
    assert_eq!(gb.cpu.program_counter, 0x0FD);
}

#[test]
fn test_jr_cc_taken_and_not() {
    // JR NZ,n taken
    let mut gb = Gameboy::new();
    gb.cpu.program_counter = 0x100;
    gb.memory.write_u8(0x101, 0x03);
    gb.cpu.set_flag(Gbz80::FLAG_Z, false);
    gb.jr(0x20).unwrap();
    assert_eq!(gb.cpu.program_counter, 0x105);

    // JR NZ,n not taken
    let mut gb2 = Gameboy::new();
    gb2.cpu.program_counter = 0x100;
    gb2.memory.write_u8(0x101, 0x03);
    gb2.cpu.set_flag(Gbz80::FLAG_Z, true);
    gb2.jr(0x20).unwrap();
    assert_eq!(gb2.cpu.program_counter, 0x102);
}

#[test]
fn test_call_nn() {
    let mut gb = Gameboy::new();
    gb.cpu.program_counter = 0x100;
    gb.cpu.stack_pointer = 0xFFFE;
    gb.memory.write_u16(0x101, 0x1234);
    gb.call(0xCD).unwrap();
    assert_eq!(gb.cpu.program_counter, 0x1234);
    assert_eq!(gb.cpu.stack_pointer, 0xFFFC);
    assert_eq!(gb.memory.read_u16(0xFFFC), 0x103);
}

#[test]
fn test_call_cc_taken() {
    let mut gb = Gameboy::new();
    gb.cpu.program_counter = 0x100;
    gb.cpu.stack_pointer = 0xFFFE;
    gb.memory.write_u16(0x101, 0x1234);
    gb.cpu.set_flag(Gbz80::FLAG_Z, false);
    gb.call(0xC4).unwrap(); // CALL NZ
    assert_eq!(gb.cpu.program_counter, 0x1234);
    assert_eq!(gb.cpu.stack_pointer, 0xFFFC);
}

#[test]
fn test_call_cc_not_taken() {
    let mut gb = Gameboy::new();
    gb.cpu.program_counter = 0x100;
    gb.cpu.stack_pointer = 0xFFFE;
    gb.memory.write_u16(0x101, 0x1234);
    gb.cpu.set_flag(Gbz80::FLAG_Z, true);
    gb.call(0xC4).unwrap(); // CALL NZ
    assert_eq!(gb.cpu.program_counter, 0x103);
    assert_eq!(gb.cpu.stack_pointer, 0xFFFE);
}

#[test]
fn test_ret() {
    let mut gb = Gameboy::new();
    gb.cpu.stack_pointer = 0xFFFC;
    gb.memory.write_u16(0xFFFC, 0x103);
    gb.ret(0xC9).unwrap();
    assert_eq!(gb.cpu.program_counter, 0x103);
    assert_eq!(gb.cpu.stack_pointer, 0xFFFE);
}

#[test]
fn test_ret_cc_taken_and_not() {
    // RET Z taken
    let mut gb = Gameboy::new();
    gb.cpu.stack_pointer = 0xFFFC;
    gb.memory.write_u16(0xFFFC, 0x1234);
    gb.cpu.set_flag(Gbz80::FLAG_Z, true);
    gb.ret(0xC8).unwrap();
    assert_eq!(gb.cpu.program_counter, 0x1234);
    assert_eq!(gb.cpu.stack_pointer, 0xFFFE);

    // RET Z not taken: 1-byte opcode
    let mut gb2 = Gameboy::new();
    gb2.cpu.program_counter = 0x200;
    gb2.cpu.stack_pointer = 0xFFFC;
    gb2.cpu.set_flag(Gbz80::FLAG_Z, false);
    gb2.ret(0xC8).unwrap();
    assert_eq!(gb2.cpu.program_counter, 0x201);
    assert_eq!(gb2.cpu.stack_pointer, 0xFFFC);
}

#[test]
fn test_reti() {
    let mut gb = Gameboy::new();
    gb.cpu.stack_pointer = 0xFFFC;
    gb.memory.write_u16(0xFFFC, 0x300);
    gb.reti(0xD9).unwrap();
    assert_eq!(gb.cpu.program_counter, 0x300);
    assert_eq!(gb.cpu.stack_pointer, 0xFFFE);
}

#[test]
fn test_rst() {
    let vectors = [0x00, 0x08, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38];
    let opcodes = [0xC7, 0xCF, 0xD7, 0xDF, 0xE7, 0xEF, 0xF7, 0xFF];
    for (&opcode, &vec) in opcodes.iter().zip(vectors.iter()) {
        let mut gb = Gameboy::new();
        gb.cpu.program_counter = 0x100;
        gb.cpu.stack_pointer = 0xFFFE;
        gb.rst(opcode).unwrap();
        assert_eq!(gb.cpu.program_counter, vec, "RST 0x{:02X} failed", opcode);
        assert_eq!(gb.cpu.stack_pointer, 0xFFFC);
        assert_eq!(gb.memory.read_u16(0xFFFC), 0x101, "RST push failed");
    }
}
