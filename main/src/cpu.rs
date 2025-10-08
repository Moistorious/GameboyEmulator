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

impl Reg8 {
    pub fn from_u8(value: u8) -> Reg8 {
        let val : Reg8 = Reg8::try_from(value).unwrap();
        val
    }
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
    AF = 6,
}

pub struct Gbz80 {
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub f: u8,
    pub a: u8,
    pub stack_pointer: u16,
    pub program_counter: u16,
}

impl Gbz80 {
    pub const FLAG_Z: u8 = 1 << 7;
    pub const FLAG_N: u8 = 1 << 6;
    pub const FLAG_H: u8 = 1 << 5;

    pub fn new() -> Self {
        Gbz80 {
            stack_pointer: 0,
            program_counter: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
        }
    }

    pub fn set_flags(&mut self, z: bool, n: bool, h: bool) {
        self.set_flag(Self::FLAG_Z, z);
        self.set_flag(Self::FLAG_N, n);
        self.set_flag(Self::FLAG_H, h);
    }

    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }
    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = value as u8;
    }

    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }
    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }

    pub fn set_flag(&mut self, flag: u8, value: bool){
        if value {
            self.write_reg8(Reg8::F, self.reg8(Reg8::F) | flag);
        }else{
            self.write_reg8(Reg8::F, self.reg8(Reg8::F) & flag);
        }
    }

    pub fn reg16(&self, reg: Reg16) -> u16 {
        match reg {
            Reg16::BC => self.bc(),
            Reg16::DE => self.de(),
            Reg16::HL => self.hl(),
            Reg16::AF => self.af(),
        }
    }

    pub fn reg8(&self, reg: Reg8) -> u8 {
        match reg {
            Reg8::B => self.b,
            Reg8::C => self.c,
            Reg8::D => self.d,
            Reg8::E => self.e,
            Reg8::H => self.h,
            Reg8::L => self.l,
            Reg8::F => self.f,
            Reg8::A => self.a,
        }
    }

    pub fn write_reg8(&mut self, reg: Reg8, value: u8) {
        match reg {
            Reg8::B => self.b = value,
            Reg8::C => self.c = value,
            Reg8::D => self.d = value,
            Reg8::E => self.e = value,
            Reg8::H => self.h = value,
            Reg8::L => self.l = value,
            Reg8::F => self.f = value,
            Reg8::A => self.a = value,
        }
    }

    pub fn write_reg16(&mut self, reg: Reg16, value: u16)  {
        match reg {
            Reg16::BC => self.set_bc(value),
            Reg16::DE => self.set_de(value),
            Reg16::HL => self.set_hl(value),
            Reg16::AF => self.set_af(value),
        }
    }

}
