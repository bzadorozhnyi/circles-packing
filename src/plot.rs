use crate::circle;
use crate::point;
use plotters::prelude::*;

pub fn draw_plot(main_circle_radius: f32, circles: &Vec<circle::Circle>) {
    let plot_size: i32 = 1000;
    let root =
        BitMapBackend::new("circle.png", (plot_size as u32, plot_size as u32)).into_drawing_area();
    root.fill(&WHITE).ok();

    let spec_size = main_circle_radius * 1.2;

    let mut chart = ChartBuilder::on(&root)
        .set_all_label_area_size(85)
        .build_cartesian_2d(-spec_size..spec_size, -spec_size..spec_size)
        .unwrap();

    chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .label_style(TextStyle::from(("bebas neue", 30)))
        .draw()
        .ok();

    let root = chart.plotting_area();

    let convert_radius = |radius: f32| {
        return (radius * (plot_size - 2 * 85) as f32) / (2.0 * spec_size);
    };

    let set_circle = |c: &circle::Circle| {
        return EmptyElement::at((c.center.unwrap().x, c.center.unwrap().y))
            + Circle::new(
                (0, 0),
                convert_radius(c.radius),
                ShapeStyle {
                    color: BLUE.mix(0.6),
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
