# CHIP-8 Emulator

This is a CHIP-8 emulator implemented in Rust.
CHIP-8 is an interpreted programming language developed in the 1970s, primarily used on 8-bit microcomputers and for creating simple video games.


## Features

- Terminal User Interface. I love TUIs! What other Emulator to implement in a TUI other than Chip-8 because of its 32x64 display?
- [Ratatui TUI](https://ratatui.rs/) with [tui-logger](https://github.com/gin66/tui-logger/) smart widget
- `GPU` Widget for rendering the `display`
- `Emu[later]` Widget for rendering the `App state`
    - Display Registers, Memory Layout
- Load Chip-8 Roms via CLI arg.
- Multi-Threaded - Input Thread, Background Threads, Main+Rendering Thread


## Changelog

- 4/ww20/25 Add tui-logger and ratatui tracing. Enable logging to ease development.
- 8/16/25 Moved to its own project.
- 9/01/25 Got threading and TUI logger implemented. Organized overall arch and iset.
- 9/06/25 Added [TEST_PLAN.md](/TEST_PLAN.md)

## Project Structure

```
Module-Level Interaction
Each component module (cpu.rs, gpu.rs, mem.rs, etc.) should be self-contained and expose public methods for the Emu struct to call.

    cpu.rs: Contains the CPU state (program counter, registers, stack) and logic for fetching and executing opcodes. It will take a mutable reference to Mem and Gpu to interact with them.

    mem.rs: Manages the memory array. It will have methods like read_byte(addr) and write_byte(addr, value).

    gpu.rs: Handles the pixel grid and drawing logic. It should have a method like draw_sprite(x, y, data) that takes a mutable reference to self to update its state. It will then be responsible for converting this state into a ratatui widget.

    input.rs: Manages the state of the keyboard keys.

    timer.rs: Manages the delay and sound timers.

By passing references between these modules when needed (e.g., cpu.execute_opcode(&mut self.memory, &mut self.gpu)), you maintain a clean, central control flow within the Emu struct. This approach is highly maintainable because you can easily trace the flow of data and logic, and each component can be tested independently.
```

The project is organized into the following modules:

- chip8
    - emu
        - cpu
            - gpu
            - memory

[joamag's boytacean gameboy emulator](https://github.com/joamag/boytacean) 
inspired my project layout to funnel all the things into the cpu. :)

Update 8/2ww31/25: I am so pissed I hurdur 'funnel'd all these things into CPU.
I refactored it all to be top-level at 'emu' and passed down to where its needed.
I wonder why I even decided to go down that dark path.

## Features

- Accurate emulation of CHIP-8 instructions
- Display output
- Keyboard input
- Sound support
- Configurable clock speed

### Instructions Progress

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
   cargo run --release -- path/to/rom.ch8
   ```

Replace `path/to/rom.ch8` with the path to a CHIP-8 ROM file you want to run.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
