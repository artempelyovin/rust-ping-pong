use crate::constants::{PADDLE_HEIGHT, PADDLE_WIDTH, WINDOW_WIDTH};
use crate::entities::ball::Ball;
use macroquad::color::{Color, WHITE};
use macroquad::shapes::draw_line;

pub struct Paddle {
    x: f32,
    y: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
}

impl Paddle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
            color: WHITE,
        }
    }
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
