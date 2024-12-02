use std::fs::File;
use std::io::{self, BufRead};

const INC: &'static str = "Increasing";
const DEC: &'static str = "Decreasing";

pub fn get_safe_reports_count(file_path: &str) {
    let mut safe_count: u16 = 0;
    if let Ok(file) = File::open(file_path) {
        let lines = io::BufReader::new(file).lines();

        'outer: for line in lines.flatten() {
            let levels: Vec<i8> = line
                .split_whitespace()
                .filter_map(|str| str.parse().ok())
                .collect();

            // we'll consider an array of a single item to be safe
            if levels.len() < 2 {
                safe_count += 1;
                continue 'outer;
            }

            let order = if levels[1] < levels[0] { DEC } else { INC };

            // Iterate from 1st index, not 0th
            for (idx, &level) in levels.iter().enumerate().skip(1) {
                let diff = level - levels[idx - 1];

                // has to be either inc or dec
                if diff == 0 {
                    continue 'outer;
                }

                // current diff didn't match the initial inc order
                if order == INC && diff < 0 {
                    continue 'outer;
                }

                // current diff didn't match the nitial dec order
                if order == DEC && diff > 0 {
                    continue 'outer;
                }

                // the difference b/w two items should be <= 3, here' it's not
                if level.abs_diff(levels[idx - 1]) > 3 {
                    continue 'outer;
                }
            }

            // If we reached here, meaning this levels array passed the check
            safe_count += 1
        }
    }

    println!("Challenge 2 soulution: {safe_count}");
}
