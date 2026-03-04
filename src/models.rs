use crate::constants::{BALL_START_SPEED, CENTER_X, CENTER_Y, WINDOW_HEIGHT, WINDOW_WIDTH};
use macroquad::color::Color;
use macroquad::prelude::{draw_circle, draw_line};

pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
}

impl Paddle {
    fn calculate_x1_x2(&self) -> (f32, f32) {
        let x1 = if self.x < 0.0 {
            0.0
        } else if self.x > (WINDOW_WIDTH - self.width) {
            WINDOW_WIDTH - self.width
        } else {
            self.x
        };
        let x2 = x1 + self.width;
        (x1, x2)
    }

    pub fn move_to(&mut self, x: f32) {
        self.x = x;
    }

    pub fn draw(&self) {
        let (x1, x2) = self.calculate_x1_x2();
        draw_line(x1, self.y, x2, self.y, self.height, self.color);
    }

    pub fn has_collision_with_ball(&self, ball: &Ball) -> bool {
        let (x1, x2) = self.calculate_x1_x2();
        // Проверяем горизонтальное пересечение (мяч находится в пределах ширины ракетки)
        let horizontal_collision = ball.x >= x1 && ball.x <= x2;

        // Проверяем вертикальное пересечение (мяч касается верхней части ракетки)
        // Ракетка - это горизонтальная линия на высоте self.y
        let vertical_collision =
            (ball.y + ball.radius >= self.y) && (ball.y - ball.radius <= self.y + self.height);

        horizontal_collision && vertical_collision
    }
}

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

    pub fn tick(&mut self, player: &Paddle, enemy: &Paddle) {
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
