
A compact, modular Game Boy emulator written in Rust. The project focuses on correctness and clarity: a minimal, well-organized codebase for implementing the CPU, MMU, PPU, APU and a small TUI for debugging and development.

Status
------
- Work in progress — core layout and scaffolding in place.
- CPU decoder, register helpers and opcode table exist; instruction semantics are being implemented.
- MMU, PPU and APU have basic stubs to allow incremental development.

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
- `roms/` — test ROMs and CPU instruction test cartridges.

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

Contributing
------------
PRs welcome. If you're adding instructions, include unit tests for flags/side effects and a short note in the PR describing any behavior differences from real hardware.

License
-------
Currently unlicensed - tbd.
