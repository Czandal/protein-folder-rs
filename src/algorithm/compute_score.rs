use std::cell::Cell;

use rand::Rng;

use super::{protein::Protein, protein_map::ProteinMap};

fn expand<InsertAuthority>(
    current_conformations: Vec<ProteinMap>,
    next_monomer: bool,
    highest_score: i32,
    mut should_insert: InsertAuthority,
) -> (Vec<ProteinMap>, i32)
where
    InsertAuthority: FnMut(i32, i32, i32, i32) -> bool,
{
    let mut conformations = Vec::with_capacity(current_conformations.len());
    // if next monomer is polar, so it does not impact energy of the structure
    if !next_monomer {
        // then append every conformation in every direction possible
        for conformation in current_conformations {
            let tail = conformation
                .points
                .last()
                .unwrap_or_else(|| panic!("Provided empty map as starting point"));
            // append to the left
            let mut left_conformation = conformation.clone();
            if left_conformation
                .push(Protein::new_on_left(&tail, false))
                .is_some()
            {
                conformations.push(left_conformation);
            }
            // append to the right
            let mut right_conformation = conformation.clone();
            if right_conformation
                .push(Protein::new_on_right(&tail, false))
                .is_some()
            {
                conformations.push(right_conformation);
            }
            // append up
            let mut up_conformation = conformation.clone();
            if up_conformation
                .push(Protein::new_on_up(&tail, false))
                .is_some()
            {
                conformations.push(up_conformation);
            }
            // append down
            let mut down_conformation = conformation.clone();
            if down_conformation
                .push(Protein::new_on_down(&tail, false))
                .is_some()
            {
                conformations.push(down_conformation);
            }
        }
        return (conformations, highest_score);
    }

    let mut sum_of_score_so_far: i32 = 0;
    let mut analyzed_conformations = 0;
    let mut highest_score_so_far = highest_score;

    for conformation in current_conformations {
        let tail = conformation
            .points
            .last()
            .unwrap_or_else(|| panic!("Provided non conformation map as starting point"));
        // append to the left
        let mut left_conformation = conformation.clone();
        if left_conformation
            .push(Protein::new_on_left(&tail, true))
            .is_some()
        {
            sum_of_score_so_far += left_conformation.score.get();
            analyzed_conformations += 1;
            if should_insert(
                left_conformation.score.get(),
                highest_score_so_far,
                sum_of_score_so_far,
                analyzed_conformations,
            ) {
                highest_score_so_far = highest_score_so_far.max(left_conformation.score.get());
                conformations.push(left_conformation);
            }
        }
        // append to the right
        let mut right_conformation = conformation.clone();
        if right_conformation
            .push(Protein::new_on_right(&tail, true))
            .is_some()
        {
            sum_of_score_so_far += right_conformation.score.get();
            analyzed_conformations += 1;
            if should_insert(
                right_conformation.score.get(),
                highest_score_so_far,
                sum_of_score_so_far,
                analyzed_conformations,
            ) {
                highest_score_so_far = highest_score_so_far.max(right_conformation.score.get());
                conformations.push(right_conformation);
            }
        }
        // append up
        let mut up_conformation = conformation.clone();
        if up_conformation
            .push(Protein::new_on_up(&tail, true))
            .is_some()
        {
            sum_of_score_so_far += up_conformation.score.get();
            analyzed_conformations += 1;
            if should_insert(
                up_conformation.score.get(),
                highest_score_so_far,
                sum_of_score_so_far,
                analyzed_conformations,
            ) {
                highest_score_so_far = highest_score_so_far.max(up_conformation.score.get());
                conformations.push(up_conformation);
            }
        }
        // append down
        let mut down_conformation = conformation.clone();
        if down_conformation
            .push(Protein::new_on_down(&tail, true))
            .is_some()
        {
            sum_of_score_so_far += down_conformation.score.get();
            analyzed_conformations += 1;
            if should_insert(
                down_conformation.score.get(),
                highest_score_so_far,
                sum_of_score_so_far,
                analyzed_conformations,
            ) {
                highest_score_so_far = highest_score_so_far.max(down_conformation.score.get());
                conformations.push(down_conformation);
            }
        }
    }

    (conformations, highest_score_so_far)
}

// returns (score, number of branches)
pub fn compute_score(monomers_string: &str, p1: f64, p2: f64) -> (i32, usize) {
    // 0. Handle edge case, which is monomers_string.len() < 2
    if monomers_string.len() < 2 {
        return (0, 1);
    }
    // 1. Create random generator
    let mut rng = rand::thread_rng();
    let mut monomers_iter = monomers_string
        .as_bytes()
        .into_iter()
        .map(|byte| *byte == 'H' as u8 || *byte == 'h' as u8);
    // 3. Compute conformations by iterating through monomers and adding one by one to possible
    //    conformations
    let mut possible_conformations: Vec<ProteinMap> = vec![ProteinMap {
        score: Cell::from(0),
        max_x: 0,
        min_x: 0,
        min_y: 0,
        max_y: 0,
        points: vec![
            // NOTE: Any possible conformation of two monomers looks the same (is linear), so to
            // save power we hardcode them here
            Protein {
                x: 0,
                y: 0,
                hydrophobic: monomers_iter.next().unwrap(),
            },
            Protein {
                x: 0,
                y: 1,
                hydrophobic: monomers_iter.next().unwrap(),
            },
        ],
    }];
    let mut best_score: i32 = 0;

    for monomer in monomers_iter {
        (possible_conformations, best_score) = expand(
            possible_conformations,
            monomer,
            best_score,
            |score: i32, highest_score: i32, sum_of_scores: i32, analyzed_conformations: i32| {
                if score >= highest_score {
                    return true;
                }
                let random_var: f64 = rng.gen();
                // equivalent of score > sum_of_score_so_far/analyzed_conformations which is equivalent to
                // score > average score
                if score * analyzed_conformations >= sum_of_scores {
                    // reject with probability p1
                    return random_var > p1;
                }

                // reject with probability p2
                return random_var > p2;
            },
        );
    }
    (best_score, possible_conformations.len())
}
