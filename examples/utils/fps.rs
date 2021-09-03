#[derive(Default)]
pub struct FPSHistory(pub Vec<f64>);

impl FPSHistory {

    pub fn push_fps(&mut self, fps: f64, max: usize) {
        if self.0.len() >= max {
            self.0.remove(0);
        }
        self.0.push(fps);
    }
}
