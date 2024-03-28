use crate::{
    circle::Circle,
    packing::is_valid_pack,
    point::Point,
    ralgo::{dichotomy_step_ralgo::dichotomy_step_ralgo, ralgo_params::RalgoParams},
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::
    sync::{Arc, Mutex}
;

fn generate_random_arrangement(
    main_circle_radius: f64,
    rng: &Arc<Mutex<StdRng>>,
    radiuses: &Vec<f64>,
) -> (Vec<Circle>, f64) {
    let mut circles = vec![];
    for i in 0..radiuses.len() {
        let mut rng = rng.lock().unwrap();

        let (mut x, mut y);
        loop {
            (x, y) = (
                rng.gen_range(-main_circle_radius..=main_circle_radius),
                rng.gen_range(-main_circle_radius..=main_circle_radius),
            );

            if x.powi(2) + y.powi(2) <= main_circle_radius.powi(2) {
                break;
            }
        }

        circles.push(Circle::new(radiuses[i], Point { x, y }))
    }

    let mut r = f64::MAX;

    for i in 0..circles.len() {
        let center_i = circles[i].center.unwrap();

        for j in (i + 1)..circles.len() {
            let center_j = circles[j].center.unwrap();

            r = r.min(
                (center_i.x - center_j.x).powi(2)
                    + (center_i.y.powi(2) - center_j.y.powi(2)).sqrt() / 2.0,
            );
        }
    }

    return (circles, r);
}

fn get_updated_main_cirlce_radius(circles: &Vec<Circle>, r: f64) -> f64 {
    return circles
        .iter()
        .map(|c| (c.center.unwrap().x.powi(2) + c.center.unwrap().y.powi(2)).sqrt() + r)
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap();
}

pub fn random_single_case_console(
    test_number: u32,
    launches: usize,
    algorithm_params: &[(bool, f64)],
    alpha_q1_pairs: &Vec<(f64, f64)>,
) -> (f64, Vec<Circle>) {
    let rng = Arc::new(Mutex::new(StdRng::seed_from_u64(0)));
    let radiuses = (1..=test_number).map(|x| x as f64).collect::<Vec<_>>();
    let gen_main_circle_radius: f64 = radiuses.iter().map(|r| r.powi(2)).sum::<f64>().sqrt() * 1.2;

    let answer_main_circle_radius = Arc::new(Mutex::new(f64::MAX));
    let answer_circles = Arc::new(Mutex::new(Vec::<Circle>::new()));

    for (alpha, q1) in alpha_q1_pairs {
        for (reset_step, eps) in algorithm_params {
            let ralgo_params = RalgoParams::default()
                .with_alpha(*alpha)
                .with_q1(*q1)
                .with_max_iterations(100_000);
            // println!("Generate with ralgo params = {ralgo_params:?}");

            (1..=launches).into_par_iter().for_each(|launch| {
                // println!("Launch: {launch}");

                let rng = Arc::clone(&rng);

                let (circles, r) =
                    generate_random_arrangement(gen_main_circle_radius, &rng, &radiuses);
                let updated_main_circle_radius = get_updated_main_cirlce_radius(&circles, r);

                // get result of dichotomy algorithm
                let (new_main_circle_radius, new_circles) = dichotomy_step_ralgo(
                    updated_main_circle_radius,
                    &circles,
                    *reset_step,
                    *eps,
                    &ralgo_params,
                );

                let mut answer_main_circle_radius = answer_main_circle_radius.lock().unwrap();

                if is_valid_pack(new_main_circle_radius, &new_circles)
                    && new_main_circle_radius < *answer_main_circle_radius
                {
                    *answer_main_circle_radius = new_main_circle_radius;
                    *answer_circles.lock().unwrap() = new_circles;
                }
            });
        }
    }

    // (0.0, Vec::<Circle>::new())
    let answer = (*answer_main_circle_radius.lock().unwrap(), answer_circles.lock().unwrap().clone());

    answer
}
