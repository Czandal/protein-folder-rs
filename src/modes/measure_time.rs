use std::time::{Duration, Instant};

pub fn measure_time<T, F>(func: F) -> (Duration, T)
where
    F: FnOnce() -> T,
{
    let start_time = Instant::now();
    let result = func();
    let elapsed_time = start_time.elapsed();
    (elapsed_time, result)
}
