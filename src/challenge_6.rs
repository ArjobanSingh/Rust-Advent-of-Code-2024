use std::{
    fs::File,
    io::{self, BufRead},
};

const TARGET_CHAR: char = '^';
const BLOCKER: char = '#';

const DIR: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn get_uniq_idx(r: isize, c: isize, cols: isize, dir: usize) -> usize {
    ((r * cols + c) as usize) * DIR.len() + dir
}

pub fn get_guard_distinct_pos_size(file_path: &str) {
    let mut result: i32 = 0;

    let mut grid: Vec<String> = Vec::new();
    let mut initial_pos: (isize, isize) = (-1, -1);

    if let Ok(file) = File::open(file_path) {
        let lines = io::BufReader::new(file).lines();

        for (line_idx, line) in lines.flatten().enumerate() {
            if let Some(col_idx) = line.find(TARGET_CHAR) {
                initial_pos.0 = line_idx as isize;
                initial_pos.1 = col_idx as isize;
            }
            grid.push(line);
        }
    }

    // initial start direction to be top;
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let matrix_size = (rows * cols) as usize;

    for row in 0..rows {
        for col in 0..cols {
            let mut dir = 0;
            let mut pos: (isize, isize) = initial_pos;
            let this_char = grid[row as usize].as_bytes()[col as usize] as char;
            if this_char == BLOCKER || this_char == TARGET_CHAR {
                continue;
            }

            let mut cell_visited: Vec<bool> = Vec::new();
            cell_visited.resize(matrix_size * DIR.len(), false);

            loop {
                // if we already visited the next cell from same direction, break
                let cell_idx = get_uniq_idx(pos.0, pos.1, cols, dir);
                if cell_visited[cell_idx] {
                    result += 1;
                    break;
                }
                cell_visited[cell_idx] = true;

                // get next row and next col
                let (nr, nc): (isize, isize) = (pos.0 + DIR[dir].0, pos.1 + DIR[dir].1);
                if nr < 0 || nr >= rows || nc < 0 || nc >= cols {
                    break;
                }

                // change direction to 90 deg on blocker
                let next_val = grid[nr as usize].as_bytes()[nc as usize] as char;

                if next_val == BLOCKER || (row == nr && col == nc) {
                    dir = (dir + 1) % DIR.len();
                    continue;
                }

                pos.0 = nr;
                pos.1 = nc;
            }
        }
    }

    println!("Challenge 6 ans puzzle 1: {result}");
}
