mod cpu;
pub mod gpu;
mod input;
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
    pub gpu: Gpu,
    pub keypad: Keypad,
    pub memory: Memory,
    pub running: bool,
    pub should_quit: bool,
    pub timers: Timer,
}

impl Emulator {
    pub fn new() -> Self {
        //let cpu = Cpu::new(memory);
        //let gpu = Gpu::new();
        //let memory = Memory::new(delay_timer, gpu, keypad, rom_buffer, sound_timer);
        //let rom_buffer = Vec::new();
        //let keypad = Keypad::new();
        //let sound_timer = Timer::new();
        //let delay_timer = Timer::new();
        Self {
            cpu: Cpu::new(),
            gpu: Gpu::new(),
            memory: Memory::new(),
            keypad: Keypad::new(),
            timers: Timer::new(99999),
            should_quit: false,
            running: false,
        }
    }

    /// The print_memory function has been moved to the Memory module
    pub fn print_memory(&self) {
        for (i, byte) in self.memory.ram.iter().enumerate() {
            if i % 16 == 0 {
                println!("\n{:04X}: ", i);
            }
            print!("{:02X} ", byte);
        }
        println!();
    }

    pub fn load_font(&mut self) -> Result<bool, bool> {
        self.memory.ram[0..80].copy_from_slice(&FONTS);
        Ok(true)
    }

    /// Puts the rom_buffer into the memory
    pub fn load_rom(&mut self) -> Result<bool, bool> {
        let rom_length: usize = self.memory.rom.len();
        self.memory.ram[512..512 + rom_length].copy_from_slice(&self.memory.rom);
        self.memory.rom.clear();
        Ok(true)
    }

    pub fn run(&mut self) {
        while !self.should_quit {
            self.cpu.tick(&mut self.memory, &self.gpu);
            self.timers.tick();
            self.gpu.draw();
        }
    }
}
