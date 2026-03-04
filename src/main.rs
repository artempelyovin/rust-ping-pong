mod constants;
mod entities;

use crate::constants::{BALL_RADIUS, CENTER_X, CENTER_Y};
use crate::entities::paddle::Paddle;
use constants::{PADDLE_HEIGHT, PADDLE_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};
use entities::ball::Ball;
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
    let mut player_score = 0;
    let mut enemy_score = 0;
    let mut ball = Ball::new(CENTER_X, CENTER_Y, BALL_RADIUS, WHITE);
    let mut player_paddle = Paddle {
        x: CENTER_X,
        y: WINDOW_HEIGHT - PADDLE_HEIGHT / 2.0,
        width: PADDLE_WIDTH,
        height: PADDLE_HEIGHT,
        color: WHITE,
    };
    let mut enemy_paddle = Paddle {
        x: CENTER_X,
        y: 0.0 + PADDLE_HEIGHT / 2.0,
        width: PADDLE_WIDTH,
        height: PADDLE_HEIGHT,
        color: WHITE,
    };
    loop {
        ball.tick(&player_paddle, &enemy_paddle);
        let (x, _) = mouse_position();
        player_paddle.move_to(x);
        enemy_paddle.move_to(ball.x - enemy_paddle.width / 2.0);

        if ball.check_player_loss() {
            enemy_score += 1;
            ball.respawn_from_center();
        } else if ball.check_enemy_loss() {
            player_score += 1;
            ball.respawn_from_center();
        } else {
            ball.accelerate_ball();
        }

        let score = format!("{}: {}", player_score, enemy_score);
        draw_text(score.as_str(), CENTER_X - 50.0, CENTER_Y, 50.0, WHITE);
        ball.draw();
        player_paddle.draw();
        enemy_paddle.draw();

        next_frame().await
    }
}
