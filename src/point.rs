use crate::utils::FloatType;

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: FloatType,
    pub y: FloatType,
}

impl Point {
    pub fn empty() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}