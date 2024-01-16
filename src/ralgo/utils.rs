use crate::{circle, point};

pub fn concat_gradients(
    gx: &nalgebra::DVector<f32>,
    gy: &nalgebra::DVector<f32>,
    gr: f32,
) -> nalgebra::DVector<f32> {
    let circles_number = gx.len();

    let mut gradient = nalgebra::DVector::<f32>::zeros(2 * circles_number + 1);
    gradient.rows_mut(0, circles_number).copy_from(&gx);
    gradient
        .rows_mut(circles_number, circles_number)
        .copy_from(&gy);
    gradient[2 * circles_number] = gr;

    return gradient;
}

pub fn circles_to_dvector(
    circles: &Vec<circle::Circle>,
    main_circle_radiuse: f32,
) -> nalgebra::DVector<f32> {
    let data: Vec<f32> = ([
        Vec::from_iter(
            circles
                .iter()
                .map(|c| c.center.as_ref().expect("Valid center").x),
        ),
        Vec::from_iter(
            circles
                .iter()
                .map(|c| c.center.as_ref().expect("Valid center").y),
        ),
        vec![main_circle_radiuse],
    ])
    .concat();
    return nalgebra::DVector::from_vec(data);
}

pub fn dvector_to_answer(
    x: &nalgebra::DVector<f32>,
    circles_radiuses: &nalgebra::DVector<f32>,
) -> (f32, Vec<circle::Circle>) {
    let main_circle_radiuse = x[x.len() - 1];
    let mut circles: Vec<circle::Circle> = Vec::from_iter(
        circles_radiuses
            .iter()
            .map(|radius| circle::Circle::with_radius(*radius)),
    );

    for i in 0..x.len() / 2 {
        circles[i].center = Some(point::Point {
            x: x[i],
            y: x[i + x.len() / 2],
        });
    }

    return (main_circle_radiuse, circles);
}

pub fn get_last(d: &nalgebra::DVector<f32>) -> f32 {
    return d[d.len() - 1];
}
