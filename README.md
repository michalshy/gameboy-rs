# gameboy-rs

A Game Boy emulator written in Rust, focused on clear architecture and incremental correctness rather than completeness or performance.
The project serves as a learning-oriented emulator core with a simple command-line interface and debugging tools.

## Status

This project is a work in progress.

Core components such as the CPU, memory system, cartridge handling, and debugging infrastructure are partially implemented. Graphics and audio subsystems are currently incomplete or stubbed.

## Goals

The main goals of this project are:

- Provide a readable and modular Game Boy emulator architecture
- Incrementally implement accurate hardware behavior
- Offer debugging and inspection tools useful for emulator development
- Serve as a reference or learning project for Rust and emulator design

## Features

Currently implemented or partially implemented features include:

- CPU (LR35902)
  - Instruction decoding
  - Register file
  - Opcode execution logic (in progress)
- Memory Management Unit (MMU)
  - Memory mapping for ROM, RAM, VRAM, OAM, HRAM
  - Cartridge loading
  - Basic MBC support scaffolding
- Cartridge handling
  - ROM parsing
  - Header inspection
- Debugging utilities
  - Disassembler
  - Instruction logging
- Command-line interface
  - Interactive shell
  - ROM loading and reset commands

Graphics (PPU) and audio (APU) are present as modules but are not yet fully implemented.

## Building

### Requirements

- Rust (stable)

Install Rust using rustup if it is not already installed:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build

Clone the repository and build the project:

```sh
git clone https://github.com/michalshy/gameboy-rs.git
cd gameboy-rs
cargo build
```

## Running

Run the emulator using:

```sh
cargo run
```

This starts an interactive command-line shell.

## Minimal Usage

Load a Game Boy ROM from the shell:

```text
load-rom path/to/rom.gb
```

Reset the emulator state:

```text
reset
```

Quit the shell:

```text
exit
```

The shell is intended for debugging and development rather than gameplay.

## Test ROMs

The repository contains test ROMs (such as CPU instruction tests) used to validate correctness during development. These are useful for verifying opcode behavior and flag handling.

## Design Philosophy

- Prefer explicit and readable code over clever optimizations
- Keep emulator subsystems loosely coupled
- Make hardware behavior easy to inspect and debug
- Accept partial implementations as long as structure is correct

Accuracy and completeness are secondary to clarity at the current stage of development.

## Future Work

Planned or potential improvements include:

- Full CPU instruction coverage with cycle accuracy
- Complete PPU rendering pipeline
- Audio (APU) implementation
- More comprehensive cartridge and MBC support
- Improved debugging tools
- Optional graphical or web-based frontends

## Contributing

Contributions are welcome.

If you plan to contribute:

- Keep changes focused and well-scoped
- Add tests for CPU instructions and edge cases where applicable
- Document any known inaccuracies or hardware deviations

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

## About

`gameboy-rs` is a personal emulator project written in Rust, intended for learning, experimentation, and exploration of emulator internals and low-level system design.