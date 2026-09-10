# Gameboy Emulator - Implementation Checklist

## Test status

`cargo test` → **163 tests: 51 passed, 112 failed** (0 ignored).

Every failure is either a `NotImplementedOpcode` (stub returns `Err`) or a logic bug in a partially-written handler.

---

## 1. CPU Core

### 1.1 Arithmetic / Logic (ALU)

- [ ] **Fix `or`** — `alu.rs:74` uses `opcode & 0xf == 0xE` to detect `(HL)`/`n8`, which is wrong. `OR A,(HL)` = `0xB6` and `OR A,n` = `0xF6`; detect via `opcode & 0x07 == 6` and `0xF6` (currently reads `F` register → wrong results).
- [ ] **Implement `xor`** — body is commented out (`alu.rs:92`); operand logic matches `or`.
- [ ] **Implement `and`** (`alu.rs:61`).
- [ ] **Implement `cp`** (`cp.rs`) — like `sub` but doesn't write `A`, sets `N`.
- [ ] **Implement `sub`** (`alu.rs:55`) — sets `N=1`, Z/H/C from borrow.
- [ ] **Implement `sbc`** (`alu.rs:49`) — `sub` + carry-in.
- [ ] **Implement `adc`** (`alu.rs:67`) — `add` + carry-in.
- [ ] **Implement 16-bit add in `add`** (`alu.rs:39`) — handle `ADD HL,BC/DE/HL/SP` (`0x09/0x19/0x29/0x39`) and `ADD SP,n` (`0xE8`); use `flags_from_16bit_add`, preserve Z.

### 1.2 Inc / Dec

- [ ] **Implement `inc`/`dec`** (`inc_dec.rs`) — 8-bit (`Z/H`, preserve `C`, set `N`) and 16-bit (`BC/DE/HL/SP`, no flags).

### 1.3 Bit Operations (CB prefix)

- [ ] **Fix `bit`** (`bit.rs:7`) — read `(HL)` memory when `opcode & 7 == 6`; set `H=1`, `N=0`, and **preserve C** (currently clears it).
- [ ] **Implement `res`** (`bit_manipulation.rs:9`).
- [ ] **Implement `set`** (`bit_manipulation.rs:5`).
- [ ] **Implement shifts/rotates** (`shifts.rs`) — `rlc`, `rrc`, `rl`, `rr`, `sla`, `sra`, `srl`, `swap` (all need reg + `(HL)` variants, Z/C flags).

### 1.4 Control Flow

- [ ] **Implement control flow** (`control_flow.rs`) — `jp`/`jp cc`/`jp (HL)`, `jr`/`jr cc`, `call`/`call cc`, `ret`/`ret cc`, `reti`, `rst` (push/pop PC on stack).
- [ ] **Stack helpers** — implement push/pop (needed by `call`/`ret`/`rst` and interrupts); handle SP wraparound at 0x0000.

### 1.5 Misc / Flag / System

- [ ] **Implement misc** (`misc.rs`) — `cpl`, `ccf`, `scf`, `daa`, `di`, `ei`, `stop`.
- [ ] **`daa`** — decimal adjust (addition + subtraction modes, full carry/half-carry behavior).

### 1.6 Dispatch

- [ ] **Wire up `step()` dispatch** (`gameboy.rs:79`) — currently missing `inc/dec`, control flow, `ADD HL/SP`, and misc opcodes; add them so `step()` actually routes these instead of `InvalidOpcode`.
- [ ] **Instruction cycle timing** — track T-cycles per opcode and expose them (needed for PPU/timer/sound sync).

---

## 2. Memory & Cartridge (MBC)

- [ ] **Cartridge header parsing** (`cartridge.rs`) — title, CGB flag, cartridge type (MBC), ROM size, RAM size, checksum.
- [ ] **ROM-only (0x00)** — map ROM to 0x0000–0x7FFF.
- [ ] **MBC1** — ROM/RAM banking, mode select (ROM/RAM banking mode), RAM enable.
- [ ] **MBC2** — fixed ROM banking + 512×4-bit built-in RAM.
- [ ] **MBC3** — ROM/RAM banking + RTC registers (seconds/minutes/hours/days/latch).
- [ ] **MBC5** — full ROM/RAM banking, rumble bit.
- [ ] **RAM banking** — map external RAM (0xA000–0xBFFF) through the selected bank.
- [ ] **Battery-backed save (.sav)** — persist cartridge RAM to disk (load on start, save on exit / periodically).
- [ ] **Unsupported MBC error** — surface `UnsupportedMBC` (already in `error.rs`) for MBC6/7/etc.
- [ ] **Echo RAM** — verify 0xE000–0xFDFF mirrors 0xC000–0xDDFF correctly (currently maps from 0xE000, wrong base).

---

## 3. Interrupts

- [ ] **IME (interrupt master enable)** — the `di`/`ei` ops should set/clear IME; `ei` delayed by one instruction.
- [ ] **IE register (0xFFFF)** and **IF register (0xFF0F)** — request/flush flags.
- [ ] **Interrupt dispatch** — check IF & IE after each instruction; push PC, jump to vector, clear IME + flag.
- [ ] **Interrupt vectors** — VBlank 0x40, LCD STAT 0x48, Timer 0x50, Serial 0x58, Joypad 0x60.
- [ ] **Interrupt priorities & latency** — HALT wakeup on IF, `reti` re-enables IME.

---

## 4. PPU / Display

### 4.1 LCD registers

- [ ] **LCDC (0xFF40)** — BG/window enable, obj enable, window/bg tile data select, bg tilemap select, obj size, bg/window priority.
- [ ] **STAT (0xFF41)** — mode, LY=LYC, interrupt sources; LCD STAT interrupt.
- [ ] **LY (0xFF44)** / **LYC (0xFF45)** — scanline counter + compare.
- [ ] **SCY/SCX (0xFF42/0xFF43)** — scroll.
- [ ] **WY/WX (0xFF4A/0xFF4B)** — window position.
- [ ] **BGP/OBP0/OBP1 (0xFF47–0xFF49)** — palettes.

### 4.2 Rendering

- [ ] **Modes 0–3** — HBlank, VBlank, OAM scan, drawing; mode timing in T-cycles.
- [ ] **Background rendering** — tile map fetch, tile data fetch, wrapping.
- [ ] **Window rendering** — window line counter, WX/WY behavior.
- [ ] **Sprites** — 8×8 / 8×16, priority, X-flip/Y-flip, palette, OAM fetch (max 10 per line), sprite behind BG.
- [ ] **OAM DMA (0xFF46)** — copy 0xA0 bytes from given source to OAM.
- [ ] **Pixel FIFO / timing accuracy** — STAT interrupt timing, LY increment, mode transitions.

### 4.3 Framebuffer output

- [ ] **Framebuffer** — produce 160×144 RGB (or indexed) output buffer.
- [ ] **Color mapping** — DMG 4 shades; CGB palettes (if doing CGB).
- [ ] **Render loop** — 59.7275 Hz (70224 T-cycles/frame).

---

## 5. Timers

- [ ] **DIV (0xFF04)** — divider, increments at 16384 Hz.
- [ ] **TIMA (0xFF05)** / **TMA (0xFF06)** / **TAC (0xFF07)** — timer counter, reload, control (clock select + enable).
- [ ] **Timer interrupt** — fire 0x50 on TIMA overflow.

---

## 6. Input (Joypad)

- [ ] **P1 (0xFF00)** — select buttons vs D-pad, read state.
- [ ] **Button mapping** — A/B/Start/Select + Up/Down/Left/Right.
- [ ] **Joypad interrupt (0x60)** — on button press edge.
- [ ] **Input API** — expose a way for the frontend to set/clear buttons.

---

## 7. APU / Sound

- [ ] **APU register map** — NRxx registers (0xFF10–0xFF26), NR52 (0xFF26) master enable.
- [ ] **Channel 1 (square, sweep)** — sweep, length, envelope, frequency.
- [ ] **Channel 2 (square)** — length, envelope, frequency (no sweep).
- [ ] **Channel 3 (wave)** — 32×4-bit wave RAM, length, volume.
- [ ] **Channel 4 (noise)** — LFSR, length, envelope.
- [ ] **Frame sequencer** — 512 Hz length/envelope/sweep steps.
- [ ] **Sample generation** — mix 4 channels, output at audio rate (e.g., 44100 Hz) with resampling.
- [ ] **Length counter & DAC** — enable/disable channels correctly.

---

## 8. Serial / Link Cable (optional)

- [ ] **SC (0xFF02) / SB (0xFF01)** — serial transfer, clock select, serial interrupt (0x58).
- [ ] **Link cable / network play** (optional, low priority).

---

## 9. Save States

- [ ] **Serialization** — capture full CPU state (registers, PC/SP, IME, halt state), memory, PPU state, timer state, APU state, cartridge/MBC state (incl. RTC).
- [ ] **Load/save API** — `save_state(&self) -> Vec<u8>` and `load_state(&mut self, &[u8])` with a versioned format.
- [ ] **Deterministic round-trip** — save/load produces identical behavior (avoid non-deterministic RNG).
- [ ] **UI hooks** — hotkeys for quick-save/quick-load, state slots.

---

## 10. Debugger

- [ ] **Disassembler** — decode opcode → mnemonic + operands (including CB prefix).
- [ ] **Breakpoints** — by PC and/or memory address.
- [ ] **Step / step-over / step-out** — instruction + frame stepping.
- [ ] **Register & memory inspector** — read/write CPU registers, view memory banks, VRAM, OAM.
- [ ] **Call stack** — track pushes/pops or disassemble return addresses.
- [ ] **Trace logging** — per-instruction log (PC, opcode, registers, flags).
- [ ] **Debug UI** — a separate frontend (e.g., egui/TUI) or CLI commands to drive the above.

---

## 11. Frontend / UI / Integration

- [ ] **Window & rendering** — present framebuffer (e.g., winit + pixels, SDL2, egui, or wasm canvas).
- [ ] **Audio output** — stream APU samples (cpal/rodio).
- [ ] **Input handling** — map keyboard/controller to GB joypad.
- [ ] **ROM loading UI** — file picker / drag-drop.
- [ ] **Run/pause/reset** — control emulator lifecycle.
- [ ] **Save management** — battery saves + save states UI.
- [ ] **Boot ROM handling** — run `dmg_boot.bin` (already loaded in `main.rs`) or skip-boot with correct initial register state.

---

## 12. Gameboy Color (CGB) — optional / stretch

- [ ] **CGB flag detection** — from cartridge header.
- [ ] **Double-speed mode** — KEY1 (0xFF4D) and speed switch.
- [ ] **VRAM/RAM banking** — 2× VRAM banks, 8× WRAM banks.
- [ ] **BG/OBJ palettes** — BCPS/BCPD/OCPS/OCPD, color palettes.
- [ ] **CGB boot ROM** — load the CGB variant.
- [ ] **CGB-only features** — HDMA/GDMA, WRAM bank switching.

---

## 13. Accuracy & Testing

- [ ] **Pass existing unit tests** — get all 163 green (this is the near-term goal).
- [ ] **Blargg test ROMs** — `cpu_instrs`, `instr_timing`, `mem_timing`, `halt_bug`, `dmg_sound` etc.
- [ ] **Mooneye / SameSuite** — for timing + edge cases.
- [ ] **dmg-acid2** — verify PPU reference image.
- [ ] **Tetris / Dr. Mario** — smoke-test real games.
- [ ] **Cycle-accurate PPU/APU timing** — match STAT, timer, and audio behavior.
- [ ] **`halt` bug** — handle the hardware HALT bug correctly.
- [ ] **Benchmarking** — ensure the emulator runs at full speed.

---

## Cleanup

- [ ] `add` (`alu.rs:6`) has a dead `alu_op` helper and an unused `a_before` warning (`alu.rs:10`) worth cleaning up.
- [ ] Fix echo RAM base in `memory.rs` (0xE000 region should mirror 0xC000).
