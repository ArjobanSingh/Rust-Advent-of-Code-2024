use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

pub fn swap_till_end(order: &mut Vec<i32>, map: &mut HashMap<i32, HashSet<i32>>, wrong_idx: usize) {
    let mut idx = wrong_idx;
    while idx < order.len() - 1 {
        // we know the first occurence is anyways wrong, no need to check
        if idx != wrong_idx {
            let pg = order[idx];
            let next_page = order[idx + 1];

            // if pg and next page are already at correct order, no need to swap further
            if map.get(&pg).and_then(|s| s.get(&next_page)).is_some() {
                break;
            }
        }

        order.swap(idx, idx + 1);
        idx += 1;
    }
}

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
                let mut order: Vec<i32> = line
                    .split(",")
                    .map(|s| s.trim().parse::<i32>().unwrap())
                    .collect();

                let mut was_wrong = false;
                for idx in (0..order.len() - 1).rev() {
                    let pg = order[idx];
                    let next_page = order[idx + 1];

                    if map.get(&pg).and_then(|s| s.get(&next_page)).is_none() {
                        // if either page itself is not present in map(meaning it should be last)
                        // or the next page is not present in it's set. Wrong, so reverse it.
                        swap_till_end(&mut order, &mut map, idx);
                        was_wrong = true;
                    }
                }

                if was_wrong {
                    result += order[order.len() / 2];
                }
            }
        }
    }

    println!("Challenge 5 ans puzzle 2 {result}");
}
