#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use num_enum::FromPrimitive;
use main::debugger::DebuggerUI;
use eframe::egui;

#[derive(Debug, Eq, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Opcode {
    Nop = 0x00,
    LdSpU16 = 0x31,
    XorAA = 0xAf,
    #[num_enum(default)]
    Undefined,
}

fn main() -> eframe::Result {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0,240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Gameboy Emulator",
        options,
        Box::new(|_cc| {
            Ok(Box::<DebuggerUI>::default())
        }),
    )

    // let mut gameboy = Gameboy::new();
    // let _rom_size = gameboy.load_rom(0x00, "./dmg_boot.bin");
    // //let video_memory: [u8; 8*1024] = [0; 8*1024];

    // loop {
    //     if gameboy.running {
    //         gameboy.step();
    //     } else {
    //         break;
    //     }
    // }
}
