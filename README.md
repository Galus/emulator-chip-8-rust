# CHIP-8 Emulator

This is a CHIP-8 emulator implemented in Rust. CHIP-8 is an interpreted programming language developed in the 1970s, primarily used on 8-bit microcomputers and for creating simple video games.

- 4/20/25 Add tui-logger and ratatui tracing. Enable logging to ease development.
- 8/16/25 Moved to its own project.

## Project Structure

The project is organized into the following modules:

- chip8
    - emu
        - cpu
            - gpu
            - memory

[joamag's boytacean gameboy emulator](https://github.com/joamag/boytacean) 
inspired my project layout to funnel all the things into the cpu. :)

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
