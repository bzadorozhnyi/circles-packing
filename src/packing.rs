use nalgebra::{self};
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::cmp::min;

use crate::circle::*;
use crate::point::Point;

fn get_rotated_point(y_coord: f32, angle: f32) -> Point {
    let angle = angle.to_radians();
    return Point {
        x: y_coord * angle.sin(),
        y: y_coord * angle.cos(),
    };
}

fn center_of_small_circle_touch_main(
    prev_circle: &Circle,
    small_circle: &Circle,
    main_circle_radius: f32,
) -> Option<(Point, Point)> {
    if let (Some(center_prev), Some(_)) =
        (prev_circle.center.as_ref(), small_circle.center.as_ref())
    {
        let e: f32 = center_prev.x.powi(2)
            + center_prev.y.powi(2)
            + (main_circle_radius - small_circle.radius).powi(2)
            - (small_circle.radius + prev_circle.radius + 0.1).powi(2);

        let a: f32 = 4.0 * (center_prev.x.powi(2) + center_prev.y.powi(2));
        let b: f32 = -4.0 * center_prev.x * e;
        let c: f32 = e.powi(2)
            - ((2.0 * center_prev.y).powi(2))
                * ((main_circle_radius - small_circle.radius).powi(2));

        let d: f32 = b.powi(2) - 4.0 * a * c;
        if d <= 0.0 || a == 0.0 {
            return None;
        }

        let x1: f32 = (-b + d.sqrt()) / (2.0 * a);
        let x2: f32 = (-b - d.sqrt()) / (2.0 * a);

        let p1: Point = Point {
            x: x1,
            y: (e - 2.0 * center_prev.x * x1) / (2.0 * center_prev.y),
        };

        let p2: Point = Point {
            x: x2,
            y: (e - 2.0 * center_prev.x * x2) / (2.0 * center_prev.y),
        };

        return Some((p1, p2));
    } else {
        return None;
    }
}

fn closest_center_to_two_touching_circles(
    start_point: Point,
    c1: &Circle,
    c2: &Circle,
    c3: &Circle,
) -> Option<Point> {
    if let (Some(c1_center), Some(c2_center)) = (c1.center.as_ref(), c2.center.as_ref()) {
        if (c1_center.x - c2_center.x).powi(2) + (c1_center.y - c2_center.y).powi(2)
            > (2.0 * c3.radius + c1.radius + c2.radius).powi(2)
        {
            return None;
        }

        let delta: f32 = 1e-6;

        let mut a: nalgebra::Vector2<f32> = nalgebra::Vector2::new(start_point.x, start_point.y);

        for _ in 0..10 {
            let j = nalgebra::Matrix2::new(
                -2.0 * c1_center.x + 2.0 * a[0],
                -2.0 * c1_center.y + 2.0 * a[1],
                -2.0 * c2_center.x + 2.0 * a[0],
                -2.0 * c2_center.y + 2.0 * a[1],
            );

            let r: nalgebra::Vector2<f32> = nalgebra::Vector2::new(
                (c1_center.x - a[0]).powi(2) + (c1_center.y - a[1]).powi(2)
                    - (c1.radius + c3.radius + 0.01).powi(2),
                (c2_center.x - a[0]).powi(2) + (c2_center.y - a[1]).powi(2)
                    - (c2.radius + c3.radius + 0.01).powi(2),
            );

            let prev_a: nalgebra::Vector2<f32> = a.clone();

            if j.determinant().abs() < 1e-6_f32 {
                return None;
            }

            a -= j.try_inverse().expect("Square matrix") * r;

            if (a - prev_a).abs().iter().all(|&value| value < delta) {
                break;
            }
        }
        Some(Point { x: a[0], y: a[1] })
    } else {
        None
    }
}

// fn bad_angle(c1: &Circle, c2: &Circle, main_circle_radius: f32) -> f32 {
//     return (c1.radius / (main_circle_radius - c1.radius)).atan()
//         + (c2.radius / (main_circle_radius - c2.radius)).atan();
// }
fn bad_angle(r1: f32, r2: f32, main_circle_radius: f32) -> f32 {
    return (r1 / (main_circle_radius - r1)).atan()
        + (r2 / (main_circle_radius - r2)).atan();
}

fn pack_circles(radiuses: &Vec<f32>, main_circle_radius: f32) -> Option<Vec<Circle>> {
    let mut circles: Vec<Circle> = radiuses
        .iter()
        .map(|&radius| Circle::new_with_radius(radius))
        .collect();

    circles[0].center = Some(Point {
        x: 0.0,
        y: main_circle_radius - circles[0].radius,
    });

    let mut order_of_circles_placement: Vec<usize> = vec![0];

    for index in 0..circles.len() {
        if circles[index].center.is_some() {
            continue;
        }

        let (mut left, mut right, mut angle_for_new_circle) = (
            0_f32,
            // 360_f32 - bad_angle(&circles[0], &circles[index], main_circle_radius),
            360_f32 - bad_angle(circles[0].radius, circles[index].radius, main_circle_radius),
            -1_f32,
        );
        for _ in 0..50 {
            let angle = (left + right) / 2.0;
            let new_circle: Circle = Circle {
                center: Some(get_rotated_point(
                    main_circle_radius - circles[index].radius,
                    angle,
                )),
                radius: circles[index].radius,
            };

            if !new_circle.is_overlap(&circles) {
                right = angle;
                angle_for_new_circle = angle;
            } else {
                left = angle;
            }
        }

        if angle_for_new_circle >= 0.0 {
            circles[index] = Circle {
                radius: circles[index].radius,
                center: Some(get_rotated_point(
                    main_circle_radius - circles[index].radius,
                    angle_for_new_circle,
                )),
            };
            order_of_circles_placement.push(index);
        }
    }

    for index in &order_of_circles_placement {
        for i in 0..circles.len() {
            if circles[i].center.is_some() {
                continue;
            }

            if let Some(points) =
                center_of_small_circle_touch_main(&circles[*index], &circles[i], main_circle_radius)
            {
                for point in [points.0, points.1] {
                    let new_circle: Circle = Circle {
                        radius: circles[i].radius,
                        center: Some(point),
                    };

                    if new_circle.is_inside_main_circle(main_circle_radius)
                        && !new_circle.is_overlap(&circles)
                    {
                        circles[i] = new_circle;
                        break;
                    }
                }
            }

            if circles[i].center.is_some() {
                break;
            }
        }
    }

    while !order_of_circles_placement.is_empty() {
        let mut new_order_of_circles_placement: Vec<usize> = Vec::new();
        for index in 0..order_of_circles_placement.len() {
            for i in 0..circles.len() {
                if circles[i].center.is_some() {
                    continue;
                }

                for j in 1..=min(2_usize, order_of_circles_placement.len()) {
                    let new_circle_center: Option<Point> = closest_center_to_two_touching_circles(
                        Point::empty(),
                        &circles[order_of_circles_placement[index]],
                        &circles[order_of_circles_placement
                            [(index + j) % order_of_circles_placement.len()]],
                        &circles[i],
                    );

                    if new_circle_center.is_none() {
                        continue;
                    }

                    let new_circle: Circle = Circle {
                        center: new_circle_center,
                        radius: circles[i].radius,
                    };

                    if new_circle.is_inside_main_circle(main_circle_radius)
                        && !new_circle.is_overlap(&circles)
                    {
                        circles[i] = new_circle;
                        new_order_of_circles_placement.push(i);
                        break;
                    }
                }

                if circles[i].center.is_some() {
                    break;
                }
            }
        }
        order_of_circles_placement = new_order_of_circles_placement;
    }

    for circle in &circles {
        if circle.center.is_none() {
            return None;
        }
    }

    return Some(circles);
}

pub fn is_valid_pack(main_circle_radius: f32, circles: &Vec<Circle>) -> bool {
    for circle in circles {
        if !circle.is_inside_main_circle(main_circle_radius) {
            return false;
        }
    }

    for i in 0..circles.len() {
        for j in i + 1..circles.len() {
            if circles[i].overlap(&circles[j]) {
                return false;
            }
        }
    }

    true
}

pub fn find_answer(radiuses: &mut Vec<f32>, number_of_iterations: u32) -> (f32, Vec<Circle>) {
    let number_of_circles: usize = radiuses.len();
    let mut main_circle_radius: f32 = (radiuses.iter().sum::<f32>() as f32).ceil();

    let mut answer: Vec<Circle> = (0..number_of_circles).map(|_| Circle::empty()).collect();
    let mut new_circles: Vec<Circle> = Vec::new();

    let mut rng = StdRng::seed_from_u64(0);

    for _ in 0..number_of_iterations {
        let (mut left, mut right) = (0_f32, main_circle_radius);

        while (right - left).abs() >= 1e-4 {
            let middle = (left + right) / 2.0;

            if let Some(circles) = pack_circles(&radiuses, middle) {
                right = middle;
                new_circles = circles.clone();
            } else {
                left = middle;
            }
        }

        if new_circles.iter().all(|circle| circle.center.is_some()) {
            let new_main_circle_radius: f32 = 1.001 * right;

            if new_main_circle_radius < main_circle_radius
                && is_valid_pack(new_main_circle_radius, &new_circles)
            {
                main_circle_radius = new_main_circle_radius;
                answer = new_circles.clone();
            }
        }

        radiuses.swap(
            rng.gen_range(0..number_of_circles),
            rng.gen_range(0..number_of_circles),
        );
    }

    return (main_circle_radius, answer);
}
