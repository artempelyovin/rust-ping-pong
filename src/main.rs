mod constants;
mod entities;
mod game;
use crate::game::Game;
use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Ping Pong".to_string(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    loop {
        game.update();
        game.draw();
        next_frame().await
    }
}
