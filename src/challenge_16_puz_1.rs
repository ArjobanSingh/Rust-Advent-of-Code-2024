use std::{
    cmp,
    collections::BinaryHeap,
    fs::File,
    i32,
    io::{self, BufRead},
};

const ROBOT: char = 'S';
const WALL: char = '#';
const END: char = 'E';
const ROTATE_COST: i32 = 1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug, Clone, Copy)]
struct NeighbourPath {
    direction: Direction,
    rotated_no: i32,
}

impl Direction {
    fn get_direction_idx(&self) -> i32 {
        match self {
            Direction::Top => 0,
            Direction::Right => 1,
            Direction::Bottom => 2,
            Direction::Left => 3,
        }
    }

    fn get_neighbour_path(&self) -> [NeighbourPath; 4] {
        match self {
            Direction::Top => [
                NeighbourPath {
                    direction: Direction::Top,
                    rotated_no: 0,
                },
                NeighbourPath {
                    direction: Direction::Left,
                    rotated_no: 1,
                },
                NeighbourPath {
                    direction: Direction::Right,
                    rotated_no: 1,
                },
                NeighbourPath {
                    direction: Direction::Bottom,
                    rotated_no: 2,
                },
            ],
            Direction::Right => [
                NeighbourPath {
                    direction: Direction::Right,
                    rotated_no: 0,
                },
                NeighbourPath {
                    direction: Direction::Top,
                    rotated_no: 1,
                },
                NeighbourPath {
                    direction: Direction::Bottom,
                    rotated_no: 1,
                },
                NeighbourPath {
                    direction: Direction::Left,
                    rotated_no: 2,
                },
            ],
            Direction::Bottom => [
                NeighbourPath {
                    direction: Direction::Bottom,
                    rotated_no: 0,
                },
                NeighbourPath {
                    direction: Direction::Left,
                    rotated_no: 1,
                },
                NeighbourPath {
                    direction: Direction::Right,
                    rotated_no: 1,
                },
                NeighbourPath {
                    direction: Direction::Top,
                    rotated_no: 2,
                },
            ],
            Direction::Left => [
                NeighbourPath {
                    direction: Direction::Left,
                    rotated_no: 0,
                },
                NeighbourPath {
                    direction: Direction::Top,
                    rotated_no: 1,
                },
                NeighbourPath {
                    direction: Direction::Bottom,
                    rotated_no: 1,
                },
                NeighbourPath {
                    direction: Direction::Right,
                    rotated_no: 2,
                },
            ],
        }
    }

    fn to_offset(&self) -> (i32, i32) {
        match self {
            Direction::Top => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Bottom => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    row: i32,
    col: i32,
    distance: i32,
    direction: Direction,
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
            .then_with(|| {
                self.direction
                    .get_direction_idx()
                    .cmp(&other.direction.get_direction_idx())
            })
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_uniq_idx_with_dir(r: i32, c: i32, cols: i32, dir: Direction) -> i32 {
    (r * cols + c) * 4 + dir.get_direction_idx()
}

fn inside(matrix: &Vec<Vec<char>>, position: (i32, i32)) -> bool {
    let rows = matrix.len() as i32;
    let cols = matrix[0].len() as i32;
    let (row, col) = position;

    !(row < 0 || row >= rows || col < 0 || col >= cols)
}

// Find using Djikstra's Algorithm
fn find_shortest_correct_path(
    matrix: &mut Vec<Vec<char>>,
    visited: &mut Vec<bool>,
    distances: &mut Vec<i32>,
    start_position: (i32, i32),
    start_direction: Direction,
) {
    let cols = matrix[0].len();
    let mut min_queue: BinaryHeap<State> = BinaryHeap::new();
    min_queue.push(State {
        distance: 0,
        row: start_position.0,
        col: start_position.1,
        direction: start_direction,
    });

    // till we have visited all the valid nodes
    while let Some(current) = min_queue.pop() {
        let State {
            row,
            col,
            direction,
            distance,
        } = current;

        let uniq_idx = get_uniq_idx_with_dir(row, col, cols as i32, direction);
        visited[uniq_idx as usize] = true;

        for neighbour_path in direction.get_neighbour_path() {
            let NeighbourPath {
                direction: neigh_direction,
                rotated_no,
            } = neighbour_path;

            let tentative_distance = ROTATE_COST * rotated_no + 1 + distance;
            let (dy, dx) = neigh_direction.to_offset();
            let (nr, nc) = (row + dy, col + dx);

            if !inside(matrix, (nr, nc)) {
                continue;
            }

            let neigh_data = matrix[nr as usize][nc as usize];
            if neigh_data == WALL {
                continue;
            }

            let neigh_uniq_idx = get_uniq_idx_with_dir(nr, nc, cols as i32, neigh_direction);

            if visited[neigh_uniq_idx as usize] {
                continue;
            }

            // update the distance and push only, if tentative smaller then prev saved distance,
            // from the same direction
            if tentative_distance < distances[neigh_uniq_idx as usize] {
                distances[neigh_uniq_idx as usize] = tentative_distance;
                min_queue.push(State {
                    row: nr,
                    col: nc,
                    direction: neigh_direction,
                    distance: tentative_distance,
                });
            }
        }
    }
}

pub fn reindeer_olympics_v1(file_path: &str) {
    if let Ok(file) = File::open(file_path) {
        let mut matrix: Vec<Vec<char>> = Vec::new();

        let lines = io::BufReader::new(file).lines();

        let mut robot_position: (i32, i32) = (0, 0);
        let mut end_position: (i32, i32) = (0, 0);

        for line in lines.flatten() {
            let mut row: Vec<char> = Vec::new();
            for ch in line.chars() {
                if ch == ROBOT {
                    robot_position = (matrix.len() as i32, row.len() as i32);
                } else if ch == END {
                    end_position = (matrix.len() as i32, row.len() as i32);
                }
                row.push(ch);
            }
            matrix.push(row);
        }

        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut visited = Vec::new();
        visited.resize(rows * cols * 4, false);

        let mut distances = Vec::new();
        distances.resize(rows * cols * 4, i32::MAX);

        find_shortest_correct_path(
            &mut matrix,
            &mut visited,
            &mut distances,
            robot_position,
            Direction::Right,
        );

        let (er, ec) = end_position;
        let answer = [
            Direction::Top,
            Direction::Right,
            Direction::Bottom,
            Direction::Left,
        ]
        .iter()
        .map(|&direction| {
            let idx = get_uniq_idx_with_dir(er, ec, cols as i32, direction);
            distances[idx as usize]
        })
        .min();

        println!("Challenge 16 ans: {:?}", answer);
    }
}
