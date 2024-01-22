use std::io;

use crate::{algorithm::compute_score::compute_score, config::Config};

pub fn input_mode(config: &Config) {
    let mut user_input = String::new();
    let stdin = io::stdin();
    if stdin.read_line(&mut user_input).is_err() {
        panic!("Failed to read input from user");
    }
    user_input = user_input.trim().to_string();
    let (best_score, number_of_branches) = compute_score(user_input.as_str(), config.p1, config.p2);
    println!(
        "Energy of most optimal computed conformation is {:?}, found with {:?} branches",
        best_score, number_of_branches
    );
}
