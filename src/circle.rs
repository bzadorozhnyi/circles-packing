use crate::point::{self, Point};

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub center: Option<point::Point>,
    pub radius: f32,
}

impl Circle {
    pub fn empty() -> Self {
        Circle {
            center: None,
            radius: 0.0,
        }
    }

    pub fn new(radius: f32, center: Point) -> Self {
        Circle {
            center: Some(center),
            radius,
        }
    }

    pub fn new_with_radius(radius: f32) -> Self {
        Circle {
            center: None,
            radius,
        }
    }

    pub fn overlap(&self, other: &Circle) -> bool {
        if let (Some(center_self), Some(center_other)) =
            (self.center.as_ref(), other.center.as_ref())
        {
            let distance = ((center_self.x - center_other.x).powi(2)
                + (center_self.y - center_other.y).powi(2))
            .sqrt();
            let radius_sum = self.radius + other.radius;

            return distance <= radius_sum;
        } else {
            false
        }
    }

    pub fn is_overlap(&self, circles: &Vec<Circle>) -> bool {
        for circle in circles {
            if self.overlap(circle) {
                return true;
            }
        }
        false
    }

    pub fn is_inside_main_circle(&self, main_circle_radius: f32) -> bool {
        if let Some(center) = self.center.as_ref() {
            let distance = (center.x.powi(2) + center.y.powi(2)).sqrt();
            return distance <= (main_circle_radius - self.radius);
        } else {
            false
        }
    }
}
