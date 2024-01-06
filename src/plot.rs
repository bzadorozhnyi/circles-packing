use crate::circle;
use crate::point;
use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;

pub fn draw_plot(main_circle_radius: f32, circles: &Vec<circle::Circle>) {
    let plot_size: i32 = 1000;
    let root =
        BitMapBackend::new("circle.png", (plot_size as u32, plot_size as u32)).into_drawing_area();
    root.fill(&WHITE).ok();

    let root = root.apply_coord_spec(Cartesian2d::<RangedCoordf32, RangedCoordf32>::new(
        -main_circle_radius..main_circle_radius,
        main_circle_radius..-main_circle_radius,
        (0..plot_size, 0..plot_size),
    ));

    let convert_radius = |radius: f32| {
        return (radius * plot_size as f32) / (2.0 * main_circle_radius);
    };

    let set_circle = |c: &circle::Circle| {
        return EmptyElement::at((c.center.unwrap().x, c.center.unwrap().y))
            + Circle::new(
                (0, 0),
                convert_radius(c.radius),
                ShapeStyle {
                    color: BLUE.mix(1.0),
                    filled: false,
                    stroke_width: 2,
                },
            );
    };

    let main_circle = circle::Circle::new(main_circle_radius, point::Point { x: 0.0, y: 0.0 });
    root.draw(&set_circle(&main_circle)).ok();
    circles.iter().for_each(|c| {
        root.draw(&set_circle(&c)).ok();
    });

    root.present().ok();
}
