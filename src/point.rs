#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn empty() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}