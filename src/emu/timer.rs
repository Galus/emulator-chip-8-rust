#[derive(Debug)]
pub struct Timer {
    pub timer1: u8,
    pub timer2: u8,
}
impl Timer {
    pub(crate) fn new(ticks: u8) -> Self {
        Self {
            timer1: ticks,
            timer2: ticks * 2,
        }
    }
    pub fn tick(&mut self) {
        if self.timer1 > 0 {
            self.timer1 -= 1;
        }
        if self.timer2 > 0 {
            self.timer2 -= 1;
        }
    }
}
