use eframe::{Frame, egui::{self, Ui}};
use crate::{cpu::{Flag}, gameboy::Gameboy};
use egui_memory_editor::MemoryEditor;
use rfd::FileDialog;

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

                // Load BIOS / boot ROM file
                if ui.button("📂 Load BIOS").clicked() {
                    if let Some(path) = FileDialog::new()
                        .add_filter("BIOS ROM", &["bin", "rom"])
                        .pick_file()
                    {
                        self.gameboy.load_rom(0x00, path.to_str().unwrap_or(""));
                    }
                }
            });
        });
        egui::Panel::left("cpu_panel").show(ui, |ui| {
            ui.heading("CPU Registers");
            ui.separator();
            
            egui::Grid::new("registers_grid").num_columns(2).show(ui, |ui| {
                ui.label("PC:");
                ui.add(egui::DragValue::new(&mut self.gameboy.cpu.program_counter).hexadecimal(4, false, true));
                ui.end_row();
                ui.label("SP:");
                ui.add(egui::DragValue::new(&mut self.gameboy.cpu.stack_pointer).hexadecimal(4, false, true));
                ui.end_row();
                ui.label("A:");
                ui.add(egui::DragValue::new(&mut self.gameboy.cpu.a).hexadecimal(2, false, true));
                ui.end_row();
                ui.label("F:");
                ui.add(egui::DragValue::new(&mut self.gameboy.cpu.f).hexadecimal(2, false, true));
                ui.end_row();
                ui.label("B:");
                ui.add(egui::DragValue::new(&mut self.gameboy.cpu.b).hexadecimal(2, false, true));
                ui.end_row();
                ui.label("C:");
                ui.add(egui::DragValue::new(&mut self.gameboy.cpu.c).hexadecimal(2, false, true));
                ui.end_row();
                ui.label("D:");
                ui.add(egui::DragValue::new(&mut self.gameboy.cpu.d).hexadecimal(2, false, true));
                ui.end_row();
                ui.label("E:");
                ui.add(egui::DragValue::new(&mut self.gameboy.cpu.e).hexadecimal(2, false, true));
                ui.end_row();
                ui.label("H:");
                ui.add(egui::DragValue::new(&mut self.gameboy.cpu.h).hexadecimal(2, false, true));
                ui.end_row();
                ui.label("L:");
                ui.add(egui::DragValue::new(&mut self.gameboy.cpu.l).hexadecimal(2, false, true));
                ui.end_row();
            });

            ui.add_space(10.0);
            ui.heading("Flags");
            ui.separator();
            ui.horizontal(|ui| {
                let mut z = self.gameboy.cpu.get_flag(Flag::Z);
                let mut n = self.gameboy.cpu.get_flag(Flag::N);
                let mut h = self.gameboy.cpu.get_flag(Flag::H);
                let mut c = self.gameboy.cpu.get_flag(Flag::C);
                if ui.checkbox(&mut z, "Z").changed() {
                    self.gameboy.cpu.set_flag(Flag::Z, z);
                }
                if ui.checkbox(&mut n, "N").changed() {
                    self.gameboy.cpu.set_flag(Flag::N, n);
                }
                if ui.checkbox(&mut h, "H").changed() {
                    self.gameboy.cpu.set_flag(Flag::H, h);
                }
                if ui.checkbox(&mut c, "C").changed() {
                    self.gameboy.cpu.set_flag(Flag::C, c);
                }
            });
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