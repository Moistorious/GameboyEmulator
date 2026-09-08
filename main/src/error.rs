use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmulatorError {
    #[error("CPU encountered an invalid opcode: {0:#X} at PC: {1:#X}")]
    InvalidOpcode(u8, u16),

    #[error("Unimplemented Memory Mapper (MBC): {0}")]
    UnsupportedMBC(u8),

    #[error("Attempted illegal memory write to read-only address {0:#X}")]
    IllegalMemoryWrite(u16),

    #[error("Failed to load ROM file")]
    RomLoadError(#[from] std::io::Error),

    #[error("Invalid Operand")]
    InvalidOperand(u8),

    #[error("CPU encountered an Unimplemented opcode: {0:#X} at PC: {1:#X}")]
    NotImplementedOpcode(u8, u16),
}