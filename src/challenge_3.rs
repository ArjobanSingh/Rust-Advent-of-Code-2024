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
