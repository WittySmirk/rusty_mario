use macroquad::prelude::*;

mod block;
mod entity;
mod mario;
mod texture_manager;
mod tilemap;
mod question;
use entity::{Entity, EntityType};
use tilemap::TileMapController;


pub type World = Vec<Entity>;

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

    let mut tilemapcontrol: TileMapController = TileMapController::new();
    tilemapcontrol.read_map("res/maps/1-1.lvl");
    let mut world: World = tilemapcontrol.spawn_from_map().await;
    let mut camera: Camera2D =
        Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height()));
    //TODO: Fix mario blinking lol

    loop {
        // Update all entities
        for i in 0..world.len() {
            // Only mario has collision so check if him and do collision
            if let EntityType::Mario = world[i].e_type {
                // Set mario to be the camera target every frame
                // if world[i].hitbox.expect("player should have hitbox").x > screen_width() / 2.0 {
                // println!("Updating camera position");
                // }

                world[i].entity.update(get_frame_time(), Some(&mut camera));
                for j in 0..world.len() {
                    let other_hit: Option<Rect> = world[j].hitbox;
                    world[i].entity.collision(&other_hit);
                }
            }
            world[i].entity.animate();
        }

        clear_background(screen_bg);

        // Draw shit
        set_camera(&camera);
        for entity in world.iter_mut() {
            entity.entity.draw(texture_atlas);
        }

        next_frame().await;
    }
}
