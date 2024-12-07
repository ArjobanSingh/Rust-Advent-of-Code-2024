use std::{
    fs::File,
    io::{self, BufRead},
};

const TARGET_CHAR: char = '^';
const BLOCKER: char = '#';

const DIR: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn get_uniq_idx(r: isize, c: isize, cols: isize) -> isize {
    r * cols + c
}

pub fn get_guard_distinct_pos_size(file_path: &str) {
    let mut result: i32 = 1;

    let mut grid: Vec<String> = Vec::new();
    let mut pos: (isize, isize) = (-1, -1);

    if let Ok(file) = File::open(file_path) {
        let lines = io::BufReader::new(file).lines();

        for (line_idx, line) in lines.flatten().enumerate() {
            if let Some(col_idx) = line.find(TARGET_CHAR) {
                pos.0 = line_idx as isize;
                pos.1 = col_idx as isize;
            }
            grid.push(line);
        }
    }

    // initial start direction to be top;
    let mut dir = 0;
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    let mut cell_visited: Vec<bool> = Vec::new();
    cell_visited.resize((rows * cols) as usize, false);
    cell_visited[get_uniq_idx(pos.0, pos.1, cols) as usize] = true;

    loop {
        // is next position end of line;
        let (r, c): (isize, isize) = (pos.0 + DIR[dir].0, pos.1 + DIR[dir].1);
        if r < 0 || r >= rows || c < 0 || c >= cols {
            break;
        }

        // change direction to 90 deg on blocker
        let next_val = grid[r as usize].as_bytes()[c as usize] as char;
        if next_val == BLOCKER {
            dir = (dir + 1) % DIR.len();
            continue;
        }

        let cell_idx = get_uniq_idx(r, c, cols);
        if !cell_visited[cell_idx as usize] {
            result += 1;
        }
        cell_visited[cell_idx as usize] = true;

        pos.0 = r;
        pos.1 = c;
    }
    println!("Challenge 6 ans puzzle 1: {result}");
}
