use std::time::Instant;

pub fn measure_time<F, T>(function: F) -> (f32, T)
where
    F: FnOnce() -> T,
{
    let start_time = Instant::now();
    let function_result = function();
    let end_time = Instant::now();

    let time = (end_time - start_time).as_secs_f32();

    return (time, function_result);
}
