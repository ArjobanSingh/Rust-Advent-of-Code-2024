use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

// Basically create a map, containing page and value as hashet of next_pages after it
// Now while iterating the order array, start from end and check if the item after it
// is present in it's set of next pages we maintained earlier.
// 1. One edge case, when we encountered an item in order, which is not a valid
// left page, that should be considered false. Because that can never come to left of something
pub fn correct_page_order_sum(file_path: &str) {
    let mut result: i32 = 0;
    let mut map: HashMap<i32, HashSet<i32>> = HashMap::new();

    let mut before_empty_line = true;
    if let Ok(file) = File::open(file_path) {
        let lines = io::BufReader::new(file).lines();

        for line in lines.flatten() {
            if line.trim().is_empty() {
                before_empty_line = false;
                continue;
            }

            if before_empty_line {
                let nums: Vec<i32> = line
                    .split("|")
                    .map(|s| s.trim().parse::<i32>().unwrap())
                    .collect();

                let (pg_num, pg_num2) = (nums[0], nums[1]);

                let set = map.entry(pg_num).or_insert_with(HashSet::new);
                set.insert(pg_num2);
            } else {
                // process order
                let order: Vec<i32> = line
                    .split(",")
                    .map(|s| s.trim().parse::<i32>().unwrap())
                    .collect();

                let mut is_all_ok = true;
                for idx in (0..order.len() - 1).rev() {
                    let pg = order[idx];
                    let next_page = order[idx + 1];
                    if let Some(set) = map.get(&pg) {
                        if let None = set.get(&next_page) {
                            // if next page is not present in set of next_pages, this is invalid
                            is_all_ok = false;
                            break;
                        }
                    } else {
                        is_all_ok = false;
                        break;
                    }
                }

                if is_all_ok {
                    result += order[order.len() / 2];
                }
            }
        }
    }

    println!("Challenge 5 ans puzzle 1 {result}");
}
