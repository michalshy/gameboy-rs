
A compact, modular Game Boy emulator written in Rust. The project focuses on correctness and clarity: a minimal, well-organized codebase for implementing the CPU, MMU, PPU, APU and a small TUI for debugging and development.

Status
------
- Work in progress — core layout and scaffolding in place.
- **CPU**: Decoder, register helpers and opcode table complete; instruction semantics being implemented.
- **MMU**: Basic memory map with VRAM, WRAM, HRAM, OAM; cartridge and MBC detection functional.
- **Cartridge**: ROM loading, MBC type detection (NoMBC, MBC1-5), and RAM sizing implemented.
- **App**: TUI shell with command system; supports ROM loading and emulator reset.
- **PPU & APU**: Basic stubs with register read/write; full implementations pending.

Goals
-----
- Implement a cycle-accurate CPU core and instruction set.
- Add a simple MMU + cartridge/MBC support and integrate with CPU.
- Implement PPU rendering pipeline and optional APU audio.
- Provide a TUI-based developer UI for inspecting registers, memory and disassembly.

Repository layout (high level)
-----------------------------
- `src/cpu/` — decoder, registers, instruction implementations.
- `src/mmu/` — memory map, cartridge and MBC logic.
- `src/ppu/` — PPU registers and renderer (WIP).
- `src/apu/` — audio channel stubs (WIP).
- `src/app/` — small TUI to drive the emulator and display state.
- `src/debug/` — disassembler and instruction logger.
- `roms/` — test ROMs and CPU instruction test cartridges.

Implemented features
--------------------
- **CPU Registers**: All 8-bit and 16-bit registers (A, F, B, C, D, E, H, L, SP, PC) with flag accessors.
- **CPU Decoder**: Complete opcode table (256 main opcodes + 256 CB-prefixed) with cycle counts.
- **MMU**: Memory management with separate regions (ROM, VRAM, WRAM, HRAM, OAM, cartridge RAM).
- **Cartridge System**: Automatic MBC detection; supports NoMBC, MBC1, MBC2, MBC3, MBC5.
- **TUI Commands**: Shell interface with ROM loading (`load <path>`) and emulator reset.
- **Emulator Core**: CPU step-cycle integration with MMU; tick-based synchronization.

Quick start
-----------
Prerequisites: Rust toolchain (stable), `cargo` on PATH.

Build the project:

```
cargo build
```

Run (debug build):

```
cargo run
```

Loading a ROM
Once running, use the shell command:

```
load-rom <path-to-rom>
```

Example: `load-rom roms/tetris.gb`

Contributing
------------
PRs welcome. If you're adding instructions, include unit tests for flags/side effects and a short note in the PR describing any behavior differences from real hardware.

License
-------
Currently unlicensed - tbd.
