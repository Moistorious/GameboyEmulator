use crate::cpu::{Reg8, Reg16};
use crate::gameboy::Gameboy;

fn setup(pc: u16, opcode: u8) -> Gameboy {
    let mut gb = Gameboy::new();
    gb.cpu.program_counter = pc;
    gb.memory.write_u8(pc, opcode);
    gb
}

#[test]
fn test_decode_opcode_groups() {
    let gb = Gameboy::new();
    assert_eq!(gb.decode_opcode(0x01), (0, 0, 1)); // LD BC,nn
    assert_eq!(gb.decode_opcode(0x09), (0, 1, 1)); // ADD HL,BC
    assert_eq!(gb.decode_opcode(0x2A), (0, 5, 2)); // LD A,(HL+)
    assert_eq!(gb.decode_opcode(0x76), (1, 6, 6)); // HALT
    assert_eq!(gb.decode_opcode(0xAF), (2, 5, 7)); // XOR A
    assert_eq!(gb.decode_opcode(0xC6), (3, 0, 6)); // ADD A,n
}

#[test]
fn test_step_routes_ld_rr_nn() {
    let cases = [
        (0x01, Reg16::BC),
        (0x11, Reg16::DE),
        (0x21, Reg16::HL),
        (0x31, Reg16::SP),
    ];
    for (opcode, reg) in cases {
        let mut gb = setup(0x100, opcode);
        gb.memory.write_u8(0x101, 0x34);
        gb.memory.write_u8(0x102, 0x12);

        gb.step();

        assert_eq!(gb.cpu.reg16(reg), 0x1234, "opcode 0x{opcode:02X} did not route to ld_rr_nn");
        assert_eq!(gb.cpu.program_counter, 0x103, "PC advance wrong for 0x{opcode:02X}");
    }
}

#[test]
fn test_step_routes_add_hl_rr() {
    let mut gb = setup(0x100, 0x09); // ADD HL,BC
    gb.cpu.set_hl(0x1000);
    gb.cpu.set_bc(0x0234);
    gb.step();
    assert_eq!(gb.cpu.hl(), 0x1234, "0x09 did not route to add_hl_rr(BC)");
    assert_eq!(gb.cpu.program_counter, 0x101);

    let mut gb = setup(0x100, 0x19); // ADD HL,DE
    gb.cpu.set_hl(0x1000);
    gb.cpu.set_de(0x0234);
    gb.step();
    assert_eq!(gb.cpu.hl(), 0x1234, "0x19 did not route to add_hl_rr(DE)");
    assert_eq!(gb.cpu.program_counter, 0x101);

    let mut gb = setup(0x100, 0x29); // ADD HL,HL
    gb.cpu.set_hl(0x1000);
    gb.step();
    assert_eq!(gb.cpu.hl(), 0x2000, "0x29 did not route to add_hl_rr(HL)");
    assert_eq!(gb.cpu.program_counter, 0x101);

    let mut gb = setup(0x100, 0x39); // ADD HL,SP
    gb.cpu.set_hl(0x1000);
    gb.cpu.stack_pointer = 0x0234;
    gb.step();
    assert_eq!(gb.cpu.hl(), 0x1234, "0x39 did not route to add_hl_rr(SP)");
    assert_eq!(gb.cpu.program_counter, 0x101);
}

#[test]
fn test_step_routes_ld_rr_a() {
    // 0x02 LD (BC),A
    let mut gb = setup(0x100, 0x02);
    gb.cpu.set_bc(0xC000);
    gb.cpu.a = 0x11;
    gb.step();
    assert_eq!(gb.memory.read_u8(0xC000), 0x11, "0x02 did not route to ld_rr_a(BC)");
    assert_eq!(gb.cpu.program_counter, 0x101);

    // 0x12 LD (DE),A
    let mut gb = setup(0x100, 0x12);
    gb.cpu.set_de(0xC000);
    gb.cpu.a = 0x22;
    gb.step();
    assert_eq!(gb.memory.read_u8(0xC000), 0x22, "0x12 did not route to ld_rr_a(DE)");
    assert_eq!(gb.cpu.program_counter, 0x101);
}

#[test]
fn test_step_routes_ld_a_rr() {
    // 0x0A LD A,(BC)
    let mut gb = setup(0x100, 0x0A);
    gb.cpu.set_bc(0xC000);
    gb.memory.write_u8(0xC000, 0x55);
    gb.step();
    assert_eq!(gb.cpu.a, 0x55, "0x0A did not route to ld_a_rr(BC)");
    assert_eq!(gb.cpu.program_counter, 0x101);

    // 0x1A LD A,(DE)
    let mut gb = setup(0x100, 0x1A);
    gb.cpu.set_de(0xC000);
    gb.memory.write_u8(0xC000, 0x66);
    gb.step();
    assert_eq!(gb.cpu.a, 0x66, "0x1A did not route to ld_a_rr(DE)");
    assert_eq!(gb.cpu.program_counter, 0x101);
}

#[test]
fn test_step_routes_ld_hli_a() {
    // 0x22 LD (HL+),A
    let mut gb = setup(0x100, 0x22);
    gb.cpu.set_hl(0xC000);
    gb.cpu.a = 0xAB;
    gb.step();
    assert_eq!(gb.memory.read_u8(0xC000), 0xAB, "0x22 did not store A at (HL)");
    assert_eq!(gb.cpu.hl(), 0xC001, "0x22 did not increment HL");
    assert_eq!(gb.cpu.program_counter, 0x101);
}

#[test]
fn test_step_routes_ld_a_hli() {
    // 0x2A LD A,(HL+)
    let mut gb = setup(0x100, 0x2A);
    gb.cpu.set_hl(0xC000);
    gb.memory.write_u8(0xC000, 0xCD);
    gb.step();
    assert_eq!(gb.cpu.a, 0xCD, "0x2A did not load A from (HL)");
    assert_eq!(gb.cpu.hl(), 0xC001, "0x2A did not increment HL");
    assert_eq!(gb.cpu.program_counter, 0x101);
}

#[test]
fn test_step_routes_ld_hld_a() {
    // 0x32 LD (HL-),A
    let mut gb = setup(0x100, 0x32);
    gb.cpu.set_hl(0xC000);
    gb.cpu.a = 0xBB;
    gb.step();
    assert_eq!(gb.memory.read_u8(0xC000), 0xBB, "0x32 did not store A at (HL)");
    assert_eq!(gb.cpu.hl(), 0xBFFF, "0x32 did not decrement HL");
    assert_eq!(gb.cpu.program_counter, 0x101);
}

#[test]
fn test_step_routes_ld_a_hld() {
    // 0x3A LD A,(HL-)
    let mut gb = setup(0x100, 0x3A);
    gb.cpu.set_hl(0xC000);
    gb.memory.write_u8(0xC000, 0xDC);
    gb.step();
    assert_eq!(gb.cpu.a, 0xDC, "0x3A did not load A from (HL)");
    assert_eq!(gb.cpu.hl(), 0xBFFF, "0x3A did not decrement HL");
    assert_eq!(gb.cpu.program_counter, 0x101);
}

#[test]
fn test_step_routes_ld_r_n() {
    let cases = [
        (0x06, Reg8::B),
        (0x0E, Reg8::C),
        (0x16, Reg8::D),
        (0x1E, Reg8::E),
        (0x26, Reg8::H),
        (0x2E, Reg8::L),
        (0x3E, Reg8::A),
    ];
    for (opcode, reg) in cases {
        let mut gb = setup(0x100, opcode);
        gb.memory.write_u8(0x101, 0x42);

        gb.step();

        assert_eq!(gb.cpu.reg8(reg), 0x42, "0x{opcode:02X} did not route to ld_r_n");
        assert_eq!(gb.cpu.program_counter, 0x102, "PC advance wrong for 0x{opcode:02X}");
    }

    // 0x36 LD (HL),n
    let mut gb = setup(0x100, 0x36);
    gb.cpu.set_hl(0xC000);
    gb.memory.write_u8(0x101, 0x77);
    gb.step();
    assert_eq!(gb.memory.read_u8(0xC000), 0x77, "0x36 did not store n at (HL)");
    assert_eq!(gb.cpu.program_counter, 0x102);
}

#[test]
fn test_step_routes_ld_r_r() {
    // 0x78 LD A,B
    let mut gb = setup(0x100, 0x78);
    gb.cpu.a = 0x00;
    gb.cpu.b = 0x34;
    gb.step();
    assert_eq!(gb.cpu.a, 0x34, "0x78 did not route to ld_r_r(A,B)");
    assert_eq!(gb.cpu.program_counter, 0x101);

    // 0x47 LD B,A
    let mut gb = setup(0x100, 0x47);
    gb.cpu.b = 0x00;
    gb.cpu.a = 0x56;
    gb.step();
    assert_eq!(gb.cpu.b, 0x56, "0x47 did not route to ld_r_r(B,A)");
    assert_eq!(gb.cpu.program_counter, 0x101);

    // 0x7E LD A,(HL)
    let mut gb = setup(0x100, 0x7E);
    gb.cpu.set_hl(0xC000);
    gb.memory.write_u8(0xC000, 0x88);
    gb.step();
    assert_eq!(gb.cpu.a, 0x88, "0x7E did not route to ld_r_r(A,HLIndirect)");
    assert_eq!(gb.cpu.program_counter, 0x101);

    // 0x71 LD (HL),C
    let mut gb = setup(0x100, 0x71);
    gb.cpu.set_hl(0xC000);
    gb.cpu.c = 0x66;
    gb.step();
    assert_eq!(gb.memory.read_u8(0xC000), 0x66, "0x71 did not route to ld_r_r(HLIndirect,C)");
    assert_eq!(gb.cpu.program_counter, 0x101);
}

#[test]
fn test_step_routes_halt() {
    let mut gb = setup(0x100, 0x76);
    gb.running = true;
    gb.step();
    assert!(!gb.running, "0x76 did not route to halt");
    assert_eq!(gb.cpu.program_counter, 0x101);

    // 0x40 LD B,B must NOT halt
    let mut gb = setup(0x100, 0x40);
    gb.running = true;
    gb.step();
    assert!(gb.running, "0x40 was mistaken for HALT");
    assert_eq!(gb.cpu.program_counter, 0x101);
}

#[test]
fn test_step_routes_nop() {
    let mut gb = setup(0x100, 0x00);
    gb.step();
    assert_eq!(gb.cpu.program_counter, 0x101);
}

#[test]
fn test_step_routes_alu_group() {
    let mut gb = setup(0x100, 0x80); // ADD A,B
    gb.cpu.a = 0x10;
    gb.cpu.b = 0x20;
    gb.step();
    assert_eq!(gb.cpu.a, 0x30, "0x80 did not route to ALU group");
}

#[test]
fn test_step_routes_control_flow_group() {
    let mut gb = setup(0x100, 0xC6); // ADD A,n
    gb.cpu.a = 0x10;
    gb.memory.write_u8(0x101, 0x20);
    gb.step();
    assert_eq!(gb.cpu.a, 0x30, "0xC6 did not route to control flow group");
}