use crate::circle::Circle;

pub struct RalgoResult {
    pub ralgo_calls: u32,
    pub iterations: u32,
    pub calcfg_calls: u32,
    pub main_circle_radius: f32,
    pub circles: Vec<Circle>,
}

impl RalgoResult {
    pub fn new(
        ralgo_calls: u32,
        iterations: u32,
        calcfg_calls: u32,
        main_circle_radius: f32,
        circles: Vec<Circle>,
    ) -> Self {
        RalgoResult {
            ralgo_calls,
            iterations,
            calcfg_calls,
            main_circle_radius,
            circles,
        }
    }
}
