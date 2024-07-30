mod meteorite;
mod player;

use meteorite::Meteorite;
use player::Player;
use rand::{thread_rng, Rng};
use raylib::prelude::*;

fn main() {
    // ------------------------------------------------------------------------------------
    // Initialize variables
    // ------------------------------------------------------------------------------------
    let (mut rl, thread) = raylib::init().size(640, 480).title("Basic Game").build();

    let mut player: Player = Player::new(
        rl.get_screen_width().as_f32() / 2.0,  // X
        rl.get_screen_height().as_f32() / 2.0, // Y
        3,                                     // Size
        1.0,                                   // Speed
        rl.load_texture(&thread, "img/player.png") // Sprite
            .expect("Error loading the texture"),
    );

    let mut rng = thread_rng();
    let mut dificulty = 5.0;
    let mut meteorite: Meteorite = Meteorite::new(
        rng.gen_range(0.0..rl.get_screen_width().as_f32()), // X
        -40.0,                                              // Y
        3,                                                  // Size
        1.0,                                                // Speed
        rl.load_texture(&thread, "img/meteorite.png") // Sprite
            .expect("Error loading the texture"),
        Vector2 {
            x: player.x,
            y: player.y,
        }, // Direction
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

        meteorite.movement();
        
        // If the enemy get out of the screen, move it to the top
        if meteorite.y > (d.get_screen_height() + meteorite.height) as f32
            || meteorite.x > (d.get_screen_width() + meteorite.width) as f32
            || meteorite.x < 0.0 - meteorite.width as f32
        {
            meteorite.x = rng.gen_range(0.0..d.get_screen_width().as_f32());
            meteorite.y = -40.0;
            meteorite.direction = Vector2 {
                x: meteorite.x - player.x,
                y: meteorite.y - player.y,
            }
            .normalized();
        }

        // Check the player-enemy collision
        if player.as_rec().check_collision_recs(&meteorite.as_rec()) {
            break;
        }

        // Increase the meteorite's speed every 5 seconds
        if d.get_time() > dificulty {
            dificulty += 5.0;
            meteorite.speed += 0.5;
        }
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
        d.draw_texture(
            &meteorite.sprite,
            meteorite.x as i32,
            meteorite.y as i32,
            Color::WHITE,
        );
        // ------------------------------------------------------------------------------------
    }
    // ------------------------------------------------------------------------------------
}
