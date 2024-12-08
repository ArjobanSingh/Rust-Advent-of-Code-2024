use std::fs::File;
use std::io::{self, BufRead};

pub fn check_in_diagonals(matrix: &Vec<Vec<char>>, result: &mut usize) {
    let diag_1: String = vec![matrix[0][0], matrix[1][1], matrix[2][2]]
        .iter()
        .collect();
    let diag_2: String = vec![matrix[0][2], matrix[1][1], matrix[2][0]]
        .iter()
        .collect();

    if (diag_1 == "MAS" || diag_1 == "SAM") && (diag_2 == "MAS" || diag_2 == "SAM") {
        *result += 1;
    }
}

pub fn search_for_xmas(file_path: &str) {
    let mut result: usize = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();

    if let Ok(file) = File::open(file_path) {
        let lines = io::BufReader::new(file).lines();

        for line in lines.flatten() {
            let chars = line.chars();
            matrix.push(chars.collect());
        }
    }

    for row in 0..=matrix.len() - 3 {
        for col in 0..=matrix[0].len() - 3 {
            // create a new gird of 3 * 3
            let mut min_matrix: Vec<Vec<char>> = Vec::new();
            for r in row..row + 3 {
                let mut row_v: Vec<char> = Vec::new();
                for c in col..col + 3 {
                    row_v.push(matrix[r][c]);
                }
                min_matrix.push(row_v);
            }
            check_in_diagonals(&min_matrix, &mut result);
        }
    }
    println!("Challenge 4 Puzzle: {result}");
}

pub fn get_xmas_count(text: String) -> usize {
    let mut result = text.matches("XMAS").count();
    result += text.matches("SAMX").count();
    result
}

pub fn search_for_xmas_part_1(file_path: &str) {
    let mut result: usize = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();

    if let Ok(file) = File::open(file_path) {
        let lines = io::BufReader::new(file).lines();

        for line in lines.flatten() {
            let chars = line.chars();
            matrix.push(chars.collect());
        }
    }

    // for every row get count of occurences
    for row in matrix.iter() {
        result += get_xmas_count(row.iter().collect());
    }

    // for every col get count of occurences
    for col in 0..matrix[0].len() {
        let mut col_vec: Vec<char> = Vec::new();
        for row in 0..matrix.len() {
            col_vec.push(matrix[row][col]);
        }

        result += get_xmas_count(col_vec.iter().collect());
    }

    let rows = matrix.len();
    let cols = matrix[0].len();
    // for every primary diagnol starting from 1st row
    for col in 0..cols {
        let mut diagonal: Vec<char> = Vec::new();
        let mut row: usize = 0;
        let mut current_col = col;

        while row < rows && current_col < cols {
            diagonal.push(matrix[row][current_col]);
            row += 1;
            current_col += 1;
        }
        result += get_xmas_count(diagonal.iter().collect());
    }

    // for every primary diagnol starting from 1st col's 2nd cell. from 1,0
    for row in 1..rows {
        let mut diagonal: Vec<char> = Vec::new();
        let mut cur_row: usize = row;
        let mut col: usize = 0;

        while cur_row < rows && col < cols {
            diagonal.push(matrix[cur_row][col]);
            cur_row += 1;
            col += 1;
        }
        result += get_xmas_count(diagonal.iter().collect());
    }

    let start_col: usize = 0;

    // for every secondary diagnol starting from 1st row
    for col in (start_col..cols).rev() {
        let mut diagonal: Vec<char> = Vec::new();
        let mut row: usize = 0;
        let mut current_col = col;

        while row < rows && current_col >= 0 {
            diagonal.push(matrix[row][current_col]);
            row += 1;
            if current_col == 0 {
                break;
            }
            current_col -= 1;
        }
        result += get_xmas_count(diagonal.iter().collect());
    }

    for row in 1..rows {
        let mut diagonal: Vec<char> = Vec::new();
        let mut cur_row: usize = row;
        let mut col: usize = cols - 1;

        while cur_row < rows && col >= start_col {
            diagonal.push(matrix[cur_row][col]);
            cur_row += 1;
            if col == 0 {
                break;
            }
            col -= 1;
        }
        result += get_xmas_count(diagonal.iter().collect());
    }

    println!("Challenge 4 Puzzle 1 soluion: {result}");
}

// a b s c d
// b c s d a
// c d s a b
// d a s b c
