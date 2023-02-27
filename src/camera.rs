use crate::prelude::*;

pub struct Camera {
    // pub left_x: i32,
    // pub right_x: i32,
    // pub top_y: i32,
    // pub bottom_y: i32,
    center: Point,
    // pub width: i32,
    // pub height: i32
    dx: i32,
    dy: i32,
}

impl Camera {
    pub fn new(center: Point, camera_width: i32, camera_height: i32) -> Self {
        Self {
            center,
            dx: camera_width / 2,
            dy: camera_height / 2,
        }
    }

    pub fn reposition(&mut self, center: Point) {
        self.center = center;
    }

    pub fn left(&self) -> i32 {
        self.center.x - self.dx
    }
    pub fn right(&self) -> i32 {
        self.center.x + self.dx
    }
    pub fn top(&self) -> i32 {
        self.center.y - self.dy
    }
    pub fn bottom(&self) -> i32 {
        self.center.y + self.dy
    }
}
