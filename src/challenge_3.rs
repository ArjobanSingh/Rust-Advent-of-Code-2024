use std::fs;

const CHECK_STR: &'static str = "mul(";

fn get_product_if_valid(text: &str, idx: usize) -> Option<u32> {
    let start_idx = idx + CHECK_STR.len();

    let mut digit_1 = 0;
    let mut digit_2 = 0;

    let is_num_expected = true;
    let mut is_first_num = true;
    let mut is_comma_expected = false;
    let mut is_paran_expected = false;

    if let Some(slice) = text.get(start_idx..) {
        for ch in slice.chars() {
            match ch {
                '0'..='9' => {
                    if !is_num_expected {
                        break;
                    }
                    if let Some(num) = ch.to_digit(10) {
                        if is_first_num {
                            digit_1 = digit_1 * 10 + num;
                            is_comma_expected = true;
                        } else {
                            digit_2 = digit_2 * 10 + num;
                            is_paran_expected = true;
                        }
                    }
                }
                ',' => {
                    if !is_comma_expected {
                        break;
                    }
                    is_comma_expected = false;
                    is_first_num = false;
                }
                ')' => {
                    if !is_paran_expected {
                        break;
                    }

                    return Some(digit_1 * digit_2);
                }
                _ => break, // in case of any other char it is wrong
            }
        }
    }

    None
}

pub fn get_uncorrupted_mul_ans(file_path: &str) {
    let mut result = 0;
    if let Ok(data) = fs::read_to_string(file_path) {
        let do_indices: Vec<usize> = data.match_indices("do()").map(|(index, _)| index).collect();
        let dont_indices: Vec<usize> = data
            .match_indices("don't()")
            .map(|(index, _)| index)
            .collect();

        // Basically for every "mul(" occurence, check what is the previous condition
        // do or don't, based on checking there occurence index before curren mul occurence
        // and then comparing b/w two indices to find the closest one.
        let occur_indices: Vec<usize> = data
            .match_indices(CHECK_STR)
            .map(|(index, _)| index)
            .filter(|&index| {
                let dont_idx_opt = dont_indices
                    .iter()
                    .rev()
                    .find(|&&dont_idx| dont_idx < index);
                if let Some(&dont_idx) = dont_idx_opt {
                    let do_idx_option = do_indices.iter().rev().find(|&&dont_idx| dont_idx < index);
                    if let Some(&do_idx) = do_idx_option {
                        return do_idx > dont_idx;
                    } else {
                        // If no do index present less than current mul(, meaning do was
                        // from start so any lesser don't index than mul( is latest, so return false
                        return false;
                    }
                }
                // if don't index is not present return true to always have the value
                true
            })
            .collect();

        for occur_index in occur_indices {
            if let Some(product) = get_product_if_valid(&data, occur_index) {
                result += product;
            }
        }
    }
    println!("Challenge 3 puzzle 2 solution: {result}");
}

pub fn get_uncorrupted_mul_ans_puzzle_1(file_path: &str) {
    let mut result = 0;
    if let Ok(data) = fs::read_to_string(file_path) {
        // Instead of getting substrings on mul(, I could check that manually
        // using the above function, based on what is expected. but easier to do this way
        let occur_indices: Vec<usize> = data
            .match_indices(CHECK_STR)
            .map(|(index, _)| index)
            .collect();

        for occur_index in occur_indices {
            if let Some(product) = get_product_if_valid(&data, occur_index) {
                result += product;
            }
        }
    }
    println!("Challenge 3 puzzle 1 solution: {result}");
}
