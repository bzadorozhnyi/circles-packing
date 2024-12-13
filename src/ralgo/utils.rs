use nalgebra::DVector;

use crate::{circle, point, utils::FloatType};

pub fn concat_gradients(
    gx: &DVector<FloatType>,
    gy: &DVector<FloatType>,
    gr: FloatType,
) -> DVector<FloatType> {
    let circles_number = gx.len();

    let mut gradient = DVector::<FloatType>::zeros(2 * circles_number + 1);
    gradient.rows_mut(0, circles_number).copy_from(&gx);
    gradient
        .rows_mut(circles_number, circles_number)
        .copy_from(&gy);
    gradient[2 * circles_number] = gr;

    return gradient;
}

pub fn circles_to_dvector(
    circles: &Vec<circle::Circle>,
    main_circle_radiuse: FloatType,
) -> DVector<FloatType> {
    let data: Vec<FloatType> = ([
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
    return DVector::from_vec(data);
}

pub fn dvector_to_answer(
    x: &DVector<FloatType>,
    circles_radiuses: &DVector<FloatType>,
) -> (FloatType, Vec<circle::Circle>) {
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

pub fn get_last(d: &DVector<FloatType>) -> FloatType {
    return d[d.len() - 1];
}
