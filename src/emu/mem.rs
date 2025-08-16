use crate::emu::{Gpu, Keypad, Timer};

/* Chip8 Memory layout
0x000-0x04F - Chip 8 interpreter (contains font set in emu)       0 -   79
0x050-0x0A0 - Used for the built in 4x5 pixel font set (0-F)    080 -  160
0x200-0xFFF - Program ROM and work RAM                          512 - 4096

0x200-0xE8F
"final 352 bytes of memory are reserved for “variables and display refresh"
https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference#registers
*/

pub const RAM_SIZE: usize = 4096;
pub const ROM_MAX_SIZE: usize = RAM_SIZE - 512;

#[derive(Debug)]
pub struct Memory {
    pub delay_timer: Timer,
    pub gpu: Gpu,
    pub pad: Keypad,
    pub ram: [u8; RAM_SIZE],
    pub rom: Vec<u8>,
    pub sound_timer: Timer,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            delay_timer: Timer::new(),
            gpu: Gpu::new(),
            pad: Keypad::new(),
            ram: [0; RAM_SIZE],
            rom: vec![0; ROM_MAX_SIZE],
            sound_timer: Timer::new(),
        }
    }
}

impl Memory {
    pub fn new(dt: Timer, gpu: Gpu, pad: Keypad, rom: Vec<u8>, st: Timer) -> Self {
        Self {
            delay_timer: dt,
            ram: [0; 4096],
            gpu,
            pad,
            rom,
            sound_timer: st,
        }
    }
    pub fn print_memory(&self) {
        for (i, byte) in self.ram.iter().enumerate() {
            if i % 16 == 0 {
                println!("\n{:04X}: ", i);
            }
            print!("{:02X} ", byte);
        }
        println!();
    }
}

/// # Chip8 FONT encoding
/// 16 chars are encoded from 00 - 4F
/// ## Examples
/// 0 = 0xF0 0x90 0x90 0x90 0xF0
/// 0xF = 1 1 1 1 =      1 1 1 1
/// 0x9 = 1 0 0 1 =      1     1
/// 0x9 = 1 0 0 1 =      1     1
/// 0x9 = 1 0 0 1 =      1     1
/// 0xF = 1 1 1 1 =      1 1 1 1
///
/// 1 = 0x20 0x60 0x20 0x20 0x70
/// 0x2 = 0 0 1 0 =          1
/// 0x6 = 0 1 1 0 =         11
/// 0x2 = 0 0 1 0 =          1
/// 0x2 = 0 0 1 0 =          1
/// 0x7 = 0 1 1 1 =         111
///
/// 2 = 0xF0 0x10 0xF0 0x80 0xF0
/// 0xF = 1 1 1 1 =       1 1 1 1
/// 0x1 = 0 0 0 1 =             1
/// 0xF = 1 1 1 1 =       1 1 1 1
/// 0x8 = 1 0 0 0 =       1
/// 0xF = 1 1 1 1 =       1 1 1 1
///
/// ... and so on.
pub const FONTS: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];
