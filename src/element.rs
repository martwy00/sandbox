use eframe::egui;

#[derive(Clone, Copy, PartialEq)]
pub enum TempUnit {
    Celsius,
    Fahrenheit,
}

impl TempUnit {
    pub fn format(self, temp_c: f32) -> String {
        match self {
            TempUnit::Celsius => format!("{:.0} °C", temp_c),
            TempUnit::Fahrenheit => format!("{:.0} °F", temp_c * 9.0 / 5.0 + 32.0),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Element {
    Empty,
    Sand,
    Gunpowder,
    Salt,
    Tnt,
    Fuse,
    Water,
    SaltWater,
    Oil,
    Acid,
    Nitro,
    Lava,
    Wood,
    Plant,
    Ice,
    Fire,
    Smoke,
    Steam,
    Metal,
    Wall,
    Void,
    Concrete,
    Glass,
    Rubber,
    Stone,
    Clay,
}

impl Element {
    pub fn name(self) -> &'static str {
        match self {
            Element::Empty => "Empty",
            Element::Sand => "Sand",
            Element::Gunpowder => "Gunpowder",
            Element::Salt => "Salt",
            Element::Tnt => "TNT",
            Element::Fuse => "Fuse",
            Element::Water => "Water",
            Element::SaltWater => "SaltWater",
            Element::Oil => "Oil",
            Element::Acid => "Acid",
            Element::Nitro => "Nitro",
            Element::Lava => "Lava",
            Element::Wood => "Wood",
            Element::Plant => "Plant",
            Element::Ice => "Ice",
            Element::Fire => "Fire",
            Element::Smoke => "Smoke",
            Element::Steam => "Steam",
            Element::Metal => "Metal",
            Element::Wall => "Wall",
            Element::Void => "Void",
            Element::Concrete => "Concrete",
            Element::Glass => "Glass",
            Element::Rubber => "Rubber",
            Element::Stone => "Stone",
            Element::Clay => "Clay",
        }
    }

    pub fn density(self) -> Option<u8> {
        match self {
            Element::Empty => Some(0),
            Element::Oil => Some(1),
            Element::Water | Element::Acid => Some(2),
            Element::SaltWater | Element::Nitro => Some(3),
            Element::Sand | Element::Gunpowder | Element::Salt => Some(4),
            _ => None,
        }
    }

    pub fn is_loose(self) -> bool {
        matches!(
            self,
            Element::Sand
                | Element::Gunpowder
                | Element::Salt
                | Element::Water
                | Element::SaltWater
                | Element::Oil
                | Element::Acid
                | Element::Nitro
                | Element::Lava
                | Element::Fire
                | Element::Smoke
                | Element::Steam
        )
    }
}

#[derive(Clone, Copy)]
pub struct Particle {
    pub kind: Element,
    pub life: u8,
    pub shade: u8,
}

impl Particle {
    pub fn empty() -> Self {
        Self {
            kind: Element::Empty,
            life: 0,
            shade: 0,
        }
    }

    pub fn temperature_c(&self) -> Option<f32> {
        match self.kind {
            Element::Metal if self.life > 0 => {
                Some(20.0 + (self.life as f32 / 255.0) * 1480.0)
            }
            Element::Lava => Some(1200.0),
            Element::Fire => Some(500.0 + (self.life as f32 * 3.0)),
            Element::Steam => Some(105.0),
            _ => None,
        }
    }

    pub fn color(&self, bg: egui::Color32) -> egui::Color32 {
        let s = self.shade;
        match self.kind {
            Element::Empty => bg,
            Element::Sand => egui::Color32::from_rgb(
                215u8.saturating_add(s),
                180u8.saturating_add(s),
                90u8.saturating_add(s / 2),
            ),
            Element::Gunpowder => egui::Color32::from_rgb(
                65u8.saturating_add(s),
                65u8.saturating_add(s),
                72u8.saturating_add(s),
            ),
            Element::Salt => egui::Color32::from_rgb(
                225u8.saturating_add(s),
                225u8.saturating_add(s),
                235u8.saturating_add(s),
            ),
            Element::Tnt => egui::Color32::from_rgb(200u8.saturating_add(s), 35, 35),
            Element::Fuse => egui::Color32::from_rgb(
                145u8.saturating_add(s),
                115u8.saturating_add(s / 2),
                85,
            ),
            Element::Water => egui::Color32::from_rgb(
                40u8.saturating_add(s / 2),
                120u8.saturating_add(s),
                220u8.saturating_add(s),
            ),
            Element::SaltWater => egui::Color32::from_rgb(
                75u8.saturating_add(s / 2),
                155u8.saturating_add(s),
                225u8.saturating_add(s),
            ),
            Element::Oil => egui::Color32::from_rgb(
                75u8.saturating_add(s / 2),
                55u8.saturating_add(s / 2),
                40u8.saturating_add(s / 3),
            ),
            Element::Acid => egui::Color32::from_rgb(
                90u8.saturating_add(s),
                235u8.saturating_add(s / 2),
                45,
            ),
            Element::Nitro => egui::Color32::from_rgb(
                50,
                200u8.saturating_add(s),
                115u8.saturating_add(s),
            ),
            Element::Lava => {
                egui::Color32::from_rgb(245, 75u8.saturating_add(s.saturating_mul(2)), 15)
            }
            Element::Wood => egui::Color32::from_rgb(
                110u8.saturating_add(s / 2),
                70u8.saturating_add(s / 2),
                35,
            ),
            Element::Plant => egui::Color32::from_rgb(45, 155u8.saturating_add(s), 55),
            Element::Ice => {
                egui::Color32::from_rgb(175u8.saturating_add(s), 225u8.saturating_add(s), 250)
            }
            Element::Fire => {
                if self.life > 45 {
                    egui::Color32::from_rgb(255, 210u8.saturating_add(s), 60)
                } else {
                    egui::Color32::from_rgb(230u8.saturating_add(s / 2), 80u8.saturating_add(s), 20)
                }
            }
            Element::Smoke => egui::Color32::from_rgb(
                80u8.saturating_add(s),
                80u8.saturating_add(s),
                90u8.saturating_add(s),
            ),
            Element::Steam => egui::Color32::from_rgb(
                185u8.saturating_add(s),
                205u8.saturating_add(s),
                225u8.saturating_add(s),
            ),
            Element::Metal => {
                let t = self.life;
                if t == 0 {
                    egui::Color32::from_rgb(
                        135u8.saturating_add(s / 2),
                        142u8.saturating_add(s / 2),
                        152u8.saturating_add(s / 2),
                    )
                } else if t < 100 {
                    let ratio = t as f32 / 100.0;
                    let r = (135.0 + ratio * 105.0) as u8;
                    let g = (142.0 * (1.0 - ratio * 0.8)) as u8;
                    let b = (152.0 * (1.0 - ratio * 0.85)) as u8;
                    egui::Color32::from_rgb(r.saturating_add(s / 3), g, b)
                } else if t < 190 {
                    let ratio = (t - 100) as f32 / 90.0;
                    let g = (28.0 + ratio * 165.0) as u8;
                    let b = (22.0 + ratio * 20.0) as u8;
                    egui::Color32::from_rgb(255, g, b)
                } else {
                    let ratio = (t - 190) as f32 / 65.0;
                    let g = (193.0 + ratio * 62.0) as u8;
                    let b = (42.0 + ratio * 195.0) as u8;
                    egui::Color32::from_rgb(255, g, b)
                }
            }
            Element::Wall => egui::Color32::from_rgb(115, 115, 125),
            Element::Void => egui::Color32::from_rgb(35, 15, 50),
            Element::Concrete => egui::Color32::from_rgb(
                175u8.saturating_add(s / 2),
                173u8.saturating_add(s / 2),
                160u8.saturating_add(s / 2),
            ),
            Element::Glass => egui::Color32::from_rgb(
                195u8.saturating_add(s / 2),
                225u8.saturating_add(s / 2),
                238u8.saturating_add(s / 2),
            ),
            Element::Rubber => egui::Color32::from_rgb(
                52u8.saturating_add(s / 2),
                52u8.saturating_add(s / 2),
                58u8.saturating_add(s / 2),
            ),
            Element::Stone => egui::Color32::from_rgb(
                105u8.saturating_add(s),
                108u8.saturating_add(s),
                112u8.saturating_add(s),
            ),
            Element::Clay => egui::Color32::from_rgb(
                155u8.saturating_add(s / 2),
                110u8.saturating_add(s / 2),
                80u8.saturating_add(s / 2),
            ),
        }
    }
}