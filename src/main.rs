use raylib::prelude::*;

fn main() {
    // Initialize variables
    let (mut rl, thread) = raylib::init().size(640, 480).title("Basic Game").build();
    let size: f32 = 30.0;
    let speed: f32 = 1.0;

    let mut player: Rectangle = Rectangle {
        x: rl.get_screen_width().as_f32() / 2.0,
        y: rl.get_screen_height().as_f32() / 2.0,
        width: size,
        height: size,
    };
    // -----------------------------------------------------------------

    // Game loop
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        // Event handling
        // Player movement
        if d.is_key_down(KeyboardKey::KEY_W) {
            player.y -= speed;
        }
        if d.is_key_down(KeyboardKey::KEY_S) {
            player.y += speed;
        }
        if d.is_key_down(KeyboardKey::KEY_A) {
            player.x -= speed;
        }
        if d.is_key_down(KeyboardKey::KEY_D) {
            player.x += speed;
        }

        // Player edge collision
        if player.y < 0.0 {
            player.y = 0.0;
        } else if player.y > d.get_screen_height().as_f32() - player.height {
            player.y = d.get_screen_height().as_f32() - player.height;
        }

        if player.x < 0.0 {
            player.x = 0.0;
        } else if player.x > d.get_screen_width().as_f32() - player.width {
            player.x = d.get_screen_width().as_f32() - player.width;
        }
        // -----------------------------------------------------------------

        // Draw elements
        d.clear_background(Color::WHITE);
        d.draw_rectangle_rec(player, Color::BLACK);
        // -----------------------------------------------------------------
    }
    // -----------------------------------------------------------------
}
