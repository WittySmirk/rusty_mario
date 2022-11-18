use macroquad::prelude::*;

mod tilemap;
mod mario;
use mario::Player;
use tilemap::read_map;


struct GameSettings {
    scale: i32,
}

const SETTINGS: GameSettings = GameSettings { scale: 3 };

// Bunch of window settings
//TODO: Proper window icon
fn window_conf() -> Conf {
    Conf {
        window_title: "Mario".to_owned(),
        window_width: 240 * SETTINGS.scale,
        window_height: 240 * SETTINGS.scale,
        window_resizable: false, // Disable this once we figure out how to do scaling based on resize
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let screen_bg: Color = Color::from_rgba(90, 147, 245, 100);
    let mut mario: Player = Player::new().await;
    
    let map: String = read_map("res/maps/1-1.lvl");
    println!("{}", map);

    loop {
        //Update shit
        mario.update(get_frame_time());

        //Animate shit
        mario.animate();

        clear_background(screen_bg);

        //Draw shit
        mario.draw();

        next_frame().await;
    }
}
