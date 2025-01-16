use std::ops::{Add, Mul};

use embedded_graphics::geometry::Point;

pub const DIAMETER: u32 = 10;

const SPEED: i32 = 1;
const SCREEN_WIDHT: i32 = 128;
const SCREEN_HEIGHT: i32 = 64;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn to_point(self) -> Point {
        Point::new(self.x, self.y)
    }

    pub fn try_move(&mut self, x: i32, y: i32) {
        let dir = Position::new(x, y);
        let new_pos = *self + dir * SPEED;

        let out_left = (new_pos.x) < 0;
        let out_right = (new_pos.x + DIAMETER as i32) >= SCREEN_WIDHT;
        let out_top = (new_pos.y) < 0;
        let out_bottom = (new_pos.y + DIAMETER as i32) >= SCREEN_HEIGHT;

        if !(out_left || out_right || out_top || out_bottom) {
            *self = new_pos;
        }
    }
}

impl Add<Position> for Position {
    type Output = Self;
    fn add(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Mul<i32> for Position {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Position {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
