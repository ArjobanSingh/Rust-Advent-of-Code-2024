use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

const DIRCTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[derive(Debug, Clone, Copy)]
struct Group {
    area: i32,
    perimeter: i32,
    ans: i32,
}

fn get_uniq_idx(r: usize, c: usize, cols: usize) -> usize {
    r * cols + c
}

fn find_ans_for_cells(
    matrix: &Vec<String>,
    map: &mut HashMap<char, Group>,
    visited: &mut Vec<bool>,
    cell: (i32, i32),
) {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let cell_idx = get_uniq_idx(cell.0 as usize, cell.1 as usize, cols);

    if visited[cell_idx] {
        return;
    }
    visited[cell_idx] = true;

    let cell_data = matrix[cell.0 as usize].as_bytes()[cell.1 as usize] as char;
    let entry = map.entry(cell_data).or_insert(Group {
        area: 0,
        perimeter: 0,
        ans: 0,
    });
    entry.area += 1;

    for dir in DIRCTIONS.iter() {
        let (r, c) = (cell.0 + dir.0, cell.1 + dir.1);
        if r < 0 || r >= rows as i32 || c < 0 || c >= cols as i32 {
            if let Some(entry) = map.get_mut(&cell_data) {
                entry.perimeter += 1;
                continue;
            }
        }

        let neighbour = matrix[r as usize].as_bytes()[c as usize] as char;
        if neighbour == cell_data {
            let neighbour_idx = get_uniq_idx(r as usize, c as usize, cols);
            if !visited[neighbour_idx] {
                find_ans_for_cells(matrix, map, visited, (r, c));
            }
        } else {
            if let Some(entry) = map.get_mut(&cell_data) {
                entry.perimeter += 1;
            }
        }
    }
}

pub fn garden_groups_v1(file_path: &str) {
    if let Ok(file) = File::open(file_path) {
        let mut matrix: Vec<String> = Vec::new();
        let mut map: HashMap<char, Group> = HashMap::new();

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
                find_ans_for_cells(&matrix, &mut map, &mut visited, (r as i32, c as i32));

                let cell_data = matrix[r].as_bytes()[c] as char;
                // after completion of each region, mutltiple area and perimeter and reset them for next region
                if let Some(entry) = map.get_mut(&cell_data) {
                    entry.ans += entry.area * entry.perimeter;
                    entry.area = 0;
                    entry.perimeter = 0;
                }
            }
        }

        println!(
            "The anser for the Challenge 12 puz 1: {}",
            map.values().fold(0, |sum, group| sum + group.ans)
        );
    }
}
