use byteorder::{BigEndian, ByteOrder};

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Reg8 {
    B = 0,
    C,
    D,
    E,
    H,
    L,
    F,
    A,
}

impl TryFrom<u8> for Reg8 {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Reg8::B),
            1 => Ok(Reg8::C),
            2 => Ok(Reg8::D),
            3 => Ok(Reg8::E),
            4 => Ok(Reg8::H),
            5 => Ok(Reg8::L),
            6 => Ok(Reg8::F),
            7 => Ok(Reg8::A),
            _ => Err("Cannot convert value to reg8"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Reg16 {
    BC = 0,
    DE = 2,
    HL = 4,
    FA = 6,
}

pub struct Gbz80 {
    regs: [u8; 8], // B=0, C=1, D=2, E=3, H=4, L=5, F=6, A=7,
    pub z: bool,
    pub n: bool,
    pub h: bool,
    pub c: bool,
    pub stack_pointer: u16,
    pub program_counter: u16,
}

impl Gbz80 {
    pub fn new() -> Self {
        Gbz80 {
            z: false,
            n: false,
            h: false,
            c: false,
            regs: [0; 8],
            stack_pointer: 0,
            program_counter: 0,
        }
    }

    pub fn reg16(&self, reg: Reg16) -> u16 {
        let size = reg as usize;
        BigEndian::read_u16(&self.regs[size  .. size + 2])
    }
    
    pub fn reg8(&self, reg: Reg8) -> u8 {
        self.regs[reg as usize]
    }

    pub fn write_reg8(&mut self, reg: Reg8, value: u8) {
        self.regs[reg as usize] = value;
    }

    pub fn write_reg16(&mut self, reg: Reg16, value: u16)  {
        let size = reg as usize;
        BigEndian::write_u16(&mut self.regs[size..size + 2], value)
    }

    pub fn bc(&self) -> u16 {
        BigEndian::read_u16(&self.regs[Reg8::B as usize..Reg8::C as usize])
    }
    
    pub fn set_bc(&mut self, val: u16) {
        BigEndian::write_u16(&mut self.regs[Reg8::B as usize..Reg8::C as usize], val);
    }
}
