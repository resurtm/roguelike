mod direction;
mod input;
mod player;
mod player_sprite;

use crate::{direction::Direction, input::Input, player::Player, player_sprite::PlayerSprite};
use sdl2::{
    event::Event, image::LoadTexture, keyboard::Keycode, pixels::Color, render::Texture,
};
use std::{collections::HashMap, thread::sleep, time::Duration};
use tiled::Loader;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("roguelike", 1024, 768)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .load_texture("./assets/red-tile.png")
        .unwrap();

    let mut loader = Loader::new();
    let map = loader.load_tmx_map("./assets/orc/tiled/Orc3.tmx").unwrap();

    let mut tex_cache: HashMap<String, Texture> = HashMap::new();
    for tileset in map.tilesets().iter() {
        if let Some(image) = &tileset.image {
            let img = texture_creator.load_texture(&image.source);
            tex_cache.insert(image.source.to_str().unwrap().to_string(), img.unwrap());
        }
    }

    let layer = map.get_layer(0).unwrap();
    println!("Layer: {:?}", layer);
    match layer.layer_type() {
        tiled::LayerType::Tiles(layer) => match layer {
            tiled::TileLayer::Finite(data) => {
                println!("{}x{}", data.width(), data.height());

                println!("{:?}", data.get_tile_data(1, 0).unwrap());
                println!("{:?}", data.get_tile(1, 0).unwrap().id());
                println!(
                    "{:?}",
                    data.get_tile(1, 0).unwrap().get_tile().unwrap().animation
                );

                println!("{:?}", data.get_tile_data(1, 1).unwrap());
                println!("{:?}", data.get_tile(1, 1).unwrap().id());
                println!(
                    "{:?}",
                    data.get_tile(1, 1).unwrap().get_tile().unwrap().animation
                );

                println!("{:?}", data.get_tile_data(1, 2).unwrap());
                println!("{:?}", data.get_tile(1, 2).unwrap().id());
                println!(
                    "{:?}",
                    data.get_tile(1, 2).unwrap().get_tile().unwrap().animation
                );
            }
            _ => {}
        },
        _ => {}
    }

    let mut player = Player::new();
    let mut player_sprite = PlayerSprite::new(&texture_creator);
    let mut input = Input::new();

    'running: loop {
        // events and input
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown { .. } | Event::KeyUp { .. } => {
                    input.handle_key_event(&event);
                }
                _ => {}
            }
        }

        // calculate
        if input.key_up {
            player.thrust(Direction::Up);
        }
        if input.key_down {
            player.thrust(Direction::Down);
        }
        if input.key_left {
            player.thrust(Direction::Left);
        }
        if input.key_right {
            player.thrust(Direction::Right);
        }

        player.advance();
        player_sprite.advance();
        player_sprite.position = player.position;

        // render
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // canvas
        //     .copy(
        //         &texture,
        //         None,
        //         Rect::new(
        //             (player.position.0 - player.size) as i32,
        //             (player.position.1 - player.size) as i32,
        //             (player.size * 2.0) as u32,
        //             (player.size * 2.0) as u32,
        //         ),
        //     )
        //     .unwrap();

        // canvas
        //     .copy(
        //         &tex_cache
        //             .get("./assets/orc/tiled/orc3_idle_full.png")
        //             .unwrap(),
        //         Rect::new(0, 0, 64, 64),
        //         Rect::new(400, 400, 128, 128),
        //     )
        //     .unwrap();

        player_sprite.render(&mut canvas);

        // present and sleep
        canvas.present();
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
