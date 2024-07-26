mod player;

use player::Player;
use raylib::prelude::*;

fn main() {
    // ------------------------------------------------------------------------------------
    // Initialize variables
    // ------------------------------------------------------------------------------------
    let (mut rl, thread) = raylib::init().size(640, 480).title("Basic Game").build();

    let mut player: Player = Player::new(
        rl.get_screen_width() / 2, // x
        rl.get_screen_height() / 2, // y
        3,
        1.0, // speed
        rl.load_texture(&thread, "img/player.png")
            .expect("Error loading the texture"),
    );
    // ------------------------------------------------------------------------------------

    // ------------------------------------------------------------------------------------
    // Game loop
    // ------------------------------------------------------------------------------------
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        // ------------------------------------------------------------------------------------
        // Event handling
        // ------------------------------------------------------------------------------------
        player.movement(&d);
        player.edge_collision(&d);
        // ------------------------------------------------------------------------------------

        // ------------------------------------------------------------------------------------
        // Draw elements
        // ------------------------------------------------------------------------------------
        d.clear_background(Color::BLACK);
        d.draw_texture(
            &player.sprite,
            player.x as i32,
            player.y as i32,
            Color::WHITE,
        );
        // ------------------------------------------------------------------------------------
    }
    // ------------------------------------------------------------------------------------
}
