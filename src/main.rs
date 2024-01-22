use config::load_config;
use modes::{input_mode::input_mode, bench_mode::bench_mode};
mod algorithm;
mod modes;
mod config;
fn main() {
    let config = load_config();
    if config.input_mode {
        return input_mode(&config);
    }
    bench_mode(&config);
}
