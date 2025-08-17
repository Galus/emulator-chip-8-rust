#[derive(Debug)]
pub struct Timer {
    pub delay_timer: u8,
    pub sound_timer: u8,
}
impl Timer {
    pub(crate) fn new(ticks: u8) -> Self {
        Self {
            delay_timer: ticks,
            sound_timer: ticks * 2,
        }
    }
    pub fn tick(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }
}
