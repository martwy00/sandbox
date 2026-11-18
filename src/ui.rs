use crate::app::SandboxApp;
use crate::element::{Element, TempUnit};
use eframe::egui;

pub fn draw(app: &mut SandboxApp, ui: &mut egui::Ui) {
    draw_toolbar(app, ui);
    draw_viewport(app, ui);
}

fn draw_toolbar(app: &mut SandboxApp, ui: &mut egui::Ui) {
    egui::Panel::top("toolbar").show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            // Sypkie i wybuchowe
            ui.selectable_value(&mut app.selected, Element::Sand, "Sand");
            ui.selectable_value(&mut app.selected, Element::Gunpowder, "Gunpowder");
            ui.selectable_value(&mut app.selected, Element::Salt, "Salt");
            ui.selectable_value(&mut app.selected, Element::Tnt, "TNT");
            ui.selectable_value(&mut app.selected, Element::Fuse, "Fuse");

            // Ciecze
            ui.selectable_value(&mut app.selected, Element::Water, "Water");
            ui.selectable_value(&mut app.selected, Element::SaltWater, "SaltWater");
            ui.selectable_value(&mut app.selected, Element::Oil, "Oil");
            ui.selectable_value(&mut app.selected, Element::Acid, "Acid");
            ui.selectable_value(&mut app.selected, Element::Nitro, "Nitro");
            ui.selectable_value(&mut app.selected, Element::Lava, "Lava");
            ui.selectable_value(&mut app.selected, Element::MoltenMetal, "MoltenMetal");

            // Gazy, ogień i elektryczność
            ui.selectable_value(&mut app.selected, Element::Fire, "Fire");
            ui.selectable_value(&mut app.selected, Element::Steam, "Steam");
            ui.selectable_value(&mut app.selected, Element::Methane, "Methane");
            ui.selectable_value(&mut app.selected, Element::Spark, "Spark");
            ui.selectable_value(&mut app.selected, Element::Copper, "Copper");
            ui.selectable_value(&mut app.selected, Element::Battery, "Battery");

            // Stałe i specjalne
            ui.selectable_value(&mut app.selected, Element::Metal, "Metal");
            ui.selectable_value(&mut app.selected, Element::Wood, "Wood");
            ui.selectable_value(&mut app.selected, Element::Plant, "Plant");
            ui.selectable_value(&mut app.selected, Element::Ice, "Ice");
            ui.selectable_value(&mut app.selected, Element::Concrete, "Concrete");
            ui.selectable_value(&mut app.selected, Element::Glass, "Glass");
            ui.selectable_value(&mut app.selected, Element::Rubber, "Rubber");
            ui.selectable_value(&mut app.selected, Element::Stone, "Stone");
            ui.selectable_value(&mut app.selected, Element::Clay, "Clay");
            ui.selectable_value(&mut app.selected, Element::Wall, "Wall");
            ui.selectable_value(&mut app.selected, Element::Clone, "Clone");
            ui.selectable_value(&mut app.selected, Element::Void, "Void");
            ui.selectable_value(&mut app.selected, Element::Empty, "Eraser");

            ui.separator();
            ui.label("Brush:");
            ui.add(egui::Slider::new(&mut app.brush_radius, 1..=20).text("px"));

            ui.separator();
            ui.label("BG:");
            ui.color_edit_button_srgba(&mut app.bg_color);

            ui.separator();
            ui.label("Unit:");
            ui.selectable_value(&mut app.temp_unit, TempUnit::Celsius, "°C");
            ui.selectable_value(&mut app.temp_unit, TempUnit::Fahrenheit, "°F");
            ui.toggle_value(&mut app.thermal_view, "🌡 Thermal");

            ui.separator();
            if ui
                .button(format!("🔍 {:.1}x (Reset)", app.camera.zoom))
                .clicked()
            {
                app.camera.reset();
            }

            ui.separator();
            let pause_label = if app.paused { "▶ Resume" } else { "⏸ Pause" };
            if ui.button(pause_label).clicked() {
                app.paused = !app.paused;
            }
            if app.paused && ui.button("⏭ Step").clicked() {
                app.world.step();
            }
            if ui.button("🗑 Clear").clicked() {
                app.world.clear();
            }

            ui.separator();
            if ui.button("💾 Save").clicked() {
                let _ = app.world.save_to_file("quicksave.bin");
            }
            if ui.button("📂 Load").clicked() {
                let _ = app.world.load_from_file("quicksave.bin");
            }

            ui.separator();
            ui.label(egui::RichText::new(format!("FPS: {:.0}", app.fps.fps)).monospace());
        });
    });
}

fn draw_viewport(app: &mut SandboxApp, ui: &mut egui::Ui) {
    egui::CentralPanel::default().show(ui, |ui| {
        let texture = ui.ctx().load_texture(
            "sandbox_world",
            app.world.to_color_image(app.bg_color, app.thermal_view),
            egui::TextureOptions::NEAREST,
        );

        let response = ui.add(
            egui::Image::new(&texture)
                .fit_to_exact_size(ui.available_size())
                .uv(app.camera.visible_uv())
                .sense(egui::Sense::click_and_drag()),
        );

        app.handle_viewport_input(ui, &response);

        let scale_x = (response.rect.width() / app.world.width as f32) * app.camera.zoom;
        let screen_radius = (app.brush_radius as f32 + 0.5) * scale_x;

        let bg_luma = app.bg_color.r() as u16 + app.bg_color.g() as u16 + app.bg_color.b() as u16;
        let reticle_color = if bg_luma > 380 {
            egui::Color32::from_rgba_unmultiplied(20, 20, 25, 190)
        } else {
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 180)
        };
        let stroke = egui::Stroke::new(1.5, reticle_color);

        if let Some((sx, sy)) = app.line_start {
            if let Some(end_pos) = response.interact_pointer_pos().or(response.hover_pos()) {
                let uv = app.camera.visible_uv();
                let world_u = (sx as f32 + 0.5) / app.world.width as f32;
                let world_v = (sy as f32 + 0.5) / app.world.height as f32;

                let norm_x = (world_u - uv.left()) / uv.width();
                let norm_y = (world_v - uv.top()) / uv.height();

                let start_pos = egui::pos2(
                    response.rect.left() + norm_x * response.rect.width(),
                    response.rect.top() + norm_y * response.rect.height(),
                );

                ui.painter().line_segment([start_pos, end_pos], stroke);
                ui.painter().circle_stroke(start_pos, screen_radius, stroke);
            }
        }

        if let Some(hover_pos) = response.hover_pos() {
            ui.painter().circle_stroke(hover_pos, screen_radius, stroke);

            let (gx, gy) = app.camera.screen_to_grid(
                hover_pos,
                response.rect,
                app.world.width,
                app.world.height,
            );

            if let Some(particle) = app.world.get_particle(gx, gy) {
                if particle.kind != Element::Empty {
                    let label = if let Some(temp_c) = particle.temperature_c() {
                        format!(
                            "{}: {} ({}, {})",
                            particle.kind.name(),
                            app.temp_unit.format(temp_c),
                            gx,
                            gy
                        )
                    } else if particle.kind == Element::Clone && particle.life > 0 {
                        format!(
                            "Clone [{}] ({}, {})",
                            Element::from_u8(particle.life).name(),
                            gx,
                            gy
                        )
                    } else {
                        format!("{} ({}, {})", particle.kind.name(), gx, gy)
                    };

                    let galley = ui.painter().layout_no_wrap(
                        label,
                        egui::FontId::monospace(13.0),
                        egui::Color32::WHITE,
                    );

                    let desired_pos = hover_pos + egui::vec2(12.0, screen_radius + 8.0);
                    let max_x = (response.rect.right() - galley.size().x - 8.0)
                        .max(response.rect.left() + 4.0);
                    let max_y = (response.rect.bottom() - galley.size().y - 8.0)
                        .max(response.rect.top() + 4.0);
                    let text_pos = egui::pos2(
                        desired_pos.x.clamp(response.rect.left() + 4.0, max_x),
                        desired_pos.y.clamp(response.rect.top() + 4.0, max_y),
                    );

                    let bg_rect = egui::Rect::from_min_size(text_pos, galley.size()).expand(4.0);
                    ui.painter().rect_filled(
                        bg_rect,
                        4u8,
                        egui::Color32::from_rgba_unmultiplied(15, 15, 22, 220),
                    );
                    ui.painter().galley(text_pos, galley, egui::Color32::WHITE);
                }
            }
        }
    });
}