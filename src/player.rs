use raylib::prelude::*;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
}

impl Player {
    pub fn movement(&mut self, d: &RaylibDrawHandle) {
        if d.is_key_down(KeyboardKey::KEY_W) {
            self.y -= self.speed;
        }
        if d.is_key_down(KeyboardKey::KEY_S) {
            self.y += self.speed;
        }
        if d.is_key_down(KeyboardKey::KEY_A) {
            self.x -= self.speed;
        }
        if d.is_key_down(KeyboardKey::KEY_D) {
            self.x += self.speed;
        }
    }

    pub fn edge_collision(&mut self, d: &RaylibDrawHandle) {
        if self.y < 0.0 {
            self.y = 0.0;
        } else if self.y > d.get_screen_height().as_f32() - self.height {
            self.y = d.get_screen_height().as_f32() - self.height;
        }

        if self.x < 0.0 {
            self.x = 0.0;
        } else if self.x > d.get_screen_width().as_f32() - self.width {
            self.x = d.get_screen_width().as_f32() - self.width;
        }
    }
}
