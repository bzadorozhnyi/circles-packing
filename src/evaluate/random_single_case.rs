use super::utils::{calculate_points, get_input_data, get_jury_answer};
use crate::{
    circle::Circle, evaluate::utils::write_row_block, packing, point::Point,
    ralgo::dichotomy_step_ralgo, utils::measure_time,
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rust_xlsxwriter::{Format, Workbook};
use std::{
    io::{self},
    sync::{Arc, Mutex},
};

fn get_table_headings(params: &[(bool, f32)]) -> Vec<String> {
    let mut headings: Vec<String> = vec!["Launch".to_string(), "R".to_string(), "r".to_string()];
    let heading_names = ["R", "Points", "Is valid?", "Time"];
    for (reset, eps) in params {
        let reset_str = if *reset { "P" } else { "B" };
        for i in 0..4 {
            headings.push(format!("{} {} EPS={}", &heading_names[i], reset_str, eps));
        }
    }

    return headings;
}

fn generate_random_arrangement(
    main_circle_radius: f32,
    rng: &Arc<Mutex<StdRng>>,
    radiuses: &Vec<f32>,
) -> (Vec<Circle>, f32) {
    let mut circles = vec![];
    for i in 0..radiuses.len() {
        let mut rng = rng.lock().unwrap();

        circles.push(Circle::new(
            radiuses[i],
            Point {
                x: rng.gen_range(-main_circle_radius..=main_circle_radius),
                y: rng.gen_range(-main_circle_radius..=main_circle_radius),
            },
        ))
    }

    let mut r = f32::MAX;

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

fn get_updated_main_cirlce_radius(circles: &Vec<Circle>, r: f32) -> f32 {
    return circles
        .iter()
        .map(|c| (c.center.unwrap().x.powi(2) + c.center.unwrap().y.powi(2)).sqrt() + r)
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap();
}

pub fn random_single_case(
    test_number: usize,
    launches: usize,
    ralgo_params: &[(bool, f32)],
) -> io::Result<()> {
    let rng = Arc::new(Mutex::new(StdRng::seed_from_u64(0)));

    let mut workbook: Workbook = Workbook::new();
    let worksheet = Arc::new(Mutex::new(workbook.add_worksheet()));
    let cell_format = Format::new().set_align(rust_xlsxwriter::FormatAlign::Center);

    // setup headings
    for (col, data) in get_table_headings(ralgo_params).iter().enumerate() {
        worksheet
            .lock()
            .unwrap()
            .write_with_format(0, col as u16, data, &cell_format)
            .ok();
    }

    // get input data
    let (_, radiuses) = get_input_data(test_number as u32);

    // get jury answer of current test
    let jury_answer = get_jury_answer(test_number as u32);

    let main_circle_radius: f32 = radiuses.iter().map(|r| r.powi(2)).sum::<f32>().sqrt();

    (1..=launches).into_par_iter().for_each(|launch| {
        println!("Launch: {launch}");

        let worksheet = Arc::clone(&worksheet);
        let rng = Arc::clone(&rng);

        worksheet
            .lock()
            .unwrap()
            .write_with_format(launch as u32, 0, launch as u32, &cell_format)
            .ok();

        let (circles, r) = generate_random_arrangement(main_circle_radius, &rng, &radiuses);
        let updated_main_circle_radius = get_updated_main_cirlce_radius(&circles, r);

        worksheet
            .lock()
            .unwrap()
            .write_with_format(launch as u32, 1, updated_main_circle_radius, &cell_format)
            .ok();

        worksheet
            .lock()
            .unwrap()
            .write_with_format(launch as u32, 2, r, &cell_format)
            .ok();

        for (index, (reset_step, eps)) in ralgo_params.iter().enumerate() {
            // get result of dichotomy algorithm
            let (ralgo_time, (new_main_circle_radius, new_circles)) = measure_time(|| {
                dichotomy_step_ralgo(updated_main_circle_radius, &circles, *reset_step, *eps)
            });
            let points = calculate_points(new_main_circle_radius, jury_answer);

            // write dichotomy results into table
            write_row_block(
                &worksheet,
                launch as u32,
                (index * 4 + 3) as u16,
                new_main_circle_radius,
                packing::is_valid_pack(new_main_circle_radius, &new_circles),
                points,
                ralgo_time,
                &cell_format,
            );
        }
    });
    worksheet.lock().unwrap().autofit();

    workbook
        .save(format!(
            "./results/random/random-single-result-test-{test_number}.xlsx"
        ))
        .ok();

    Ok(())
}
