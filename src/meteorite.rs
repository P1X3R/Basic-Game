use raylib::prelude::*;

pub struct Meteorite {
    pub x: f32,
    pub y: f32,
    pub width: i32,
    pub height: i32,
    pub speed: f32,
    pub sprite: Texture2D,
    pub direction: Vector2,
}

impl Meteorite {
    pub fn new(
        x: f32,
        y: f32,
        size: i32,
        speed: f32,
        sprite: Texture2D,
        direction: Vector2,
    ) -> Self {
        let mut sprite = sprite;
        sprite.width = size + sprite.width;
        sprite.height = size + sprite.height;

        Meteorite {
            x,
            y,
            width: sprite.width,
            height: sprite.height,
            speed,
            sprite,
            direction: Vector2 {
                x: x - direction.x,
                y: y - direction.y,
            }
            .normalized(),
        }
    }

    pub fn movement(&mut self) {
        self.x -= self.direction.x * self.speed;
        self.y -= self.direction.y * self.speed;
    }

    pub fn as_rec(&self) -> Rectangle {
        Rectangle {
            x: self.x,
            y: self.y,
            width: self.width as f32,
            height: self.height as f32,
        }
    }
}
