use std::{
    fs::File,
    io::{self, BufRead},
};

const SUM: char = '+';
const PROD: char = '*';

// Backtrack and find every possible combination of + and * for available max_operators size
fn get_operator_combinations(
    combinations: &mut Vec<Vec<char>>,
    combination: &mut Vec<char>,
    max_operators: usize,
) {
    if combination.len() == max_operators {
        combinations.push(combination.clone());
        return;
    }

    combination.push(SUM);
    get_operator_combinations(combinations, combination, max_operators);
    combination.pop();

    combination.push(PROD);
    get_operator_combinations(combinations, combination, max_operators);
    combination.pop();
}

pub fn bridge_repair_puz_1(file_path: &str) {
    let mut result: i64 = 0;

    if let Ok(file) = File::open(file_path) {
        let lines = io::BufReader::new(file).lines();

        for line in lines.flatten() {
            let mut exp_val: Option<i64> = None;
            let mut operands: Vec<i32> = Vec::new();

            for (idx, item) in line.split(":").enumerate() {
                if idx == 0 {
                    if let Ok(num) = item.parse::<i64>() {
                        exp_val = Some(num);
                    }
                } else {
                    for operand in item.split_whitespace() {
                        if let Ok(num) = operand.parse::<i32>() {
                            operands.push(num);
                        }
                    }
                }
            }

            if let Some(expected) = exp_val {
                let mut combinations: Vec<Vec<char>> = Vec::new();
                get_operator_combinations(&mut combinations, &mut Vec::new(), operands.len() - 1);

                // for every possible operator combination based on available operator space, find ans
                for combination in combinations {
                    let mut cur_ans: i64 = operands[0] as i64;

                    for (operand_idx, &operand) in operands.iter().enumerate().skip(1) {
                        let operator_idx = operand_idx - 1;

                        let operator = combination[operator_idx];
                        if operator == SUM {
                            cur_ans += operand as i64;
                        } else {
                            cur_ans *= operand as i64;
                        }
                    }

                    if cur_ans == expected {
                        result += expected;
                        break;
                    }
                }
            }
        }
    }

    println!("Result of Challenge 7, puz 1 {result}");
}
