pub(crate) mod cpu;
mod gpu;
mod input;
mod iset;
mod mem;
mod timer;

use cpu::Cpu;
use gpu::Gpu;
use input::Keypad;
use mem::{Memory, FONTS};
use timer::Timer;

#[derive(Debug)]
pub struct Emulator {
    pub cpu: Cpu,
    pub running: bool,
}

impl Emulator {
    pub fn new() -> Self {
        let delay_timer = Timer::new();
        let gpu = Gpu::new();
        let keypad = Keypad::new();
        let rom_buffer = Vec::new();
        let sound_timer = Timer::new();
        let memory = Memory::new(delay_timer, gpu, keypad, rom_buffer, sound_timer);
        let cpu = Cpu::new(memory);
        Self {
            cpu,
            running: false,
        }
    }

    /// The print_memory function has been moved to the Memory module
    pub fn print_memory(&self) {
        for (i, byte) in self.cpu.memory.ram.iter().enumerate() {
            if i % 16 == 0 {
                println!("\n{:04X}: ", i);
            }
            print!("{:02X} ", byte);
        }
        println!();
    }

    pub fn load_font(&mut self) -> Result<bool, bool> {
        self.cpu.memory.ram[0..80].copy_from_slice(&FONTS);
        Ok(true)
    }

    /// Puts the rom_buffer into the memory
    pub fn load_rom(&mut self) -> Result<bool, bool> {
        let rom_length: usize = self.cpu.memory.rom.len();
        self.cpu.memory.ram[512..512 + rom_length].copy_from_slice(&self.cpu.memory.rom);
        self.cpu.memory.rom.clear();
        Ok(true)
    }
}
