use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::cmp::min;

use crate::circle::*;
use crate::point::Point;
use crate::utils::FloatType;

fn get_rotated_point(y_coord: FloatType, angle: FloatType) -> Point {
    return Point {
        x: y_coord * angle.sin(),
        y: y_coord * angle.cos(),
    };
}

fn center_of_small_circle_touch_main(
    prev_circle: &Circle,
    small_circle: &Circle,
    main_circle_radius: FloatType,
) -> Option<(Point, Point)> {
    if let (Some(center_prev), Some(_)) =
        (prev_circle.center.as_ref(), small_circle.center.as_ref())
    {
        let e: FloatType = center_prev.x.powi(2)
            + center_prev.y.powi(2)
            + (main_circle_radius - small_circle.radius).powi(2)
            - (small_circle.radius + prev_circle.radius + 0.1).powi(2);

        let a: FloatType = 4.0 * (center_prev.x.powi(2) + center_prev.y.powi(2));
        let b: FloatType = -4.0 * center_prev.x * e;
        let c: FloatType = e.powi(2)
            - ((2.0 * center_prev.y).powi(2))
                * ((main_circle_radius - small_circle.radius).powi(2));

        let d: FloatType = b.powi(2) - 4.0 * a * c;
        if d <= 0.0 || a == 0.0 {
            return None;
        }

        let x1: FloatType = (-b + d.sqrt()) / (2.0 * a);
        let x2: FloatType = (-b - d.sqrt()) / (2.0 * a);

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

pub fn closest_center_to_two_touching_circles(
    c1: &Circle,
    c2: &Circle,
    r3: FloatType,
) -> Option<Point> {
    if let (Some(c1_center), Some(c2_center)) = (c1.center.as_ref(), c2.center.as_ref()) {
        if (c1_center.x - c2_center.x).powi(2) + (c1_center.y - c2_center.y).powi(2)
            > (2.0 * r3 + c1.radius + c2.radius).powi(2)
        {
            return None;
        }

        let (x1, y1, r1) = (c1_center.x, c1_center.y, c1.radius);
        let (x2, y2, r2) = (c2_center.x, c2_center.y, c2.radius);

        let delta: FloatType = 1e-6;
        let omega = 1e-6;

        if (y1 - y2).abs() < delta {
            if (x1 - x2).abs() < delta {
                return None;
            }

            let x_3 = ((x1.powi(2) - x2.powi(2)) - (r1 + r3).powi(2) + (r2 + r3).powi(2))
                / (2.0 * (x1 - x2));

            let y_3 = ((x1 - x_3).powi(2) - (r1 + r3).powi(2)).sqrt() + y1;

            return Some(Point { x: x_3, y: y_3 });
        }

        let t = x1.powi(2) - x2.powi(2) + y1.powi(2) - y2.powi(2) - (r1 + r3 + omega).powi(2)
            + (r2 + r3 + omega).powi(2);
        let p = 2.0 * (y1 - y2);

        let a = p.powi(2) + 4.0 * (x1 - x2).powi(2);

        let b = -2.0 * x1 * p.powi(2) + 4.0 * (y1 * p - t) * (x1 - x2);
        let c = p.powi(2) * (x1.powi(2) + y1.powi(2) - (r1 + r3 + omega).powi(2))
            - 2.0 * y1 * p * t
            + t.powi(2);

        let d = b.powi(2) - 4.0 * a * c;

        if d < delta {
            return None;
        }

        let sqrt_d = d.sqrt();
        let x3_1 = (-b - sqrt_d) / (2.0 * a);
        let x3_2 = (-b + sqrt_d) / (2.0 * a);

        let y3_1 = (t - 2.0 * (x1 - x2) * x3_1) / p;
        let y3_2 = (t - 2.0 * (x1 - x2) * x3_2) / p;

        let c3_center = if x3_1.powi(2) + y3_1.powi(2) < x3_2.powi(2) + y3_2.powi(2) {
            Point { x: x3_1, y: y3_1 }
        } else {
            Point { x: x3_2, y: y3_2 }
        };

        return Some(c3_center);
    } else {
        None
    }
}

fn extra_angle(r1: FloatType, r2: FloatType, main_circle_radius: FloatType) -> FloatType {
    return (r1 / (main_circle_radius - r1)).asin() + (r2 / (main_circle_radius - r2)).asin();
}

fn pack_circles(radiuses: &Vec<FloatType>, main_circle_radius: FloatType) -> Option<Vec<Circle>> {
    let mut circles: Vec<Circle> = radiuses
        .iter()
        .map(|&radius| Circle::with_radius(radius))
        .collect();

    circles[0].center = Some(Point {
        x: 0.0,
        y: main_circle_radius - circles[0].radius,
    });

    let mut level_of_placed_circle_indices: Vec<usize> = vec![0];
    let mut prev_circle_angle: FloatType = 0.0;

    for index in 1..circles.len() {
        let approximate_angle = prev_circle_angle
            + extra_angle(
                circles[level_of_placed_circle_indices[level_of_placed_circle_indices.len() - 1]]
                    .radius,
                circles[index].radius,
                main_circle_radius,
            );

        let (mut left, mut right) = (0.99 * approximate_angle, 1.01 * approximate_angle);
        let mut new_circle_angle = -1.0;
        while (right - left) >= 1e-4 {
            let angle = (left + right) / 2.0;

            let new_circle = Circle {
                center: Some(get_rotated_point(
                    main_circle_radius - circles[index].radius,
                    angle,
                )),
                radius: circles[index].radius,
            };

            match !new_circle.is_overlap(&circles) {
                true => {
                    right = angle;
                    new_circle_angle = angle;
                }
                false => left = angle,
            }
        }

        if new_circle_angle >= 0.0 {
            circles[index] = Circle {
                center: Some(get_rotated_point(
                    main_circle_radius - circles[index].radius,
                    new_circle_angle,
                )),
                radius: circles[index].radius,
            };
            level_of_placed_circle_indices.push(index);

            prev_circle_angle = new_circle_angle;
        }
    }

    for placed_circle_index in &level_of_placed_circle_indices {
        'circles_loop: for i in 0..circles.len() {
            if circles[i].center.is_some() {
                continue;
            }

            if let Some(points) = center_of_small_circle_touch_main(
                &circles[*placed_circle_index],
                &circles[i],
                main_circle_radius,
            ) {
                for point in [points.0, points.1] {
                    let new_circle: Circle = Circle {
                        radius: circles[i].radius,
                        center: Some(point),
                    };

                    if new_circle.is_inside_main_circle(main_circle_radius)
                        && !new_circle.is_overlap(&circles)
                    {
                        circles[i] = new_circle;
                        break 'circles_loop;
                    }
                }
            }
        }
    }

    let cycle_index = |vector: &Vec<usize>, index: usize| -> usize { vector[index % vector.len()] };

    while !level_of_placed_circle_indices.is_empty() {
        let mut new_level_of_placed_circle_indices: Vec<usize> = Vec::new();
        for placed_circle_index in 0..level_of_placed_circle_indices.len() {
            'circles_loop: for i in 0..circles.len() {
                if circles[i].center.is_some() {
                    continue;
                }

                for shift in 1..=min(2_usize, level_of_placed_circle_indices.len()) {
                    let new_circle_center: Option<Point> = closest_center_to_two_touching_circles(
                        &circles[cycle_index(&level_of_placed_circle_indices, placed_circle_index)],
                        &circles[cycle_index(
                            &level_of_placed_circle_indices,
                            placed_circle_index + shift,
                        )],
                        circles[i].radius,
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
                        new_level_of_placed_circle_indices.push(i);
                        break 'circles_loop;
                    }
                }
            }
        }
        level_of_placed_circle_indices = new_level_of_placed_circle_indices;
    }

    if circles.iter().any(|circle| circle.center.is_none()) {
        return None;
    }

    Some(circles)
}

pub fn is_valid_pack(main_circle_radius: FloatType, circles: &Vec<Circle>) -> bool {
    if circles
        .iter()
        .any(|circle| !circle.is_inside_main_circle(main_circle_radius))
    {
        return false;
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

pub fn find_answer(
    radiuses: &mut Vec<FloatType>,
    number_of_iterations: u32,
) -> (FloatType, Vec<Circle>) {
    let number_of_circles: usize = radiuses.len();
    let mut main_circle_radius: FloatType =
        (radiuses.iter().sum::<FloatType>() as FloatType).ceil();

    let mut answer: Vec<Circle> = (0..number_of_circles).map(|_| Circle::empty()).collect();
    let mut new_circles: Vec<Circle> = Vec::new();

    let mut rng = StdRng::seed_from_u64(0);

    for _ in 0..number_of_iterations {
        let (mut left, mut right) = (0 as FloatType, main_circle_radius);

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
            let new_main_circle_radius: FloatType = 1.001 * right;

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
