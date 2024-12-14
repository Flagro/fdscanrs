use rayon::prelude::*;
use std::collections::HashSet;

pub fn brute_force_keys(data: Vec<Vec<String>>) -> Vec<Vec<usize>> {
    let num_cols: usize = data[0].len();
    let mut candidate_keys: Vec<Vec<usize>> = vec![];

    // Brute-force all subsets of columns
    (1..=num_cols).into_par_iter().for_each(|subset_size| {
        let combinations = (0..num_cols).combinations(subset_size);
        for combo: [usize] in combinations {
            if is_candidate_key(&data, &combo) {
                candidate_keys.push(combo);
            }
        }
    });

    candidate_keys
}

fn is_candidate_key(data: &[Vec<String>], columns: &[usize]) -> bool {
    let mut seen: HashSet<Vec<&str>> = HashSet::new();
    for row: &Vec<String> in data {
        let key: Vec<&str> = columns.iter().map(|&col| &row[col]).collect();
        if !seen.insert(key) {
            return false;
        }
    }
    true
}
