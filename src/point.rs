#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn empty() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}