use std::{
    fs::File,
    io::{self, BufRead},
};

const DIR: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

// fn get_uniq_idx(r: usize, c: usize, cols: usize) -> usize {
//     r * cols + c
// }

fn find_trails(matrix: &mut Vec<Vec<u8>>, cell: (usize, usize), result: &mut i32) {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let cell_data = matrix[cell.0][cell.1];

    if cell_data == 9 {
        *result += 1;
        return;
    }

    for dir in DIR {
        let (n_row, n_col) = (cell.0 as isize + dir.0, cell.1 as isize + dir.1);
        if n_row < 0 || n_col < 0 {
            continue;
        }

        let n_row = n_row as usize;
        let n_col = n_col as usize;
        if n_row >= rows || n_col >= cols {
            continue;
        }

        let n_cell_data = matrix[n_row][n_col];

        if cell_data + 1 == n_cell_data {
            find_trails(matrix, (n_row, n_col), result);
        }
    }
}

pub fn topographic_map_search(file_path: &str) {
    if let Ok(file) = File::open(file_path) {
        let mut result = 0;
        let mut matrix: Vec<Vec<u8>> = Vec::new();
        let lines = io::BufReader::new(file).lines();

        for line in lines.flatten() {
            let mut row: Vec<u8> = Vec::new();
            for ch in line.chars() {
                if let Some(num) = ch.to_digit(10) {
                    row.push(num as u8);
                }
            }
            matrix.push(row);
        }

        let rows = matrix.len();
        let cols = matrix[0].len();

        for r in 0..rows {
            for c in 0..cols {
                if matrix[r][c] == 0 {
                    find_trails(&mut matrix, (r, c), &mut result);
                }
            }
        }

        println!("The anser for the Challenge 10 puz 1: {result}");
    }
}

// 7770777
// 7771777
// 8772778
// 6543456
// 7111117
// 8111118
// 9111119
