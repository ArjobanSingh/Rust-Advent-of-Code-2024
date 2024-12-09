use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}

impl Direction {
    /// Get the matrix value (row, col) for the direction
    fn to_offset(self) -> (isize, isize) {
        match self {
            Direction::Top => (-1, 0),
            Direction::TopRight => (-1, 1),
            Direction::Right => (0, 1),
            Direction::BottomRight => (1, 1),
            Direction::Bottom => (1, 0),
            Direction::BottomLeft => (1, -1),
            Direction::Left => (0, -1),
            Direction::TopLeft => (-1, -1),
        }
    }
}

fn get_uniq_idx(r: usize, c: usize, cols: usize) -> usize {
    r * cols + c
}

fn get_direction(cell: (usize, usize), position: (usize, usize)) -> Option<Direction> {
    // If same row, then direction is either left or right
    if cell.0 == position.0 {
        return if cell.1 < position.1 {
            Some(Direction::Right)
        } else {
            Some(Direction::Left)
        };
    }

    // If same column, then direction is either top or bottom
    if cell.1 == position.1 {
        return if cell.0 < position.0 {
            Some(Direction::Bottom)
        } else {
            Some(Direction::Top)
        };
    }

    // Else give all 4 possible diagonal directions
    if position.0 < cell.0 && position.1 > cell.1 {
        return Some(Direction::TopRight);
    }

    if position.0 > cell.0 && position.1 > cell.1 {
        return Some(Direction::BottomRight);
    }

    if position.0 > cell.0 && position.1 < cell.1 {
        return Some(Direction::BottomLeft);
    }

    if position.0 < cell.0 && position.1 < cell.1 {
        return Some(Direction::TopLeft);
    }

    // would never reach here actually
    None
}

fn get_is_antinode_on_cell(
    cell: (usize, usize),
    positions: &HashSet<(usize, usize)>,
    ant_nodes: &mut Vec<bool>,
    matrix_size: (usize, usize),
) -> bool {
    let rows_len = matrix_size.0 as isize;
    let cols_len = matrix_size.1 as isize;

    for &position in positions {
        if cell.0 == position.0 && cell.1 == position.1 {
            continue;
        }

        if let Some(dir) = get_direction(cell, position) {
            // These two are aligned
            let row_diff = cell.0.abs_diff(position.0);
            let col_diff = cell.1.abs_diff(position.1);

            let dir_offset = dir.to_offset();

            // While in same direction of the matched Antenaa, till we are in bounds, keep on adding antinodes
            // at same difference.
            let mut offset_row = position.0 as isize;
            let mut offset_col = position.1 as isize;

            while offset_row >= 0
                && offset_row < rows_len
                && offset_col >= 0
                && offset_col < cols_len
            {
                let cell_idx =
                    get_uniq_idx(offset_row as usize, offset_col as usize, cols_len as usize);
                ant_nodes[cell_idx] = true;
                offset_row += row_diff as isize * dir_offset.0;
                offset_col += col_diff as isize * dir_offset.1;
            }
        }
    }
    false
}

pub fn resonant_collinearity(file_path: &str) {
    let mut node_map: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();
    let mut matrix_size: Option<(usize, usize)> = None;

    if let Ok(file) = File::open(file_path) {
        let lines = io::BufReader::new(file).lines();
        let mut rows: usize = 0;
        let mut col_size: usize = 0;

        for (row, line) in lines.flatten().enumerate() {
            rows += 1;
            if col_size == 0 {
                col_size = line.len();
            }
            for (col, &byte) in line.as_bytes().iter().enumerate() {
                let ch = byte as char;
                if ch != '.' {
                    let set = node_map.entry(ch).or_insert_with(HashSet::new);
                    set.insert((row, col));
                }
            }
        }

        matrix_size = Some((rows, col_size));
    }

    if let Some(size) = matrix_size {
        let (rows, cols) = size;

        let mut anti_nodes: Vec<bool> = Vec::new();
        anti_nodes.resize(rows * cols, false);

        // For every antenna, check every other antenna of same frequency, and calculate the diff b/w the two
        // and use the same diff to add antinodes at same direction till they are in bounds.
        for (_, positions) in node_map {
            for &position in &positions {
                get_is_antinode_on_cell(position, &positions, &mut anti_nodes, size);
            }
        }

        let result = anti_nodes.iter().filter(|&&val| val).count();
        println!("Result of Challenge 8, puz 2: {result}");
    }
}
