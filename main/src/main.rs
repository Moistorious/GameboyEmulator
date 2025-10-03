use num_enum::FromPrimitive;
use main::gameboy::Gameboy;

#[derive(Debug, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Opcode {
    Nop = 0x00,
    LdSpU16 = 0x31,
    XorAA = 0xAf,
    #[num_enum(default)]
    Undefined,
}

fn main() {
    let mut gameboy = Gameboy::new();
    let _rom_size = gameboy.load_rom(0x00, "./dmg_boot.bin");
    //let video_memory: [u8; 8*1024] = [0; 8*1024];

    loop {
        if gameboy.running {
            gameboy.execute_next();
        } else {
            break;
        }
    }
}
