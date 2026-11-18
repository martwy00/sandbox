use crate::camera::Camera;
use crate::element::{Element, TempUnit};
use crate::fps::FpsCounter;
use crate::ui;
use crate::world::World;
use eframe::egui;

pub struct SandboxApp {
    pub(crate) world: World,
    pub(crate) camera: Camera,
    pub(crate) fps: FpsCounter,
    pub(crate) selected: Element,
    pub(crate) brush_radius: isize,
    pub(crate) paused: bool,
    pub(crate) thermal_view: bool,
    pub(crate) bg_color: egui::Color32,
    pub(crate) temp_unit: TempUnit,
    pub(crate) line_start: Option<(usize, usize)>,
    brush_scroll_acc: f32,
}

impl SandboxApp {
    pub fn new() -> Self {
        let mut world = World::new(400, 300);

        for x in 100..300 {
            world.paint_brush(x, 260, 2, Element::Wall);
        }
        for y in 210..260 {
            world.paint_brush(100, y, 2, Element::Wall);
            world.paint_brush(300, y, 2, Element::Wall);
        }

        Self {
            world,
            camera: Camera::new(),
            fps: FpsCounter::new(),
            selected: Element::Sand,
            brush_radius: 5,
            paused: false,
            thermal_view: false,
            bg_color: egui::Color32::from_rgb(15, 15, 20),
            temp_unit: TempUnit::Celsius,
            line_start: None,
            brush_scroll_acc: 0.0,
        }
    }

    fn handle_keyboard_shortcuts(&mut self, ui: &egui::Ui) {
        if ui.ctx().egui_wants_keyboard_input() {
            return;
        }

        ui.input(|i| {
            if i.key_pressed(egui::Key::Space) {
                self.paused = !self.paused;
            }

            if self.paused
                && (i.key_pressed(egui::Key::Period) || i.key_pressed(egui::Key::ArrowRight))
            {
                self.world.step();
            }

            if i.key_pressed(egui::Key::C) && !i.modifiers.ctrl && !i.modifiers.alt {
                self.world.clear();
            }

            if i.key_pressed(egui::Key::T) && !i.modifiers.ctrl && !i.modifiers.alt {
                self.thermal_view = !self.thermal_view;
            }

            if i.key_pressed(egui::Key::F5) {
                let _ = self.world.save_to_file("quicksave.bin");
            }

            if i.key_pressed(egui::Key::F9) {
                let _ = self.world.load_from_file("quicksave.bin");
            }
        });
    }

    pub(crate) fn handle_viewport_input(&mut self, ui: &egui::Ui, response: &egui::Response) {
        let rect = response.rect;

        ui.ctx().options_mut(|o| o.zoom_with_keyboard = false);

        if response.hovered() {
            let ctrl_held = ui.input(|i| i.modifiers.ctrl);
            let zoom_delta = ui.input(|i| i.zoom_delta());
            let smooth_scroll_y = ui.input(|i| i.smooth_scroll_delta.y);

            if zoom_delta != 1.0 || ctrl_held {
                let delta = if zoom_delta != 1.0 {
                    zoom_delta.ln() * 300.0
                } else {
                    smooth_scroll_y
                };

                self.brush_scroll_acc += delta;

                let step_threshold = 45.0;
                while self.brush_scroll_acc >= step_threshold {
                    self.brush_radius = (self.brush_radius + 1).min(20);
                    self.brush_scroll_acc -= step_threshold;
                }
                while self.brush_scroll_acc <= -step_threshold {
                    self.brush_radius = (self.brush_radius - 1).max(1);
                    self.brush_scroll_acc += step_threshold;
                }
            } else {
                self.brush_scroll_acc = 0.0;
                self.camera
                    .zoom_towards(smooth_scroll_y, response.hover_pos(), rect);
            }

            if ui.input(|i| i.key_pressed(egui::Key::I)) && !ui.ctx().egui_wants_keyboard_input() {
                if let Some(pos) = response.hover_pos() {
                    let (gx, gy) =
                        self.camera
                            .screen_to_grid(pos, rect, self.world.width, self.world.height);
                    if let Some(p) = self.world.get_particle(gx, gy) {
                        self.selected = p.kind;
                    }
                }
            }
        }

        if response.dragged_by(egui::PointerButton::Secondary)
            || response.dragged_by(egui::PointerButton::Middle)
        {
            let delta = ui.input(|i| i.pointer.delta());
            self.camera.pan(delta, rect);
        }

        let alt_held = ui.input(|i| i.modifiers.alt);
        let shift_held = ui.input(|i| i.modifiers.shift);
        let primary_pressed = ui.input(|i| i.pointer.primary_pressed());
        let primary_down = ui.input(|i| i.pointer.primary_down());
        let primary_released = ui.input(|i| i.pointer.primary_released());

        if alt_held && primary_down {
            if let Some(pos) = response.interact_pointer_pos() {
                let (gx, gy) =
                    self.camera
                        .screen_to_grid(pos, rect, self.world.width, self.world.height);
                if let Some(p) = self.world.get_particle(gx, gy) {
                    self.selected = p.kind;
                }
            }
            return;
        }

        if primary_pressed && shift_held {
            if let Some(pos) = response.interact_pointer_pos() {
                let grid_pos =
                    self.camera
                        .screen_to_grid(pos, rect, self.world.width, self.world.height);
                self.line_start = Some(grid_pos);
            }
        }

        if let Some((x0, y0)) = self.line_start {
            if primary_released {
                if let Some(pos) = response.interact_pointer_pos().or(response.hover_pos()) {
                    let (x1, y1) =
                        self.camera
                            .screen_to_grid(pos, rect, self.world.width, self.world.height);
                    self.world
                        .paint_line(x0, y0, x1, y1, self.brush_radius, self.selected);
                }
                self.line_start = None;
            } else if !primary_down {
                self.line_start = None;
            }
        } else if primary_down {
            if let Some(pos) = response.interact_pointer_pos() {
                let (gx, gy) =
                    self.camera
                        .screen_to_grid(pos, rect, self.world.width, self.world.height);
                self.world
                    .paint_brush(gx, gy, self.brush_radius, self.selected);
            }
        }
    }
}

impl eframe::App for SandboxApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.fps.tick();
        self.handle_keyboard_shortcuts(ui);
        ui.ctx().request_repaint();

        if !self.paused {
            self.world.step();
        }

        ui::draw(self, ui);
    }
}