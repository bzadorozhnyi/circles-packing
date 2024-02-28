#[derive(Debug)]
pub struct RalgoParams {
    pub alpha: f64,
    pub q1: f64,
    pub epsx: f64,
    pub epsg: f64,
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
    pub fn with_alpha(&self, alpha: f64) -> Self {
        RalgoParams { alpha, ..*self }
    }

    pub fn with_q1(&self, q1: f64) -> Self {
        RalgoParams { q1, ..*self }
    }

    pub fn with_max_iterations(&self, max_iterations: usize) -> Self {
        RalgoParams {
            max_iterations,
            ..*self
        }
    }

    pub fn with_epsx(&self, epsx: f64) -> Self {
        RalgoParams {
            epsx,
            ..*self
        }
    }

    pub fn with_epsg(&self, epsg: f64) -> Self {
        RalgoParams {
            epsg,
            ..*self
        }
    }
}
