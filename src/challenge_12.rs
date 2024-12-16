use std::{
    fs::File,
    io::{self, BufRead},
};

const DIRCTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn get_uniq_idx(r: usize, c: usize, cols: usize) -> usize {
    r * cols + c
}

fn inside(cell: (i32, i32), grid_size: (i32, i32)) -> bool {
    let (r, c) = cell;
    let (rows, cols) = grid_size;
    r >= 0 && r < rows && c >= 0 && c < cols
}

fn is_same_neighbour(matrix: &Vec<String>, dir: (i32, i32), cell: (i32, i32)) -> bool {
    let cell_data = matrix[cell.0 as usize].as_bytes()[cell.1 as usize] as char;
    let neighbour = (cell.0 + dir.0, cell.1 + dir.1);

    let (r, c) = neighbour;
    inside(neighbour, (matrix.len() as i32, matrix[0].len() as i32))
        && cell_data == (matrix[r as usize].as_bytes()[c as usize] as char)
}

fn find_ans_for_cells(
    matrix: &Vec<String>,
    visited: &mut Vec<bool>,
    area: &mut i32,
    perimeter: &mut i32,
    cell: (i32, i32),
) {
    let cols = matrix[0].len();
    let cell_idx = get_uniq_idx(cell.0 as usize, cell.1 as usize, cols);

    if visited[cell_idx] {
        return;
    }
    visited[cell_idx] = true;

    *area += 1;

    for idx in 0..4 {
        let (r1, c1) = DIRCTIONS[idx];
        let (r2, c2) = DIRCTIONS[(idx + 1) % 4];

        let is_neigh1_same = is_same_neighbour(matrix, (r1, c1), cell);
        let is_neigh2_same = is_same_neighbour(matrix, (r2, c2), cell);

        // Check for Convex corner, when two neighs are not same
        if !is_neigh1_same && !is_neigh2_same {
            *perimeter += 1;
        } else if is_neigh1_same
            && is_neigh2_same
            && !is_same_neighbour(matrix, (r1 + r2, c1 + c2), cell)
        {
            // Here we checked the Concave corner, where two neighs were same,
            // and diagonal neigh not same in that dir.
            *perimeter += 1;
        }
    }

    for &dir in DIRCTIONS.iter() {
        let (r, c) = (cell.0 + dir.0, cell.1 + dir.1);
        if is_same_neighbour(matrix, dir, cell) {
            let neighbour_idx = get_uniq_idx(r as usize, c as usize, cols);
            if !visited[neighbour_idx] {
                find_ans_for_cells(matrix, visited, area, perimeter, (r, c));
            }
        }
    }
}

pub fn garden_groups(file_path: &str) {
    if let Ok(file) = File::open(file_path) {
        let mut result = 0;
        let mut matrix: Vec<String> = Vec::new();

        let lines = io::BufReader::new(file).lines();

        for line in lines.flatten() {
            matrix.push(line);
        }

        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut visited: Vec<bool> = Vec::new();
        visited.resize(rows * cols, false);

        for r in 0..rows {
            for c in 0..cols {
                let mut area: i32 = 0;
                let mut perimeter: i32 = 0;
                find_ans_for_cells(
                    &matrix,
                    &mut visited,
                    &mut area,
                    &mut perimeter,
                    (r as i32, c as i32),
                );

                result += area * perimeter;
            }
        }

        println!("The anser for the Challenge 12 puz 1: {}", result);
    }
}
