use std::time::Instant;

pub struct FpsCounter {
    last_update: Instant,
    frame_count: u32,
    pub fps: f32,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            last_update: Instant::now(),
            frame_count: 0,
            fps: 0.0,
        }
    }

    pub fn tick(&mut self) {
        self.frame_count += 1;
        let elapsed = self.last_update.elapsed().as_secs_f32();

        // Aktualizujemy wartość co 0.5s, żeby licznik był czytelny i nie migotał
        if elapsed >= 0.5 {
            self.fps = self.frame_count as f32 / elapsed;
            self.frame_count = 0;
            self.last_update = Instant::now();
        }
    }
}