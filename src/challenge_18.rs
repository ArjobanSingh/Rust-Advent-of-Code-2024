use std::{
    cmp,
    collections::BinaryHeap,
    fs::File,
    io::{self, BufRead},
};

const WALL: char = '#';
const ROWS: i32 = 71;
const COLS: i32 = 71;
const BREAK_AT: i32 = 1024;

const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    row: i32,
    col: i32,
    distance: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // as we need to push this in Heap, which is by default MaxHeap, so we updating the ordering comparator
        // to return min value first. Kinda reverse() value to treat it as min heap
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.row.cmp(&other.row))
            .then_with(|| self.col.cmp(&other.col))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_uniq_idx(r: i32, c: i32, cols: i32) -> i32 {
    r * cols + c
}

fn inside(matrix: &Vec<Vec<char>>, position: (i32, i32)) -> bool {
    let rows = matrix.len() as i32;
    let cols = matrix[0].len() as i32;
    let (row, col) = position;

    !(row < 0 || row >= rows || col < 0 || col >= cols)
}

// Djikstra's Algorithm
fn check_if_shortest_path_available(
    matrix: &Vec<Vec<char>>,
    distances: &mut Vec<i32>,
    visited: &mut Vec<bool>,
) -> bool {
    let start_pos: (i32, i32) = (0, 0);
    let end_position: (i32, i32) = (ROWS - 1, COLS - 1);

    let mut min_queue: BinaryHeap<State> = BinaryHeap::new();
    min_queue.push(State {
        distance: 0,
        row: start_pos.0,
        col: start_pos.1,
    });

    while let Some(state) = min_queue.pop() {
        let State { distance, row, col } = state;
        let idx = get_uniq_idx(row, col, COLS);

        if visited[idx as usize] {
            continue;
        }
        visited[idx as usize] = true;

        if (row, col) == end_position {
            break;
        }

        for (dy, dx) in DIRS {
            let (nr, nc) = (row + dy, col + dx);
            let neigh_idx = get_uniq_idx(nr, nc, COLS);

            // if out of bounds or wall or already visited/processed, skip
            if !inside(matrix, (nr, nc))
                || matrix[nr as usize][nc as usize] == WALL
                || visited[neigh_idx as usize]
            {
                continue;
            }

            let tentative_distance = distance + 1;
            if tentative_distance < distances[neigh_idx as usize] {
                distances[neigh_idx as usize] = tentative_distance;
                min_queue.push(State {
                    row: nr,
                    col: nc,
                    distance: tentative_distance,
                });
            }
        }
    }

    let idx = get_uniq_idx(end_position.0, end_position.1, COLS);

    if distances[idx as usize] == i32::MAX {
        // We didn't find the exit path
        false
    } else {
        true
    }
}

pub fn ram_run(file_path: &str) {
    if let Ok(file) = File::open(file_path) {
        let mut matrix: Vec<Vec<char>> = vec![vec!['.'; ROWS as usize]; COLS as usize];

        let lines = io::BufReader::new(file).lines();

        let mut byte_idx = 0;
        for line in lines.flatten() {
            let coords: Vec<i32> = line
                .split(',')
                .filter_map(|str| str.to_string().parse::<i32>().ok())
                .collect();

            let col = coords[0];
            let row = coords[1];

            matrix[row as usize][col as usize] = WALL;
            byte_idx += 1;
            if byte_idx <= BREAK_AT {
                continue;
            }

            // Instead of checking all the next paths after break index.
            // I could have run a BINARY SEARCH to find the shortest input pair
            // which would have blocked the exit.
            let mut distances: Vec<i32> = vec![i32::MAX; (ROWS * COLS) as usize];
            let mut visited: Vec<bool> = vec![false; (ROWS * COLS) as usize];
            let is_available =
                check_if_shortest_path_available(&matrix, &mut distances, &mut visited);

            if !is_available {
                println!("Byte Index in input: {}", byte_idx);
                println!("Ans: {:?}", (col, row));
                break;
            }
        }
    }
}
