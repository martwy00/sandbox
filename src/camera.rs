use eframe::egui;

pub struct Camera {
    pub zoom: f32,
    center: egui::Vec2,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            zoom: 1.0,
            center: egui::vec2(0.5, 0.5),
        }
    }

    pub fn reset(&mut self) {
        self.zoom = 1.0;
        self.center = egui::vec2(0.5, 0.5);
    }

    pub fn visible_uv(&self) -> egui::Rect {
        let half = 0.5 / self.zoom;
        let cx = self.center.x.clamp(half, 1.0 - half);
        let cy = self.center.y.clamp(half, 1.0 - half);

        egui::Rect::from_min_max(
            egui::pos2(cx - half, cy - half),
            egui::pos2(cx + half, cy + half),
        )
    }

    pub fn zoom_towards(&mut self, scroll_y: f32, cursor_pos: Option<egui::Pos2>, rect: egui::Rect) {
        if scroll_y == 0.0 {
            return;
        }

        let uv_before = self.visible_uv();
        let old_zoom = self.zoom;
        let factor = (scroll_y * 0.0018).exp();
        self.zoom = (self.zoom * factor).clamp(1.0, 12.0);

        if self.zoom != old_zoom {
            if let Some(pos) = cursor_pos {
                let norm_x = ((pos.x - rect.left()) / rect.width()).clamp(0.0, 1.0);
                let norm_y = ((pos.y - rect.top()) / rect.height()).clamp(0.0, 1.0);

                let world_u = uv_before.left() + norm_x * uv_before.width();
                let world_v = uv_before.top() + norm_y * uv_before.height();

                let new_half = 0.5 / self.zoom;
                self.center.x = world_u - norm_x * (2.0 * new_half) + new_half;
                self.center.y = world_v - norm_y * (2.0 * new_half) + new_half;
            }
            self.clamp();
        }
    }

    pub fn pan(&mut self, delta: egui::Vec2, rect: egui::Rect) {
        if rect.width() > 0.0 && rect.height() > 0.0 {
            self.center.x -= (delta.x / rect.width()) / self.zoom;
            self.center.y -= (delta.y / rect.height()) / self.zoom;
            self.clamp();
        }
    }

    fn clamp(&mut self) {
        let half = 0.5 / self.zoom;
        self.center.x = self.center.x.clamp(half, 1.0 - half);
        self.center.y = self.center.y.clamp(half, 1.0 - half);
    }

    pub fn screen_to_grid(
        &self,
        pos: egui::Pos2,
        rect: egui::Rect,
        world_width: usize,
        world_height: usize,
    ) -> (usize, usize) {
        let uv = self.visible_uv();
        let norm_x = ((pos.x - rect.left()) / rect.width()).clamp(0.0, 0.9999);
        let norm_y = ((pos.y - rect.top()) / rect.height()).clamp(0.0, 0.9999);

        let world_u = uv.left() + norm_x * uv.width();
        let world_v = uv.top() + norm_y * uv.height();

        let gx = (world_u * world_width as f32) as usize;
        let gy = (world_v * world_height as f32) as usize;
        (gx, gy)
    }
}