use std::io;

use crate::compute_score::compute_score;

mod compute_score;
mod protein;
mod proteins_map;

fn main() {
    // 1. Read first line of input
    let mut user_input = String::new();
    let stdin = io::stdin();
    if stdin.read_line(&mut user_input).is_err() {
        panic!("Failed to read input from user")
    }
    user_input = user_input.trim().to_string();
    let (best_score, number_of_branches) = compute_score(user_input);
    println!(
        "Energy of most optimal computed conformation is {:?}, found with {:?} branches",
        best_score, number_of_branches
    );
}
