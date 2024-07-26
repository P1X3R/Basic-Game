use raylib::prelude::*;

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub width: f32,
    pub height: f32,
    pub speed: i32,
    pub sprite: Texture2D,
}

impl Player {
    pub fn new(x: i32, y: i32, size: i32, speed: i32, sprite: Texture2D) -> Self {
        let mut sprite = sprite;
        sprite.width = size * sprite.width;
        sprite.height = size * sprite.height;

        Player {
            x,
            y,
            width: sprite.width().as_f32(),
            height: sprite.height().as_f32(),
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
        if self.y < 0 {
            self.y = 0;
        } else if self.y > d.get_screen_height() as i32 - self.height as i32 {
            self.y = d.get_screen_height() as i32 - self.height as i32;
        }

        if self.x < 0 {
            self.x = 0;
        } else if self.x > d.get_screen_width() as i32 - self.width as i32 {
            self.x = d.get_screen_width() as i32 - self.width as i32;
        }
    }
}
