use crate::constants::{BALL_RADIUS, CENTER_X, CENTER_Y, PADDLE_HEIGHT, WINDOW_HEIGHT};
use crate::entities::ball::Ball;
use crate::entities::paddle::Paddle;
use macroquad::color::WHITE;
use macroquad::input::mouse_position;
use macroquad::prelude::draw_text;

pub struct Game {
    player_paddle: Paddle,
    enemy_paddle: Paddle,
    ball: Ball,
    player_score: u32,
    enemy_score: u32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player_paddle: Paddle::new(CENTER_X, WINDOW_HEIGHT - PADDLE_HEIGHT / 2.0),
            enemy_paddle: Paddle::new(CENTER_X, 0.0 + PADDLE_HEIGHT / 2.0),
            ball: Ball::new(CENTER_X, CENTER_Y, BALL_RADIUS, WHITE),
            player_score: 0,
            enemy_score: 0,
        }
    }

    fn update_player(&mut self) {
        let (x, _) = mouse_position();
        self.player_paddle.move_to(x);
    }

    fn update_enemy(&mut self) {
        // Invincible AI :D
        self.enemy_paddle
            .move_to(self.ball.x - self.enemy_paddle.width / 2.0);
    }

    fn update_score(&mut self) {
        if self.ball.check_player_loss() {
            self.enemy_score += 1;
            self.ball.respawn_from_center();
        } else if self.ball.check_enemy_loss() {
            self.player_score += 1;
            self.ball.respawn_from_center();
        } else {
            self.ball.accelerate_ball();
        }
    }

    pub fn update(&mut self) {
        self.update_player();
        self.update_enemy();
        self.ball.update(&self.player_paddle, &self.enemy_paddle);
        self.update_score();
    }

    pub fn draw(&self) {
        let score = format!("{}: {}", self.player_score, self.enemy_score);
        draw_text(score.as_str(), CENTER_X - 50.0, CENTER_Y, 50.0, WHITE);
        self.ball.draw();
        self.player_paddle.draw();
        self.enemy_paddle.draw();
    }
}
