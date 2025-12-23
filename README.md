# ❯ gameboy-rs

*A Game Boy emulator written in Rust, focused on correctness, modularity, and an interactive terminal-based debugging experience.*

---

## Overview

**gameboy-rs** is a work-in-progress Nintendo Game Boy emulator implemented in Rust.  
It emulates the original Game Boy hardware, including CPU, MMU, PPU, APU, timers, interrupts, and cartridge memory controllers.  
The project places special emphasis on **debugging and introspection**, offering a rich terminal user interface (TUI) for stepping through execution, inspecting memory, registers, and instruction history.

This project is intended for:
- Learning emulator development
- Exploring Game Boy hardware internals
- Experimenting with Rust for low-level systems programming

---

## Features

| Area | Description |
|---|---|
| ⚙️ Architecture | Fully modular design with clear separation between CPU, MMU, PPU, APU, and peripherals |
| 🧠 CPU | Implements the Game Boy LR35902 instruction set, decoding, execution, and interrupt handling |
| 🖥️ PPU | Emulates Game Boy graphics pipeline and framebuffer rendering |
| 🔊 APU | Partial audio processing unit implementation with channel abstractions |
| 🎮 Input | Joypad emulation with register-level accuracy |
| 🧪 Debugging | Breakpoints, instruction history, disassembly, logging |
| 🖥️ TUI | Interactive terminal UI using `ratatui` and `crossterm` |
| 📦 Cartridges | Supports multiple MBC types (NoMBC, MBC1, MBC2, MBC3, MBC5) |
| 🤖 CI | GitHub Actions workflow for build, test, clippy, and formatting |

---

## Project Structure

```
.
├── src
│   ├── app            # CLI and TUI frontend
│   ├── apu            # Audio Processing Unit
│   ├── cpu            # CPU core, decoder, registers, interrupts
│   ├── debug          # Disassembler, logger, breakpoints
│   ├── mmu            # Memory management and cartridges
│   ├── ppu            # Graphics rendering
│   ├── emulator.rs   # High-level emulator orchestration
│   └── main.rs        # Application entry point
├── roms               # Test ROMs and save files
└── .github/workflows  # CI configuration
```

---

## Getting Started

### Prerequisites

- **Rust** (stable)
- **Cargo**

Install Rust via https://www.rust-lang.org/tools/install

---

### Installation

Clone the repository and build the project:

```sh
git clone https://github.com/michalshy/gameboy-rs
cd gameboy-rs
cargo build
```

---

### Usage

Run the emulator:

```sh
cargo run
```

You can load ROMs via the interactive shell inside the TUI.  
The interface supports stepping, continuous execution, breakpoints, memory inspection, and CPU state visualization.

---

### Testing

Run the full test suite:

```sh
cargo test
```

CI runs:
- `cargo build`
- `cargo test`
- `cargo clippy`
- `cargo fmt --check`

across Linux, macOS, and Windows.

---

## Roadmap

- [x] CPU core and instruction decoding
- [x] MMU and cartridge support
- [x] Terminal-based debugger (TUI)
- [ ] Improve PPU accuracy and timing
- [ ] Complete APU sound output
- [ ] Save-state support
- [ ] Performance optimizations
- [ ] WebAssembly frontend (stretch goal)

---

## Contributing

Contributions are welcome!

1. Fork the repository
2. Create a feature branch
3. Commit your changes with clear messages
4. Open a pull request

Please ensure:
- Code is formatted with `cargo fmt`
- No new clippy warnings are introduced
- Tests pass

---

## License

This project is licensed under the **MIT License**.  
See the [LICENSE](LICENSE) file for details.

---

## Acknowledgments

- Pan Docs – Game Boy hardware documentation
- Other open-source Game Boy emulator projects
- Rust community for excellent tooling and libraries
