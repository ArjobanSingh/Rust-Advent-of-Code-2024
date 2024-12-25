use std::{
    cmp,
    collections::{BinaryHeap, HashSet},
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
    rotations: i32,
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

    fn get_opposite_dir(&self) -> Direction {
        match self {
            Direction::Top => Direction::Bottom,
            Direction::Bottom => Direction::Top,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn get_neighbour_path(&self) -> [NeighbourPath; 4] {
        match self {
            Direction::Top => [
                NeighbourPath {
                    direction: Direction::Top,
                    rotations: 0,
                },
                NeighbourPath {
                    direction: Direction::Left,
                    rotations: 1,
                },
                NeighbourPath {
                    direction: Direction::Right,
                    rotations: 1,
                },
                NeighbourPath {
                    direction: Direction::Bottom,
                    rotations: 2,
                },
            ],
            Direction::Right => [
                NeighbourPath {
                    direction: Direction::Right,
                    rotations: 0,
                },
                NeighbourPath {
                    direction: Direction::Top,
                    rotations: 1,
                },
                NeighbourPath {
                    direction: Direction::Bottom,
                    rotations: 1,
                },
                NeighbourPath {
                    direction: Direction::Left,
                    rotations: 2,
                },
            ],
            Direction::Bottom => [
                NeighbourPath {
                    direction: Direction::Bottom,
                    rotations: 0,
                },
                NeighbourPath {
                    direction: Direction::Left,
                    rotations: 1,
                },
                NeighbourPath {
                    direction: Direction::Right,
                    rotations: 1,
                },
                NeighbourPath {
                    direction: Direction::Top,
                    rotations: 2,
                },
            ],
            Direction::Left => [
                NeighbourPath {
                    direction: Direction::Left,
                    rotations: 0,
                },
                NeighbourPath {
                    direction: Direction::Top,
                    rotations: 1,
                },
                NeighbourPath {
                    direction: Direction::Bottom,
                    rotations: 1,
                },
                NeighbourPath {
                    direction: Direction::Right,
                    rotations: 2,
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

pub fn print_nodes_in_path(matrix: &Vec<Vec<char>>, nodes: &HashSet<(i32, i32)>) {
    let rows = matrix.len();
    let cols = matrix[0].len();

    for r in 0..rows {
        for c in 0..cols {
            if nodes.contains(&(r as i32, c as i32)) {
                print!("O");
            } else {
                print!("{}", matrix[r][c]);
            }
        }
        println!();
    }
}
pub fn backtrack_single_best_path(
    matrix: &Vec<Vec<char>>,
    start_position: (i32, i32),
    mut end_position: (i32, i32),
    distances: &Vec<i32>,
) {
    let cols = matrix[0].len();

    let mut path: Vec<(i32, i32)> = Vec::new();
    path.push(end_position);

    while end_position != start_position {
        let (er, ec) = end_position;

        let min_neigh_direction = [
            Direction::Top,
            Direction::Right,
            Direction::Bottom,
            Direction::Left,
        ]
        .iter()
        .map(|&direction| {
            let idx = get_uniq_idx_with_dir(er, ec, cols as i32, direction);
            (distances[idx as usize], direction)
        })
        .min_by(|a, b| a.0.cmp(&b.0));

        if let Some((_, min_neigh_direction)) = min_neigh_direction {
            let (dy, dx) = min_neigh_direction.to_offset();
            let (nr, nc) = (er + dy * -1, ec + dx * -1);

            path.push((nr, nc));
            end_position.0 = nr;
            end_position.1 = nc;
        } else {
            // Shouldn't happen though
            break;
        }
    }

    let set: HashSet<(i32, i32)> = path.into_iter().collect();
    print_nodes_in_path(matrix, &set);
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

    let idx = get_uniq_idx_with_dir(
        start_position.0,
        start_position.1,
        cols as i32,
        start_direction,
    );
    distances[idx as usize] = 0;

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
                rotations,
            } = neighbour_path;

            // If there's different direction, just rotate it and do not move forward.
            let (tentative_distance, (nr, nc)) = if neigh_direction != direction {
                (ROTATE_COST * rotations + distance, (row, col))
            } else {
                // Move forward in same direction.
                let (dy, dx) = neigh_direction.to_offset();
                (1 + distance, (row + dy, col + dx))
            };

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

fn get_min_node_dist_for_node(
    position: (i32, i32),
    cols: i32,
    distances: &Vec<i32>,
) -> Option<(i32, Direction)> {
    [
        Direction::Top,
        Direction::Right,
        Direction::Bottom,
        Direction::Left,
    ]
    .iter()
    .map(|&direction| {
        let idx = get_uniq_idx_with_dir(position.0, position.1, cols as i32, direction);
        (distances[idx as usize], direction)
    })
    .min_by(|a, b| a.0.cmp(&b.0))
}

pub fn reindeer_olympics(file_path: &str) {
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

        let answer = get_min_node_dist_for_node(end_position, cols as i32, &distances);

        // The solution to find all nodes in path is. Just run Djikstra from end, from opposite
        // direction of reaching. And at last check for every node check, if dist_from start node + dist from end_node
        // in any direction is equal to the best shortest distance we found.
        if let Some((shortest_cost, dir_to_reach_end)) = answer {
            visited.clear();
            visited.resize(rows * cols * 4, false);

            let mut end_distances = Vec::new();
            end_distances.resize(rows * cols * 4, i32::MAX);
            // Find shortest path from end
            find_shortest_correct_path(
                &mut matrix,
                &mut visited,
                &mut end_distances,
                end_position,
                dir_to_reach_end.get_opposite_dir(),
            );

            let mut uniq_nodes: HashSet<(i32, i32)> = HashSet::new();

            for r in 0..rows {
                for c in 0..cols {
                    if matrix[r][c] == '#' {
                        continue;
                    }

                    for start_dir in [
                        Direction::Top,
                        Direction::Right,
                        Direction::Bottom,
                        Direction::Left,
                    ] {
                        let idx = get_uniq_idx_with_dir(r as i32, c as i32, cols as i32, start_dir);
                        let dist_from_start = distances[idx as usize];
                        if dist_from_start == i32::MAX {
                            continue;
                        }

                        for end_dir in [
                            Direction::Top,
                            Direction::Right,
                            Direction::Bottom,
                            Direction::Left,
                        ] {
                            let end_idx =
                                get_uniq_idx_with_dir(r as i32, c as i32, cols as i32, end_dir);
                            let dist_from_end = end_distances[end_idx as usize];

                            if dist_from_end == i32::MAX {
                                continue;
                            }

                            if dist_from_start + dist_from_end == shortest_cost {
                                uniq_nodes.insert((r as i32, c as i32));
                            }
                        }
                    }
                }
            }
            // print_nodes_in_path(&matrix, &uniq_nodes);
            println!(
                "Challenge 16 sortest cost {shortest_cost} and best nodes count: {:?}",
                uniq_nodes.len()
            );
        }
    }
}
