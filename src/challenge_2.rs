use std::fs::File;
use std::io::{self, BufRead};

const INC: &'static str = "Increasing";
const DEC: &'static str = "Decreasing";

fn check_if_safe_vector(levels: &Vec<i8>, ignore_idx_option: Option<usize>) -> bool {
    // If only 1 item or 2(in case of ignore option), return true early,
    // as single item vector is always valid
    if let Some(_) = ignore_idx_option {
        if levels.len() < 3 {
            return true;
        }
    } else if levels.len() < 2 {
        return true;
    }

    let mut order = "";

    // In case 0th idx is the skipping index, start iteration from 2nd index, else for 1st idx
    let skip = match ignore_idx_option {
        Some(idx) => {
            if idx != 0 {
                1
            } else {
                2
            }
        }
        None => 1,
    };

    for (idx, &level) in levels.iter().enumerate().skip(skip) {
        // Get the valid prev idx by skipping the ignore_idx
        let prev_idx = if let Some(ignore_idx) = ignore_idx_option {
            // skip over whole iteration if the current iterated value is the one being ignored
            if ignore_idx == idx {
                continue;
            }

            if ignore_idx == idx - 1 {
                idx - 2
            } else {
                idx - 1
            }
        } else {
            idx - 1
        };

        order = if order != "" {
            order
        } else if levels[idx] < levels[prev_idx] {
            DEC
        } else {
            INC
        };

        let diff = level - levels[prev_idx];

        // has to be either inc or dec
        if diff == 0 {
            return false;
        }

        // current diff didn't match the initial inc order
        if order == INC && diff < 0 {
            return false;
        }

        // current diff didn't match the nitial dec order
        if order == DEC && diff > 0 {
            return false;
        }

        // the difference b/w two items should be <= 3, here' it's not
        if level.abs_diff(levels[prev_idx]) > 3 {
            return false;
        }
    }

    true
}

pub fn get_safe_reports_count(file_path: &str) {
    let mut safe_count: u16 = 0;
    if let Ok(file) = File::open(file_path) {
        let lines = io::BufReader::new(file).lines();

        for line in lines.flatten() {
            let levels: Vec<i8> = line
                .split_whitespace()
                .filter_map(|str| str.parse().ok())
                .collect();

            // we'll consider an array of a single item to be safe
            let is_safe = check_if_safe_vector(&levels, None);

            // If we reached here, meaning this levels array passed the check
            if is_safe {
                safe_count += 1;
                continue;
            }

            // check if vector becomes safe after removing any 1 element in it
            for (idx, _) in levels.iter().enumerate() {
                let is_safe = check_if_safe_vector(&levels, Some(idx));
                if is_safe {
                    safe_count += 1;
                    break;
                }
            }
        }
    }

    println!("Challenge 2 soulution: {safe_count}");
}

pub fn get_safe_reports_count_ch_1(file_path: &str) {
    let mut safe_count: u16 = 0;
    if let Ok(file) = File::open(file_path) {
        let lines = io::BufReader::new(file).lines();

        for line in lines.flatten() {
            let levels: Vec<i8> = line
                .split_whitespace()
                .filter_map(|str| str.parse().ok())
                .collect();

            // we'll consider an array of a single item to be safe
            if check_if_safe_vector(&levels, None) {
                safe_count += 1;
            }
        }
    }

    println!("Challenge 2, Puzzle 1 soulution: {safe_count}");
}
