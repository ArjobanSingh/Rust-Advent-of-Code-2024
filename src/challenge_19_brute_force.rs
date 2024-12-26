use std::{
    fs::File,
    io::{self, BufRead},
};

fn can_create(cur_str: String, ans: &String, patterns: &Vec<String>, start_idx: usize) -> bool {
    if cur_str == *ans {
        return true;
    }

    if cur_str.len() >= ans.len() {
        return false;
    }

    // brwrr
    //
    for pattern in patterns {
        if !&ans[start_idx..].starts_with(pattern) {
            continue;
        }

        let new_str = cur_str.clone() + pattern;
        let new_len = &new_str.len();

        if can_create(new_str, ans, patterns, *new_len) {
            return true;
        }
    }

    false
}

pub fn linen_layout(file_path: &str) {
    if let Ok(file) = File::open(file_path) {
        let mut patterns: Vec<String> = Vec::new();

        let mut result = 0;

        let lines = io::BufReader::new(file).lines();
        let mut is_reading_patterns = true;

        let mut count = 3;
        for line in lines.flatten() {
            if line.trim().is_empty() {
                is_reading_patterns = false;
                continue;
            }

            if is_reading_patterns {
                for pattern in line.split(", ").map(|s| s.to_string()) {
                    patterns.push(pattern);
                }
            } else {
                println!("Count: {count} and line {}", line);
                let is_possible = can_create(String::new(), &line, &patterns, 0);
                println!("Is possible: {}", is_possible);
                if is_possible {
                    result += 1;
                }
                count += 1;
            }
        }

        println!("Challenge 19 {result}");
    }
}
