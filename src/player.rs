use raylib::prelude::*;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub width: i32,
    pub height: i32,
    pub speed: f32,
    pub sprite: Texture2D,
}

impl Player {
    pub fn new(x: f32, y: f32, size: i32, speed: f32, sprite: Texture2D) -> Self {
        let mut sprite = sprite;
        sprite.width = size * sprite.width;
        sprite.height = size * sprite.height;

        Player {
            x,
            y,
            width: sprite.width(),
            height: sprite.height(),
            speed,
            sprite,
        }
    }

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
        } else if self.y > d.get_screen_height() as f32 - self.height as f32 {
            self.y = d.get_screen_height() as f32 - self.height as f32;
        }

        if self.x < 0.0 {
            self.x = 0.0;
        } else if self.x > d.get_screen_width() as f32 - self.width as f32 {
            self.x = d.get_screen_width() as f32 - self.width as f32;
        }
    }
}
