use crate::utils::FloatType;

#[derive(Debug)]
pub struct RalgoParams {
    pub alpha: FloatType,
    pub q1: FloatType,
    pub epsx: FloatType,
    pub epsg: FloatType,
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
    pub fn with_alpha(&self, alpha: FloatType) -> Self {
        RalgoParams { alpha, ..*self }
    }

    pub fn with_q1(&self, q1: FloatType) -> Self {
        RalgoParams { q1, ..*self }
    }

    pub fn with_max_iterations(&self, max_iterations: usize) -> Self {
        RalgoParams {
            max_iterations,
            ..*self
        }
    }

    pub fn with_epsx(&self, epsx: FloatType) -> Self {
        RalgoParams {
            epsx,
            ..*self
        }
    }

    pub fn with_epsg(&self, epsg: FloatType) -> Self {
        RalgoParams {
            epsg,
            ..*self
        }
    }
}
