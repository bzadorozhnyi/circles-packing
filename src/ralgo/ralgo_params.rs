#[derive(Debug)]
pub struct RalgoParams {
    pub alpha: f32,
    pub q1: f32,
    pub epsx: f32,
    pub epsg: f32,
    pub max_iterations: usize,
}

impl Default for RalgoParams {
    fn default() -> Self {
        RalgoParams {
            alpha: 3.0,
            q1: 0.9,
            epsx: 1e-6,
            epsg: 1e-7,
            max_iterations: 3000,
        }
    }
}

impl RalgoParams {
    pub fn with_alpha(&self, alpha: f32) -> Self {
        RalgoParams { alpha, ..*self }
    }

    pub fn with_q1(&self, q1: f32) -> Self {
        RalgoParams { q1, ..*self }
    }

    pub fn with_max_iterations(&self, max_iterations: usize) -> Self {
        RalgoParams {
            max_iterations,
            ..*self
        }
    }
}
