# CHIP-8 Emulator

This is a CHIP-8 emulator implemented in Rust.
I like to call it "Baby's first emulator".
CHIP-8 is an interpreted programming language developed in the 1970s, primarily used on 8-bit microcomputers and for creating simple video games.

## Features

- Terminal User Interface. I love TUIs! What other Emulator to implement in a TUI other than Chip-8 because of its 32x64 display?
- [Ratatui TUI](https://ratatui.rs/) with [tui-logger](https://github.com/gin66/tui-logger/) smart widget
- `GPU` Widget for rendering the `display`
- `Emu[later]` Widget for rendering the `App state`
    - Display Registers, Memory Layout
- Load Chip-8 Roms via CLI arg.
- Multi-Threaded - Input Thread, Background Threads, Main+Rendering Thread
- Logs to file `./chip8.log`

## Building and Running

To build and run the emulator, make sure you have Rust installed on your system. Then, follow these steps:

1. Clone the repository:
   ```
   git clone https://github.com/galus/rust-edu.git
   cd rust-edu/chip8
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Run the emulator:
   ```
   cargo run --release -- roms/test.ch8
   ```

Replace `roms/test.ch8` with the path to a CHIP-8 ROM file you want to run.

## Usage

Once the application is running press `?` to open the help:

```
  General Controls
  - ?: Toggle the help screen.
  - q or Ctrl-C: Quit the application.

  Chip-8 Keypad Mapping
  The emulator maps your keyboard to a standard Chip-8 keypad.
  - 1: 1
  - 2: 2
  - 3: 3
  - 4: C
  - q: 4
  - w: 5
  - e: 6
  - r: D
  - a: 7
  - s: 8
  - d: 9
  - f: E
  - z: A
  - x: 0
  - c: B
  - v: F

  Log Panel Controls
  These controls are active when the log panel is focused.
  - l: Toggle Log Panel on/off
  - Arrow Keys (↑, ↓, ←, →): Navigate through log messages.
  - Page Up / Page Down: Jump to the previous or next page of logs.
  - + / -: Increase or decrease the log verbosity level.
  - h: Hide the log target selector.
  - f: Focus on the log target selector.
  - Tab: Switch between the different log states.
  - Escape: Exit the log focus mode.
```

## Changelog

- 4/20/25 Add tui-logger and ratatui tracing. Enable logging to ease development.
- 8/16/25 Moved to its own project.
- 9/01/25 Got threading and TUI logger implemented. Organized overall arch and iset.
- 9/06/25 Added [TEST_PLAN.md](/TEST_PLAN.md)

## Instructions Progress
For more detailed progress see [TEST_PLAN.md](/TEST_PLAN.md)

- [X] 00E0 - CLS
- [X] 00EE - RET
- [X] 0nnn - SYS addr
- [X] 1nnn - JP addr
- [X] 2nnn - CALL addr
- [X] 3xkk - SE Vx, byte
- [X] 4xkk - SNE Vx, byte
- [X] 5xy0 - SE Vx, Vy
- [X] 6xkk - LD Vx, byte
- [X] 7xkk - ADD Vx, byte
- [X] 8xy0 - LD Vx, Vy
- [X] 8xy1 - OR Vx, Vy
- [X] 8xy2 - AND Vx, Vy
- [X] 8xy3 - XOR Vx, Vy
- [X] 8xy4 - ADD Vx, Vy
- [X] 8xy5 - SUB Vx, Vy
- [X] 8xy6 - SHR Vx {, Vy}
- [X] 8xy7 - SUBN Vx, Vy
- [X] 8xyE - SHL Vx {, Vy}
- [X] 9xy0 - SNE Vx, Vy
- [X] Annn - LD I, addr
- [X] Bnnn - JP V0, addr
- [X] Cxkk - RND Vx, byte
- [X] Dxyn - DRW Vx, Vy, nibble
- [ ] Ex9E - SKP Vx
- [ ] ExA1 - SKNP Vx
- [ ] Fx07 - LD Vx, DT
- [ ] Fx0A - LD Vx, K
- [ ] Fx15 - LD DT, Vx
- [ ] Fx18 - LD ST, Vx
- [ ] Fx1E - ADD I, Vx
- [ ] Fx29 - LD F, Vx
- [ ] Fx33 - LD B, Vx
- [ ] Fx55 - LD [I], Vx
- [ ] Fx65 - LD Vx, [I]

## Project Structure

The project is organized into the following modules:

```
src
├── emojis.rs
├── emu
│   ├── cpu.rs
│   ├── gpu.rs
│   ├── input.rs
│   ├── iset.rs
│   ├── mem.rs
│   ├── mod.rs
│   └── timer.rs
└── main.rs
```

### WARNING Old Project Structure
I am leaving this in the readme to help future first-time emulator
developers from going down this deep rabbit hole of coupling/nesting.

- chip8
    - emu
        - cpu
            - gpu
            - memory

[joamag's boytacean gameboy emulator](https://github.com/joamag/boytacean) 
inspired my project layout to funnel all the things into the cpu. :)

**Update 8/23/25**

I am so pissed I hurdur 'funnel'd all these things into CPU.
I refactored it all to be top-level at 'emu' and passed down to where its needed.
I wonder why I even decided to go down that dark path.

## Architecture Design Philosophy
```
Module-Level Interaction
Each component module (cpu.rs, gpu.rs, mem.rs, etc.) should be self-contained and expose public methods for the Emu struct to call.

    cpu.rs: Contains the CPU state (program counter, registers, stack) and logic for fetching and executing opcodes. It will take a mutable reference to Mem and Gpu to interact with them.

    mem.rs: Manages the memory array. It will have methods like read_byte(addr) and write_byte(addr, value).

    gpu.rs: Handles the pixel grid and drawing logic. It should have a method like draw_sprite(x, y, data) that takes a mutable reference to self to update its state. It will then be responsible for converting this state into a ratatui widget.

    input.rs: Manages the state of the keyboard keys.

    timer.rs: Manages the delay and sound timers.

```

## Technical Requirements

- Accurate emulation of CHIP-8 instructions
- Display output
- Keyboard input
- Sound support
- Configurable clock speed

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
