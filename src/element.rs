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

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum Element {
    Empty = 0,
    Sand = 1,
    Gunpowder = 2,
    Salt = 3,
    Tnt = 4,
    Fuse = 5,
    Water = 6,
    SaltWater = 7,
    Oil = 8,
    Acid = 9,
    Nitro = 10,
    Lava = 11,
    MoltenMetal = 12,
    Wood = 13,
    Plant = 14,
    Ice = 15,
    Fire = 16,
    Smoke = 17,
    Steam = 18,
    Methane = 19,
    Spark = 20,
    Metal = 21,
    Copper = 22,
    Battery = 23,
    Wall = 24,
    Void = 25,
    Clone = 26,
    Concrete = 27,
    Glass = 28,
    Rubber = 29,
    Stone = 30,
    Clay = 31,
}

impl Element {
    pub fn from_u8(v: u8) -> Self {
        match v {
            1 => Element::Sand,
            2 => Element::Gunpowder,
            3 => Element::Salt,
            4 => Element::Tnt,
            5 => Element::Fuse,
            6 => Element::Water,
            7 => Element::SaltWater,
            8 => Element::Oil,
            9 => Element::Acid,
            10 => Element::Nitro,
            11 => Element::Lava,
            12 => Element::MoltenMetal,
            13 => Element::Wood,
            14 => Element::Plant,
            15 => Element::Ice,
            16 => Element::Fire,
            17 => Element::Smoke,
            18 => Element::Steam,
            19 => Element::Methane,
            20 => Element::Spark,
            21 => Element::Metal,
            22 => Element::Copper,
            23 => Element::Battery,
            24 => Element::Wall,
            25 => Element::Void,
            26 => Element::Clone,
            27 => Element::Concrete,
            28 => Element::Glass,
            29 => Element::Rubber,
            30 => Element::Stone,
            31 => Element::Clay,
            _ => Element::Empty,
        }
    }

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
            Element::MoltenMetal => "MoltenMetal",
            Element::Wood => "Wood",
            Element::Plant => "Plant",
            Element::Ice => "Ice",
            Element::Fire => "Fire",
            Element::Smoke => "Smoke",
            Element::Steam => "Steam",
            Element::Methane => "Methane",
            Element::Spark => "Spark",
            Element::Metal => "Metal",
            Element::Copper => "Copper",
            Element::Battery => "Battery",
            Element::Wall => "Wall",
            Element::Void => "Void",
            Element::Clone => "Clone",
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
            Element::MoltenMetal => Some(5),
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
                | Element::MoltenMetal
                | Element::Fire
                | Element::Smoke
                | Element::Steam
                | Element::Methane
                | Element::Spark
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
            Element::Ice => Some(-10.0),
            Element::Metal if self.life > 0 => {
                Some(20.0 + (self.life as f32 / 255.0) * 1480.0)
            }
            Element::MoltenMetal => {
                Some(20.0 + (self.life as f32 / 255.0) * 1480.0)
            }
            Element::Lava => Some(1200.0),
            Element::Fire => Some(500.0 + (self.life as f32 * 3.0)),
            Element::Spark => Some(900.0),
            Element::Steam => Some(105.0),
            _ => None,
        }
    }

    pub fn thermal_color(&self, bg: egui::Color32) -> egui::Color32 {
        if self.kind == Element::Empty {
            return bg;
        }
        let temp = self.temperature_c().unwrap_or(20.0);
        if temp < 0.0 {
            // Zimno: głęboki błękit
            egui::Color32::from_rgb(35, 110, 245)
        } else if temp <= 25.0 {
            // Temperatura pokojowa: ciemny fiolet
            egui::Color32::from_rgb(35, 18, 70)
        } else if temp < 450.0 {
            // Ciepły -> Czerwony
            let t = ((temp - 25.0) / 425.0).clamp(0.0, 1.0);
            egui::Color32::from_rgb(
                (35.0 + t * 215.0) as u8,
                (18.0 + t * 15.0) as u8,
                (70.0 * (1.0 - t)) as u8,
            )
        } else if temp < 1100.0 {
            // Czerwony -> Pomarańczowy/Żółty
            let t = ((temp - 450.0) / 650.0).clamp(0.0, 1.0);
            egui::Color32::from_rgb(255, (33.0 + t * 195.0) as u8, (t * 40.0) as u8)
        } else {
            // Ekstremalny żar -> Biały
            let t = ((temp - 1100.0) / 400.0).clamp(0.0, 1.0);
            egui::Color32::from_rgb(
                255,
                (228.0 + t * 27.0) as u8,
                (40.0 + t * 215.0) as u8,
            )
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
            Element::MoltenMetal => {
                egui::Color32::from_rgb(255, 225u8.saturating_add(s), 140u8.saturating_add(s * 2))
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
            Element::Methane => egui::Color32::from_rgb(
                120u8.saturating_add(s),
                185u8.saturating_add(s),
                135u8.saturating_add(s),
            ),
            Element::Spark => egui::Color32::from_rgb(120, 235, 255),
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
            Element::Copper => {
                if self.life > 12 {
                    // Rozbłysk prądu w przewodzie
                    egui::Color32::from_rgb(130, 240, 255)
                } else {
                    egui::Color32::from_rgb(
                        185u8.saturating_add(s / 2),
                        105u8.saturating_add(s / 2),
                        65u8.saturating_add(s / 3),
                    )
                }
            }
            Element::Battery => egui::Color32::from_rgb(235, 185, 30),
            Element::Wall => egui::Color32::from_rgb(115, 115, 125),
            Element::Void => egui::Color32::from_rgb(35, 15, 50),
            Element::Clone => {
                if self.life == 0 {
                    egui::Color32::from_rgb(215, 175, 55)
                } else {
                    egui::Color32::from_rgb(245, 205, 85)
                }
            }
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