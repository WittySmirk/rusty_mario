use macroquad::prelude::*;

mod tilemap;
mod entity;
mod mario;
mod texture_manager;
use mario::Player;
use tilemap::TileMapController;
use entity::Physics;
use entity::Render;


struct GameSettings {
    scale: i32,
}

// const SETTINGS: GameSettings = GameSettings { scale: 3 };

struct GameConsts {
    settings: GameSettings,
    mario_size: Vec2,
    block_size: i32,
}

const CONSTS: GameConsts = GameConsts {
    settings: GameSettings { scale: 3 },
    mario_size: Vec2::new(13.0 * 3f32, 16.0 * 3f32),
    block_size: 16 * 3,
};

// Bunch of window settings
//TODO: Proper window icon
fn window_conf() -> Conf {
    Conf {
        window_title: "Mario".to_owned(),
        window_width: 240 * CONSTS.settings.scale,
        window_height: 240 * CONSTS.settings.scale,
        window_resizable: false, // Disable this once we figure out how to do scaling based on resize       
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let texture_atlas: Texture2D = load_texture("res/mario_sprites.png").await.unwrap();
    let screen_bg: Color = Color::from_rgba(90, 147, 245, 100);
    let mut mario: Player = Player::new(texture_manager::Entity::Mario.get_start()).await;

    let mut tilemapcontrol: TileMapController = TileMapController::new();
    tilemapcontrol.read_map("res/maps/1-1.lvl");
    tilemapcontrol.spawn_from_map();

    loop {
        //Update shit
        mario.update(get_frame_time());

        //Animate shit
        mario.animate();

        clear_background(screen_bg);

        //Draw shit
        mario.draw(texture_atlas);

        next_frame().await;
    }
}
