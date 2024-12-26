use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

fn can_create(design: &String, patterns: &HashSet<String>) -> bool {
    let mut dp = vec![false; design.len() + 1];
    dp[0] = true;

    for i in 1..=design.len() {
        for j in 0..i {
            if dp[j] && patterns.get(&design[j..i]).is_some() {
                dp[i] = true;
                break;
            }
        }
    }

    *dp.last().unwrap_or(&false)
}

pub fn linen_layout(file_path: &str) {
    if let Ok(file) = File::open(file_path) {
        let mut patterns: HashSet<String> = HashSet::new();

        let mut result = 0;

        let lines = io::BufReader::new(file).lines();
        let mut is_reading_patterns = true;

        for line in lines.flatten() {
            if line.trim().is_empty() {
                is_reading_patterns = false;
                continue;
            }

            if is_reading_patterns {
                for pattern in line.split(", ").map(|s| s.to_string()) {
                    patterns.insert(pattern);
                }
            } else {
                let is_possible = can_create(&line, &patterns);
                if is_possible {
                    result += 1;
                }
            }
        }

        println!("Challenge 19 {result}");
    }
}
