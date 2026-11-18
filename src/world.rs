use crate::element::{Element, Particle};
use eframe::egui;

pub struct World {
    pub width: usize,
    pub height: usize,
    cells: Vec<Particle>,
    updated: Vec<bool>,
    rng_state: u32,
    frame_count: usize,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![Particle::empty(); width * height],
            updated: vec![false; width * height],
            rng_state: 0x1234_5678,
            frame_count: 0,
        }
    }

    pub fn clear(&mut self) {
        self.cells.fill(Particle::empty());
    }

    pub fn get_particle(&self, x: usize, y: usize) -> Option<Particle> {
        if x < self.width && y < self.height {
            Some(self.cells[self.idx(x, y)])
        } else {
            None
        }
    }

    fn rand_u32(&mut self) -> u32 {
        self.rng_state = self
            .rng_state
            .wrapping_mul(1664525)
            .wrapping_add(1013904223);
        self.rng_state
    }

    fn rand_u8(&mut self) -> u8 {
        (self.rand_u32() >> 24) as u8
    }

    fn rand_bool(&mut self) -> bool {
        (self.rand_u32() & 1) == 0
    }

    fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let i1 = self.idx(x1, y1);
        let i2 = self.idx(x2, y2);
        self.cells.swap(i1, i2);
        self.updated[i1] = true;
        self.updated[i2] = true;
    }

    fn is_empty(&self, x: isize, y: isize) -> bool {
        self.in_bounds(x, y) && self.cells[self.idx(x as usize, y as usize)].kind == Element::Empty
    }

    fn can_displace(&self, mover: Element, x: isize, y: isize) -> bool {
        if !self.in_bounds(x, y) {
            return false;
        }
        let target = self.cells[self.idx(x as usize, y as usize)].kind;
        match (mover.density(), target.density()) {
            (Some(m), Some(t)) => m > t,
            _ => false,
        }
    }

    pub fn paint_brush(&mut self, cx: usize, cy: usize, radius: isize, kind: Element) {
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                if dx * dx + dy * dy <= radius * radius {
                    let nx = cx as isize + dx;
                    let ny = cy as isize + dy;
                    if self.in_bounds(nx, ny) {
                        let i = self.idx(nx as usize, ny as usize);

                        if kind == Element::Empty {
                            self.cells[i] = Particle::empty();
                            continue;
                        }

                        if kind.is_loose() && self.rand_u8() > 160 {
                            continue;
                        }

                        if self.cells[i].kind == Element::Empty {
                            let life = match kind {
                                Element::Fire => 40 + (self.rand_u8() % 50),
                                Element::Smoke => 80 + (self.rand_u8() % 90),
                                Element::Steam => 140 + (self.rand_u8() % 100),
                                Element::Acid => 4,
                                _ => 0,
                            };
                            self.cells[i] = Particle {
                                kind,
                                life,
                                shade: self.rand_u8() % 30,
                            };
                        }
                    }
                }
            }
        }
    }

    pub fn paint_line(
        &mut self,
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
        radius: isize,
        kind: Element,
    ) {
        let mut x = x0 as isize;
        let mut y = y0 as isize;
        let end_x = x1 as isize;
        let end_y = y1 as isize;

        let dx = (end_x - x).abs();
        let sx = if x < end_x { 1 } else { -1 };
        let dy = -(end_y - y).abs();
        let sy = if y < end_y { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            self.paint_brush(x as usize, y as usize, radius, kind);
            if x == end_x && y == end_y {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn step(&mut self) {
        self.updated.fill(false);
        self.frame_count = self.frame_count.wrapping_add(1);

        for y in (0..self.height).rev() {
            let left_to_right = (y + self.frame_count) % 2 == 0;

            for step_x in 0..self.width {
                let x = if left_to_right {
                    step_x
                } else {
                    self.width - 1 - step_x
                };

                let i = self.idx(x, y);
                if self.updated[i] {
                    continue;
                }

                match self.cells[i].kind {
                    Element::Empty | Element::Wall | Element::Wood | Element::Concrete => {}
                    Element::Metal => self.update_metal(x, y),
                    Element::Void => self.update_void(x, y),
                    Element::Glass => self.update_glass(x, y),
                    Element::Rubber => self.update_rubber(x, y),
                    Element::Stone => self.update_stone(x, y),
                    Element::Clay => self.update_clay(x, y),
                    Element::Sand | Element::Gunpowder | Element::Salt => self.update_powder(x, y),
                    Element::Tnt => self.update_tnt(x, y),
                    Element::Fuse => self.update_fuse(x, y),
                    Element::Water | Element::SaltWater => self.update_liquid(x, y, 4),
                    Element::Oil => self.update_oil(x, y),
                    Element::Acid => self.update_acid(x, y),
                    Element::Nitro => self.update_nitro(x, y),
                    Element::Lava => self.update_lava(x, y),
                    Element::Plant => self.update_plant(x, y),
                    Element::Ice => self.update_ice(x, y),
                    Element::Fire => self.update_fire(x, y),
                    Element::Smoke => self.update_smoke(x, y),
                    Element::Steam => self.update_steam(x, y),
                }
            }
        }
    }

    fn update_metal(&mut self, x: usize, y: usize) {
        let i = self.idx(x, y);
        let mut temp = self.cells[i].life;

        let neighbors = [
            (x as isize - 1, y as isize),
            (x as isize + 1, y as isize),
            (x as isize, y as isize - 1),
            (x as isize, y as isize + 1),
        ];

        for (nx, ny) in neighbors {
            if !self.in_bounds(nx, ny) {
                continue;
            }
            let ni = self.idx(nx as usize, ny as usize);
            match self.cells[ni].kind {
                Element::Lava => {
                    temp = temp.saturating_add(14);
                }
                Element::Fire => {
                    temp = temp.saturating_add(7);
                }
               Element::Metal => {
                    let n_temp = self.cells[ni].life;
                    if n_temp > temp.saturating_add(1) {
                        let transfer = ((n_temp - temp) / 3).max(1);
                        temp = temp.saturating_add(transfer);
                        self.cells[ni].life = n_temp.saturating_sub(transfer);
                    }
                }
                Element::Water => {
                    if temp > 95 {
                        temp = temp.saturating_sub(25);
                        self.cells[ni] = Particle {
                            kind: Element::Steam,
                            life: 140 + (self.rand_u8() % 80),
                            shade: self.rand_u8() % 25,
                        };
                        self.updated[ni] = true;
                    }
                }
                Element::SaltWater => {
                    if temp > 95 {
                        temp = temp.saturating_sub(25);
                        self.cells[ni] = Particle {
                            kind: Element::Salt,
                            life: 0,
                            shade: self.rand_u8() % 25,
                        };
                        self.updated[ni] = true;
                    }
                }
                Element::Ice => {
                    if temp > 35 {
                        temp = temp.saturating_sub(20);
                        self.cells[ni] = Particle {
                            kind: Element::Water,
                            life: 0,
                            shade: self.rand_u8() % 30,
                        };
                        self.updated[ni] = true;
                    }
                }
                Element::Wood | Element::Plant | Element::Fuse | Element::Rubber => {
                    if temp > 130 && self.rand_u8() < 35 {
                        self.cells[ni] = Particle {
                            kind: Element::Fire,
                            life: 60 + (self.rand_u8() % 40),
                            shade: self.rand_u8() % 30,
                        };
                        self.updated[ni] = true;
                    }
                }
                Element::Gunpowder => {
                    if temp > 135 {
                        self.explode(nx as usize, ny as usize, 14, 0, false);
                    }
                }
                Element::Tnt => {
                    if temp > 135 {
                        self.cells[ni] = Particle::empty();
                        self.explode(nx as usize, ny as usize, 26, 0, true);
                    }
                }
                Element::Nitro => {
                    if temp > 135 {
                        self.cells[ni] = Particle::empty();
                        self.explode(nx as usize, ny as usize, 22, 0, true);
                    }
                }
                _ => {}
            }
        }

        if temp > 0 && self.rand_u8() < 28 {
            temp -= 1;
        }

        self.cells[i].life = temp;
    }

    fn update_glass(&mut self, x: usize, y: usize) {
        let i = self.idx(x, y);
        let neighbors = [
            (x as isize - 1, y as isize),
            (x as isize + 1, y as isize),
            (x as isize, y as isize - 1),
            (x as isize, y as isize + 1),
        ];

        for (nx, ny) in neighbors {
            if !self.in_bounds(nx, ny) {
                continue;
            }
            let ni = self.idx(nx as usize, ny as usize);
            // Szkło pęka i topi się przy bezpośrednim kontakcie z lawą
            if self.cells[ni].kind == Element::Lava && self.rand_u8() < 12 {
                self.cells[i] = Particle::empty();
                return;
            }
        }
    }

    fn update_rubber(&mut self, x: usize, y: usize) {
        let i = self.idx(x, y);
        let neighbors = [
            (x as isize - 1, y as isize),
            (x as isize + 1, y as isize),
            (x as isize, y as isize - 1),
            (x as isize, y as isize + 1),
        ];

        for (nx, ny) in neighbors {
            if !self.in_bounds(nx, ny) {
                continue;
            }
            let ni = self.idx(nx as usize, ny as usize);
            if matches!(self.cells[ni].kind, Element::Fire | Element::Lava) && self.rand_u8() < 25 {
                self.cells[i] = Particle {
                    kind: Element::Fire,
                    life: 80 + (self.rand_u8() % 40),
                    shade: self.rand_u8() % 30,
                };
                return;
            }
        }
    }

    fn update_stone(&mut self, x: usize, y: usize) {
        let i = self.idx(x, y);
        let neighbors = [
            (x as isize - 1, y as isize),
            (x as isize + 1, y as isize),
            (x as isize, y as isize - 1),
            (x as isize, y as isize + 1),
        ];

        for (nx, ny) in neighbors {
            if !self.in_bounds(nx, ny) {
                continue;
            }
            let ni = self.idx(nx as usize, ny as usize);
            // Kamień przy długim kontakcie z lawą topi się w lawę
            if self.cells[ni].kind == Element::Lava && self.rand_u8() < 3 {
                self.cells[i] = Particle {
                    kind: Element::Lava,
                    life: 0,
                    shade: self.rand_u8() % 30,
                };
                return;
            }
        }
    }

    fn update_clay(&mut self, x: usize, y: usize) {
        let i = self.idx(x, y);
        let neighbors = [
            (x as isize - 1, y as isize),
            (x as isize + 1, y as isize),
            (x as isize, y as isize - 1),
            (x as isize, y as isize + 1),
        ];

        for (nx, ny) in neighbors {
            if !self.in_bounds(nx, ny) {
                continue;
            }
            let ni = self.idx(nx as usize, ny as usize);
            // Glina pod wpływem ognia lub lawy wypala się w twardy kamień
            if matches!(self.cells[ni].kind, Element::Fire | Element::Lava) && self.rand_u8() < 25 {
                self.cells[i] = Particle {
                    kind: Element::Stone,
                    life: 0,
                    shade: self.rand_u8() % 30,
                };
                return;
            }
        }
    }

    fn update_void(&mut self, x: usize, y: usize) {
        let neighbors = [
            (x as isize - 1, y as isize),
            (x as isize + 1, y as isize),
            (x as isize, y as isize - 1),
            (x as isize, y as isize + 1),
        ];

        for (nx, ny) in neighbors {
            if self.in_bounds(nx, ny) {
                let ni = self.idx(nx as usize, ny as usize);
                let nkind = self.cells[ni].kind;
                if nkind != Element::Empty && nkind != Element::Void && nkind != Element::Wall {
                    self.cells[ni] = Particle::empty();
                    self.updated[ni] = true;
                }
            }
        }
    }

    fn update_powder(&mut self, x: usize, y: usize) {
        let i = self.idx(x, y);
        let kind = self.cells[i].kind;

        let neighbors = [
            (x as isize - 1, y as isize),
            (x as isize + 1, y as isize),
            (x as isize, y as isize - 1),
            (x as isize, y as isize + 1),
        ];

        if kind == Element::Gunpowder {
            for (nx, ny) in neighbors {
                if self.in_bounds(nx, ny) {
                    let nkind = self.cells[self.idx(nx as usize, ny as usize)].kind;
                    if nkind == Element::Fire || nkind == Element::Lava {
                        self.explode(x, y, 14, 0, false);
                        return;
                    }
                }
            }
        } else if kind == Element::Salt {
            for (nx, ny) in neighbors {
                if self.in_bounds(nx, ny) {
                    let ni = self.idx(nx as usize, ny as usize);
                    match self.cells[ni].kind {
                        Element::Water => {
                            self.cells[ni] = Particle {
                                kind: Element::SaltWater,
                                life: 0,
                                shade: self.rand_u8() % 25,
                            };
                            self.cells[i] = Particle::empty();
                            self.updated[ni] = true;
                            return;
                        }
                        Element::Ice => {
                            if self.rand_u8() < 35 {
                                self.cells[ni] = Particle {
                                    kind: Element::Water,
                                    life: 0,
                                    shade: self.rand_u8() % 30,
                                };
                                self.cells[i] = Particle::empty();
                                self.updated[ni] = true;
                                return;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        let dir: isize = if self.rand_bool() { -1 } else { 1 };

        if self.can_displace(kind, x as isize, y as isize + 1) {
            self.swap(x, y, x, y + 1);
            return;
        }

        if self.can_displace(kind, x as isize + dir, y as isize + 1) {
            self.swap(x, y, (x as isize + dir) as usize, y + 1);
        } else if self.can_displace(kind, x as isize - dir, y as isize + 1) {
            self.swap(x, y, (x as isize - dir) as usize, y + 1);
        }
    }

    fn update_tnt(&mut self, x: usize, y: usize) {
        let neighbors = [
            (x as isize - 1, y as isize),
            (x as isize + 1, y as isize),
            (x as isize, y as isize - 1),
            (x as isize, y as isize + 1),
        ];

        for (nx, ny) in neighbors {
            if self.in_bounds(nx, ny) {
                let nkind = self.cells[self.idx(nx as usize, ny as usize)].kind;
                if matches!(nkind, Element::Fire | Element::Lava | Element::Acid) {
                    let i = self.idx(x, y);
                    self.cells[i] = Particle::empty();
                    self.explode(x, y, 26, 0, true);
                    return;
                }
            }
        }
    }

    fn update_fuse(&mut self, x: usize, y: usize) {
        let neighbors = [
            (x as isize - 1, y as isize),
            (x as isize + 1, y as isize),
            (x as isize, y as isize - 1),
            (x as isize, y as isize + 1),
        ];

        for (nx, ny) in neighbors {
            if self.in_bounds(nx, ny) {
                let nkind = self.cells[self.idx(nx as usize, ny as usize)].kind;
                if (nkind == Element::Fire || nkind == Element::Lava) && self.rand_u8() < 75 {
                    let i = self.idx(x, y);
                    self.cells[i] = Particle {
                        kind: Element::Fire,
                        life: 16,
                        shade: self.rand_u8() % 30,
                    };
                    self.updated[i] = true;
                    return;
                }
            }
        }
    }

    fn update_liquid(&mut self, x: usize, y: usize, dispersion: isize) {
        let kind = self.cells[self.idx(x, y)].kind;
        let dir: isize = if self.rand_bool() { -1 } else { 1 };

        if self.can_displace(kind, x as isize, y as isize + 1) {
            self.swap(x, y, x, y + 1);
            return;
        }

        if self.can_displace(kind, x as isize + dir, y as isize + 1) {
            self.swap(x, y, (x as isize + dir) as usize, y + 1);
            return;
        }
        if self.can_displace(kind, x as isize - dir, y as isize + 1) {
            self.swap(x, y, (x as isize - dir) as usize, y + 1);
            return;
        }

        let mut target_x = None;
        for dist in 1..=dispersion {
            let nx = x as isize + dir * dist;
            if self.can_displace(kind, nx, y as isize) {
                target_x = Some(nx as usize);
            } else {
                break;
            }
        }
        if let Some(tx) = target_x {
            self.swap(x, y, tx, y);
        }
    }

    fn update_oil(&mut self, x: usize, y: usize) {
        let neighbors = [
            (x as isize - 1, y as isize),
            (x as isize + 1, y as isize),
            (x as isize, y as isize - 1),
            (x as isize, y as isize + 1),
        ];

        for (nx, ny) in neighbors {
            if self.in_bounds(nx, ny) {
                let nkind = self.cells[self.idx(nx as usize, ny as usize)].kind;
                if nkind == Element::Fire || nkind == Element::Lava {
                    let i = self.idx(x, y);
                    self.cells[i] = Particle {
                        kind: Element::Fire,
                        life: 100 + (self.rand_u8() % 50),
                        shade: self.rand_u8() % 30,
                    };
                    self.updated[i] = true;
                    return;
                }
            }
        }

        self.update_liquid(x, y, 4);
    }

    fn update_nitro(&mut self, x: usize, y: usize) {
        let neighbors = [
            (x as isize - 1, y as isize),
            (x as isize + 1, y as isize),
            (x as isize, y as isize - 1),
            (x as isize, y as isize + 1),
        ];

        for (nx, ny) in neighbors {
            if self.in_bounds(nx, ny) {
                let nkind = self.cells[self.idx(nx as usize, ny as usize)].kind;
                if matches!(nkind, Element::Fire | Element::Lava | Element::Acid) {
                    let i = self.idx(x, y);
                    self.cells[i] = Particle::empty();
                    self.explode(x, y, 22, 0, true);
                    return;
                }
            }
        }

        self.update_liquid(x, y, 5);
    }

    fn update_acid(&mut self, x: usize, y: usize) {
        let i = self.idx(x, y);
        let neighbors = [
            (x as isize - 1, y as isize),
            (x as isize + 1, y as isize),
            (x as isize, y as isize - 1),
            (x as isize, y as isize + 1),
        ];

        for (nx, ny) in neighbors {
            if !self.in_bounds(nx, ny) {
                continue;
            }
            let ni = self.idx(nx as usize, ny as usize);
            let nkind = self.cells[ni].kind;

            // Szkło (Glass) i guma (Rubber) są odporne na kwas
            let dissolvable = matches!(
                nkind,
                Element::Wall
                    | Element::Metal
                    | Element::Wood
                    | Element::Plant
                    | Element::Sand
                    | Element::Gunpowder
                    | Element::Salt
                    | Element::Fuse
                    | Element::Ice
                    | Element::Concrete
                    | Element::Stone
                    | Element::Clay
            );

            if dissolvable && self.rand_u8() < 35 {
                self.cells[ni] = Particle {
                    kind: Element::Smoke,
                    life: 30 + (self.rand_u8() % 30),
                    shade: self.rand_u8() % 20,
                };
                self.updated[ni] = true;

                if self.cells[i].life <= 1 {
                    self.cells[i] = Particle::empty();
                    return;
                } else {
                    self.cells[i].life -= 1;
                }
            }
        }

        self.update_liquid(x, y, 4);
    }

    fn update_lava(&mut self, x: usize, y: usize) {
        let neighbors = [
            (x as isize - 1, y as isize),
            (x as isize + 1, y as isize),
            (x as isize, y as isize - 1),
            (x as isize, y as isize + 1),
        ];

        for (nx, ny) in neighbors {
            if !self.in_bounds(nx, ny) {
                continue;
            }
            let ni = self.idx(nx as usize, ny as usize);
            match self.cells[ni].kind {
                Element::Water => {
                    let i = self.idx(x, y);
                    self.cells[i] = Particle {
                        kind: Element::Stone,
                        life: 0,
                        shade: self.rand_u8() % 30,
                    };
                    self.cells[ni] = Particle {
                        kind: Element::Steam,
                        life: 140 + (self.rand_u8() % 80),
                        shade: self.rand_u8() % 25,
                    };
                    self.updated[ni] = true;
                    return;
                }
                Element::SaltWater => {
                    let i = self.idx(x, y);
                    self.cells[i] = Particle {
                        kind: Element::Stone,
                        life: 0,
                        shade: self.rand_u8() % 30,
                    };
                    self.cells[ni] = Particle {
                        kind: Element::Salt,
                        life: 0,
                        shade: self.rand_u8() % 25,
                    };
                    self.updated[ni] = true;
                    return;
                }
                Element::Wood | Element::Plant | Element::Fuse | Element::Rubber => {
                    if self.rand_u8() < 45 {
                        self.cells[ni] = Particle {
                            kind: Element::Fire,
                            life: 70 + (self.rand_u8() % 50),
                            shade: self.rand_u8() % 30,
                        };
                        self.updated[ni] = true;
                    }
                }
                Element::Ice => {
                    self.cells[ni] = Particle {
                        kind: Element::Water,
                        life: 0,
                        shade: self.rand_u8() % 30,
                    };
                    self.updated[ni] = true;
                }
                _ => {}
            }
        }

        if self.rand_u8() < 100 {
            return;
        }

        let dir: isize = if self.rand_bool() { -1 } else { 1 };
        if self.is_empty(x as isize, y as isize + 1) {
            self.swap(x, y, x, y + 1);
        } else if self.is_empty(x as isize + dir, y as isize + 1) {
            self.swap(x, y, (x as isize + dir) as usize, y + 1);
        } else if self.is_empty(x as isize - dir, y as isize + 1) {
            self.swap(x, y, (x as isize - dir) as usize, y + 1);
        } else if self.is_empty(x as isize + dir, y as isize) {
            self.swap(x, y, (x as isize + dir) as usize, y);
        }
    }

    fn update_plant(&mut self, x: usize, y: usize) {
        let neighbors = [
            (x as isize - 1, y as isize),
            (x as isize + 1, y as isize),
            (x as isize, y as isize - 1),
            (x as isize, y as isize + 1),
        ];

        for (nx, ny) in neighbors {
            if self.in_bounds(nx, ny) {
                let ni = self.idx(nx as usize, ny as usize);
                if self.cells[ni].kind == Element::Water && self.rand_u8() < 30 {
                    self.cells[ni] = Particle {
                        kind: Element::Plant,
                        life: 0,
                        shade: self.rand_u8() % 30,
                    };
                    self.updated[ni] = true;
                }
            }
        }
    }

    fn update_ice(&mut self, x: usize, y: usize) {
        let i = self.idx(x, y);
        let neighbors = [
            (x as isize - 1, y as isize),
            (x as isize + 1, y as isize),
            (x as isize, y as isize - 1),
            (x as isize, y as isize + 1),
        ];

        for (nx, ny) in neighbors {
            if !self.in_bounds(nx, ny) {
                continue;
            }
            let ni = self.idx(nx as usize, ny as usize);
            match self.cells[ni].kind {
                Element::Fire | Element::Lava => {
                    self.cells[i] = Particle {
                        kind: Element::Water,
                        life: 0,
                        shade: self.rand_u8() % 30,
                    };
                    return;
                }
                Element::Water => {
                    if self.rand_u8() < 2 {
                        self.cells[ni] = Particle {
                            kind: Element::Ice,
                            life: 0,
                            shade: self.rand_u8() % 20,
                        };
                        self.updated[ni] = true;
                    }
                }
                _ => {}
            }
        }
    }

    fn update_fire(&mut self, x: usize, y: usize) {
        let i = self.idx(x, y);
        if self.cells[i].life == 0 {
            if self.rand_u8() < 70 {
                self.cells[i] = Particle {
                    kind: Element::Smoke,
                    life: 60 + (self.rand_u8() % 60),
                    shade: self.rand_u8() % 25,
                };
            } else {
                self.cells[i] = Particle::empty();
            }
            return;
        }
        self.cells[i].life -= 1;

        let neighbors = [
            (x as isize - 1, y as isize),
            (x as isize + 1, y as isize),
            (x as isize, y as isize - 1),
            (x as isize, y as isize + 1),
        ];

        for (nx, ny) in neighbors {
            if !self.in_bounds(nx, ny) {
                continue;
            }
            let ni = self.idx(nx as usize, ny as usize);
            match self.cells[ni].kind {
                Element::Water => {
                    self.cells[i] = Particle {
                        kind: Element::Steam,
                        life: 120 + (self.rand_u8() % 60),
                        shade: 20,
                    };
                    return;
                }
                Element::SaltWater => {
                    self.cells[ni] = Particle {
                        kind: Element::Salt,
                        life: 0,
                        shade: self.rand_u8() % 25,
                    };
                    self.cells[i] = Particle {
                        kind: Element::Steam,
                        life: 100,
                        shade: 20,
                    };
                    self.updated[ni] = true;
                    return;
                }
                Element::Wood => {
                    if self.rand_u8() < 18 {
                        self.cells[ni] = Particle {
                            kind: Element::Fire,
                            life: 70 + (self.rand_u8() % 50),
                            shade: self.rand_u8() % 30,
                        };
                        self.updated[ni] = true;
                    }
                }
                Element::Plant => {
                    if self.rand_u8() < 45 {
                        self.cells[ni] = Particle {
                            kind: Element::Fire,
                            life: 50 + (self.rand_u8() % 40),
                            shade: self.rand_u8() % 30,
                        };
                        self.updated[ni] = true;
                    }
                }
                _ => {}
            }
        }

        let dx = (self.rand_u8() % 3) as isize - 1;
        let ny = y as isize - 1;
        if self.is_empty(x as isize + dx, ny) {
            self.swap(x, y, (x as isize + dx) as usize, ny as usize);
        }
    }

    fn update_smoke(&mut self, x: usize, y: usize) {
        let i = self.idx(x, y);
        if self.cells[i].life == 0 {
            self.cells[i] = Particle::empty();
            return;
        }
        self.cells[i].life -= 1;

        let dir: isize = if self.rand_bool() { -1 } else { 1 };
        let ny = y as isize - 1;

        if self.is_empty(x as isize, ny) {
            self.swap(x, y, x, ny as usize);
        } else if self.is_empty(x as isize + dir, ny) {
            self.swap(x, y, (x as isize + dir) as usize, ny as usize);
        }
    }

    fn update_steam(&mut self, x: usize, y: usize) {
        let i = self.idx(x, y);
        if self.cells[i].life == 0 {
            self.cells[i] = Particle {
                kind: Element::Water,
                life: 0,
                shade: self.rand_u8() % 30,
            };
            return;
        }
        self.cells[i].life -= 1;

        let dir: isize = if self.rand_bool() { -1 } else { 1 };
        let ny = y as isize - 1;

        if self.is_empty(x as isize, ny) {
            self.swap(x, y, x, ny as usize);
        } else if self.is_empty(x as isize + dir, ny) {
            self.swap(x, y, (x as isize + dir) as usize, ny as usize);
        } else if self.is_empty(x as isize + dir, y as isize) {
            self.swap(x, y, (x as isize + dir) as usize, y);
        }
    }

    fn explode(&mut self, cx: usize, cy: usize, radius: isize, depth: u8, destroy_walls: bool) {
        let r_sq = radius * radius;
        let inner_r_sq = (radius - 3).max(1) * (radius - 3).max(1);

        for dy in -radius..=radius {
            for dx in -radius..=radius {
                let dist_sq = dx * dx + dy * dy;
                if dist_sq > r_sq {
                    continue;
                }

                if dist_sq > inner_r_sq && self.rand_u8() < 90 {
                    continue;
                }

                let nx = cx as isize + dx;
                let ny = cy as isize + dy;
                if !self.in_bounds(nx, ny) {
                    continue;
                }

                let i = self.idx(nx as usize, ny as usize);
                let kind = self.cells[i].kind;

                let blast_resistant = matches!(
                    kind,
                    Element::Wall
                        | Element::Concrete
                        | Element::Stone
                        | Element::Rubber
                );

                if (blast_resistant && !destroy_walls) || kind == Element::Void {
                    continue;
                }

                if kind == Element::Metal && !destroy_walls {
                    self.cells[i].life = self.cells[i].life.saturating_add(90);
                    continue;
                }

                if kind == Element::Gunpowder && depth < 3 {
                    self.cells[i] = Particle::empty();
                    self.explode(nx as usize, ny as usize, 12, depth + 1, destroy_walls);
                    continue;
                }

                if kind == Element::Tnt && depth < 3 {
                    self.cells[i] = Particle::empty();
                    self.explode(nx as usize, ny as usize, 24, depth + 1, true);
                    continue;
                }

                if kind == Element::Nitro && depth < 3 {
                    self.cells[i] = Particle::empty();
                    self.explode(nx as usize, ny as usize, 20, depth + 1, true);
                    continue;
                }

                if self.rand_u8() < 180 {
                    self.cells[i] = Particle {
                        kind: Element::Fire,
                        life: 30 + (self.rand_u8() % 45),
                        shade: self.rand_u8() % 30,
                    };
                } else {
                    self.cells[i] = Particle {
                        kind: Element::Smoke,
                        life: 50 + (self.rand_u8() % 50),
                        shade: self.rand_u8() % 25,
                    };
                }
                self.updated[i] = true;
            }
        }
    }

    pub fn to_color_image(&self, bg: egui::Color32) -> egui::ColorImage {
        let pixels: Vec<egui::Color32> = self.cells.iter().map(|p| p.color(bg)).collect();
        egui::ColorImage::new([self.width, self.height], pixels)
    }
}