use std::{
    cmp,
    fs::File,
    io::{self, BufRead},
};

const ROBOT: char = 'S';
const WALL: char = '#';
const END: char = 'E';
const SPACE: char = '.';
const ROTATE_COST: i32 = 1000;

#[derive(Debug, Clone, Copy)]
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

fn get_uniq_idx(r: usize, c: usize, cols: usize) -> usize {
    r * cols + c
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

fn find_shortest_correct_path(
    matrix: &mut Vec<Vec<char>>,
    visited: &mut Vec<bool>,
    dp: &mut Vec<Option<Option<i32>>>,
    position: (i32, i32),
    direction: Direction,
) -> Option<i32> {
    let cols = matrix[0].len();
    let value = matrix[position.0 as usize][position.1 as usize];
    let idx_with_dir =
        get_uniq_idx_with_dir(position.0, position.1, cols as i32, direction) as usize;

    if value == END {
        dp[idx_with_dir] = Some(Some(0));
        return Some(0);
    }

    let mut ans: Option<i32> = None;

    for neighbour_path in direction.get_neighbour_path() {
        let NeighbourPath {
            direction: neigh_direction,
            rotated_no,
        } = neighbour_path;

        let (dy, dx) = neigh_direction.to_offset();
        let (nr, nc) = (position.0 + dy, position.1 + dx);

        if !inside(matrix, (nr, nc)) {
            continue;
        }

        let next_ch = matrix[nr as usize][nc as usize];
        if next_ch == WALL {
            continue;
        }

        let next_idx_with_dir = get_uniq_idx_with_dir(nr, nc, cols as i32, neigh_direction);

        // check if already calculated
        let inner_ans = if let Some(prev_ans) = dp[next_idx_with_dir as usize] {
            prev_ans
        } else {
            let next_idx = get_uniq_idx(nr as usize, nc as usize, cols);
            if visited[next_idx] {
                continue;
            }

            visited[next_idx] = true;
            let returned =
                find_shortest_correct_path(matrix, visited, dp, (nr, nc), neigh_direction);
            visited[next_idx] = false;
            returned
        };

        if let Some(mut inner_ans) = inner_ans {
            inner_ans += ROTATE_COST * rotated_no;

            if let Some(ans_val) = ans {
                ans = Some(cmp::min(ans_val, inner_ans));
            } else {
                ans = Some(inner_ans);
            }
        }
    }

    // If we found the path, return by adding the points needed to come on this dir
    let cal_ans = if let Some(ans_val) = ans {
        Some(ans_val + 1)
    } else {
        None
    };

    dp[idx_with_dir] = Some(cal_ans);

    cal_ans
}

pub fn reindeer_olympics(file_path: &str) {
    if let Ok(file) = File::open(file_path) {
        let mut matrix: Vec<Vec<char>> = Vec::new();

        let lines = io::BufReader::new(file).lines();

        let mut robot_position: (i32, i32) = (0, 0);

        for line in lines.flatten() {
            let mut row: Vec<char> = Vec::new();
            for ch in line.chars() {
                if ch == ROBOT {
                    robot_position = (matrix.len() as i32, row.len() as i32);
                }
                row.push(ch);
            }
            matrix.push(row);
        }

        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut visited: Vec<bool> = Vec::new();
        visited.resize(rows * cols, false);

        let idx = get_uniq_idx(robot_position.0 as usize, robot_position.1 as usize, cols);
        visited[idx as usize] = true;

        let mut dp: Vec<Option<Option<i32>>> = Vec::new();
        dp.resize(rows * cols * 4, None);

        let result = find_shortest_correct_path(
            &mut matrix,
            &mut visited,
            &mut dp,
            robot_position,
            Direction::Right,
        );

        // for row in matrix.iter() {
        //     println!("{:?}", row);
        // }

        if let Some(result) = result {
            println!("The anser for the Challenge 15: {:?}", result);
        } else {
            println!("Is error")
        }
    }
}
