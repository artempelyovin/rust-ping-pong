use crate::constants::{BALL_START_SPEED, CENTER_X, CENTER_Y, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::entities::paddle::Paddle;
use macroquad::color::Color;
use macroquad::prelude::draw_circle;

pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub color: Color,
    dx: f32,
    dy: f32,
}

impl Ball {
    pub fn new(x: f32, y: f32, radius: f32, color: Color) -> Self {
        Self {
            x,
            y,
            radius,
            color,
            dx: -BALL_START_SPEED,
            dy: BALL_START_SPEED,
        }
    }

    pub fn respawn_from_center(&mut self) {
        self.x = CENTER_X;
        self.y = CENTER_Y;
        self.dx = -BALL_START_SPEED;
        self.dy = BALL_START_SPEED;
    }

    pub fn accelerate_ball(&mut self) {
        if self.dx < 0.0 {
            self.dx -= 0.01
        } else {
            self.dx += 0.01
        }
        if self.dy < 0.0 {
            self.dy -= 0.01
        } else {
            self.dy += 0.01
        }
    }

    pub fn update(&mut self, player: &Paddle, enemy: &Paddle) {
        self.x -= self.dx;
        self.y -= self.dy;
        // Столкновение о стенки
        if (self.x <= 0.0) || (self.x >= WINDOW_WIDTH) {
            self.dx *= -1.0;
        }
        // Столкновение с ракеткой игрока
        if player.has_collision_with_ball(&self) {
            self.dy *= -1.0;
        }
        // Столкновение с ракеткой противника
        if enemy.has_collision_with_ball(&self) {
            self.dy *= -1.0;
        }
    }

    pub fn check_player_loss(&self) -> bool {
        self.y > WINDOW_HEIGHT
    }

    pub fn check_enemy_loss(&self) -> bool {
        self.y < 0.0
    }

    pub fn draw(&self) {
        draw_circle(self.x, self.y, self.radius, self.color);
    }
}
