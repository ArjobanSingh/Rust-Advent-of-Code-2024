use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

// Simple for every char at i(0 to end), check at how many different ways, the substrings
// can be created at idx j(subtring of 0..=j), where j goes from 0..i, such that str[j..i] is also available in patterns.
fn can_create(design: &String, patterns: &HashSet<String>) -> i64 {
    let mut dp: Vec<i64> = vec![0; design.len() + 1];
    dp[0] = 1;

    for i in 1..=design.len() {
        for j in 0..i {
            if dp[j] != 0 && patterns.get(&design[j..i]).is_some() {
                dp[i] += dp[j];
            }
        }
    }

    *dp.last().unwrap_or(&0)
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
                let possible_count = can_create(&line, &patterns);
                result += possible_count;
            }
        }

        println!("Challenge 19 {result}");
    }
}
