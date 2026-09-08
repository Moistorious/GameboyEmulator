use crate::cpu::Gbz80;
use crate::gameboy::Gameboy;

#[test]
fn test_jp_nn() {
    let mut gb = Gameboy::new();
    gb.cpu.program_counter = 0x100;
    gb.memory.write_u16(0x101, 0x1234);
    
    gb.jp(0xC3); // JP nn
    
    assert_eq!(gb.cpu.program_counter, 0x1234);
}

#[test]
fn test_jp_cc_nn() {
    let mut gb = Gameboy::new();
    
    // NZ condition, Zero flag is NOT set
    gb.cpu.program_counter = 0x100;
    gb.memory.write_u16(0x101, 0x1234);
    gb.cpu.set_flag(Gbz80::FLAG_Z, false);
    gb.jp(0xC2); // JP NZ, nn
    assert_eq!(gb.cpu.program_counter, 0x1234);

    // NZ condition, Zero flag IS set -> No jump
    gb.cpu.program_counter = 0x100;
    gb.cpu.set_flag(Gbz80::FLAG_Z, true);
    gb.jp(0xC2); // JP NZ, nn
    // JP nn is 3 bytes, if no jump, PC should be at 0x103
    assert_eq!(gb.cpu.program_counter, 0x103);
}

#[test]
fn test_jr_n() {
    let mut gb = Gameboy::new();
    gb.cpu.program_counter = 0x100;
    gb.memory.write_u8(0x101, 0x05); // JR +5
    
    gb.jr(0x18);
    
    // JR is 2 bytes, so PC becomes 0x102 + 5 = 0x107
    assert_eq!(gb.cpu.program_counter, 0x107);
}

#[test]
fn test_jr_n_negative() {
    let mut gb = Gameboy::new();
    gb.cpu.program_counter = 0x100;
    gb.memory.write_u8(0x101, 0xFB as u8); // JR -5 (0xFB is -5 in 2's complement)
    
    gb.jr(0x18);
    
    // JR is 2 bytes, so PC becomes 0x102 - 5 = 0x0FD
    assert_eq!(gb.cpu.program_counter, 0x0FD);
}

#[test]
fn test_call_nn() {
    let mut gb = Gameboy::new();
    gb.cpu.program_counter = 0x100;
    gb.cpu.stack_pointer = 0xFFFE;
    gb.memory.write_u16(0x101, 0x1234);
    
    gb.call(0xCD); // CALL nn
    
    assert_eq!(gb.cpu.program_counter, 0x1234);
    assert_eq!(gb.cpu.stack_pointer, 0xFFFC);
    assert_eq!(gb.memory.read_u16(0xFFFC), 0x103); // Return address pushed
}

#[test]
fn test_ret() {
    let mut gb = Gameboy::new();
    gb.cpu.stack_pointer = 0xFFFC;
    gb.memory.write_u16(0xFFFC, 0x103);
    
    gb.ret(0xC9); // RET
    
    assert_eq!(gb.cpu.program_counter, 0x103);
    assert_eq!(gb.cpu.stack_pointer, 0xFFFE);
}

#[test]
fn test_ret_cc() {
    let mut gb = Gameboy::new();
    
    // Z condition, Zero flag IS set
    gb.cpu.program_counter = 0x200;
    gb.cpu.stack_pointer = 0xFFFC;
    gb.memory.write_u16(0xFFFC, 0x1234);
    gb.cpu.set_flag(Gbz80::FLAG_Z, true);
    gb.ret(0xC8); // RET Z
    assert_eq!(gb.cpu.program_counter, 0x1234);
    assert_eq!(gb.cpu.stack_pointer, 0xFFFE);

    // Z condition, Zero flag is NOT set -> No ret
    gb.cpu.program_counter = 0x200;
    gb.cpu.stack_pointer = 0xFFFC;
    gb.cpu.set_flag(Gbz80::FLAG_Z, false);
    gb.ret(0xC8); // RET Z
    assert_eq!(gb.cpu.program_counter, 0x201); // RET cc is 1 byte
    assert_eq!(gb.cpu.stack_pointer, 0xFFFC);
}

#[test]
fn test_rst() {
    let mut gb = Gameboy::new();
    gb.cpu.program_counter = 0x100;
    gb.cpu.stack_pointer = 0xFFFE;
    
    gb.rst(0xCF); // RST 08h
    
    assert_eq!(gb.cpu.program_counter, 0x0008);
    assert_eq!(gb.cpu.stack_pointer, 0xFFFC);
    assert_eq!(gb.memory.read_u16(0xFFFC), 0x101); // RST is 1 byte
}
