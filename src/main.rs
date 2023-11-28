use crate::utils::measure_time;

mod calc_all_cases;
mod circle;
mod packing;
mod plot;
mod point;
mod ralgo;
mod utils;

fn main() {
    let ralgo_params = [
        (false, 1e-2),
        (true, 1e-2),
        (false, 1e-3),
        (true, 1e-3),
        (false, 1e-4),
        (true, 1e-4),
        (false, 1e-5),
        (true, 1e-5),
        (false, 0.0),
        (true, 0.0),
    ];

    let (total_time, _) = measure_time(|| calc_all_cases::calc_all_cases(&ralgo_params));

    println!("Total time = {}", total_time);
}
