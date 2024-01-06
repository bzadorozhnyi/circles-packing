use crate::{packing::find_answer, plot::draw_plot};

use super::utils::get_input_data;

pub fn heuristic_single_case(test_number: u32) {
    let (_, mut radiuses) = get_input_data(test_number);
    let (main_circle_radius, circles) = find_answer(&mut radiuses, 100);

    draw_plot(main_circle_radius, &circles);
}
