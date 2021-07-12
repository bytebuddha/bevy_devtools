pub struct History {
    pub fps: Vec<f32>,
}

impl History {
    pub fn push_fps(&mut self, fps: f64) {
        if self.fps.len() == crate::consts::HISTORY_LENGTH {
            self.fps.remove(0);
        }
        self.fps.push(fps as f32);
    }
}

impl Default for History {
    fn default() -> History {
        History {
            fps: Vec::with_capacity(crate::consts::HISTORY_LENGTH),
        }
    }
}
