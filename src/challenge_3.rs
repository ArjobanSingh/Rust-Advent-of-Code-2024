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

fn find_largest_smaller_than_x(arr: &Vec<usize>, x: usize) -> Option<usize> {
    match arr.binary_search(&x) {
        Ok(index) => {
            // If we found `x`, return the previous element (largest smaller)
            if index > 0 {
                Some(arr[index - 1])
            } else {
                None
            }
        }
        Err(index) => {
            // If `x` isn't found, `index` is where `x` would go
            if index > 0 {
                Some(arr[index - 1]) // Largest smaller element
            } else {
                None // No smaller element exists
            }
        }
    }
}

pub fn get_uncorrupted_mul_ans(file_path: &str) {
    let mut result = 0;
    if let Ok(data) = fs::read_to_string(file_path) {
        let do_indices: Vec<usize> = data.match_indices("do()").map(|(index, _)| index).collect();
        let dont_indices: Vec<usize> = data
            .match_indices("don't()")
            .map(|(index, _)| index)
            .collect();

        // For each "mul(" occurrence, check the closest preceding condition by comparing indices.
        let occur_indices: Vec<usize> = data
            .match_indices(CHECK_STR)
            .map(|(index, _)| index)
            .filter(|&mul_occ_idx| {
                if let Some(dont_idx) = find_largest_smaller_than_x(&dont_indices, mul_occ_idx) {
                    if let Some(do_idx) = find_largest_smaller_than_x(&do_indices, mul_occ_idx) {
                        return do_idx > dont_idx;
                    } else {
                        // If no "do()"" index present less than current mul(, meaning don't()
                        // condition is recent here, so return false to not use this mul.
                        return false;
                    }
                }

                // if don't index is not present return true to always consider this mul occurence
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
