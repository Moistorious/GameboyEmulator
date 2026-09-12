use eframe::{Frame, egui::{self, Ui}};
use crate::{cpu::{Flag}, gameboy::Gameboy};
use egui_memory_editor::MemoryEditor;

pub struct DebuggerUI {
    gameboy: Gameboy,
    memory_editor: MemoryEditor
}

impl Default for DebuggerUI {
    fn default() -> Self {
        DebuggerUI { gameboy: Gameboy::new(), memory_editor: MemoryEditor::new().with_address_range("All RAM", 0x0000..0xFFFF) }
    }
}

impl DebuggerUI {
    pub fn new (gameboy: Gameboy) -> Self {
        Self {
            gameboy: gameboy,
            memory_editor: MemoryEditor::new().with_address_range("All RAM", 0x0000..0xFFFF )
        }
    }
}

impl eframe::App for DebuggerUI {
    
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        // egui::TopBottomPanel::top("controls").show(ctx, |ui| {
        //     ui.horizontal(|ui| {
        //         if ui.button("step").clicked() {
        //             self.cpu.step();
        //         }
        //     });
        // });
        egui::Panel::top("controls").show(ui, |ui| {
            ui.horizontal(|ui| {
                  
                // Step single instruction
                if ui.button("⏭ Step Instruction").clicked() {
                    self.gameboy.step();
                }
            });
        });
        egui::Panel::left("cpu_panel").show(ui, |ui| {
            ui.heading("CPU Registers");
            ui.separator();
            
            egui::Grid::new("registers_grid").num_columns(2).show(ui, |ui| {
                ui.label("PC:"); ui.monospace(format!("0x{:04X}", self.gameboy.cpu.program_counter)); ui.end_row();
                ui.label("SP:"); ui.monospace(format!("0x{:04X}", self.gameboy.cpu.stack_pointer)); ui.end_row();
                ui.label("A:");  ui.monospace(format!("0x{:02X}", self.gameboy.cpu.a)); ui.end_row();
                ui.label("F:");  ui.monospace(format!("0x{:02X}", self.gameboy.cpu.f)); ui.end_row();
                ui.label("B:");  ui.monospace(format!("0x{:02X}", self.gameboy.cpu.b)); ui.end_row();
                ui.label("C:");  ui.monospace(format!("0x{:02X}", self.gameboy.cpu.c)); ui.end_row();
                ui.label("D:");  ui.monospace(format!("0x{:02X}", self.gameboy.cpu.d)); ui.end_row();
                ui.label("E:");  ui.monospace(format!("0x{:02X}", self.gameboy.cpu.e)); ui.end_row();
                ui.label("H:");  ui.monospace(format!("0x{:02X}", self.gameboy.cpu.h)); ui.end_row();
                ui.label("L:");  ui.monospace(format!("0x{:02X}", self.gameboy.cpu.l)); ui.end_row();
            });

            ui.add_space(10.0);
            ui.heading("Flags");
            ui.separator();
            ui.monospace(format!(
                "Z: {} | N: {} | H: {} | C: {}",
                self.gameboy.cpu.get_flag(Flag::Z) as u8,
                self.gameboy.cpu.get_flag(Flag::N) as u8,
                self.gameboy.cpu.get_flag(Flag::H) as u8,
                self.gameboy.cpu.get_flag(Flag::C) as u8
            ));
        });
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Memory Inspector");
            ui.separator();
            
            // Draw hex viewer bound to the Bus
            self.memory_editor.draw_editor_contents(
                ui,
                &mut self.gameboy.memory,
                |bus, address| Some(bus.read_u8(address as u16)), // Read callback
                |bus, address, val| bus.write_u8(address as u16, val), // Write callback
            );
        });
    }
    
    fn logic(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        _ = (ctx, frame);
    }
    
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}
    
    fn on_exit(&mut self) {}
    
    fn auto_save_interval(&self) -> core::time::Duration {
        core::time::Duration::from_secs(30)
    }
    
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).to_normalized_gamma_f32()
    
        // _visuals.window_fill() would also be a natural choice
    }
    
    fn persist_egui_memory(&self) -> bool {
        true
    }
    
    fn raw_input_hook(&mut self, _ctx: &egui::Context, _raw_input: &mut egui::RawInput) {}
}