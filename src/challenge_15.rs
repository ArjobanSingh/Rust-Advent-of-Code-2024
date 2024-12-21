use std::{
    fs::File,
    io::{self, BufRead},
};

const ROBOT: char = '@';
const BOX: char = 'O';
const BOX_START: char = '[';
const BOX_END: char = ']';

const WALL: char = '#';
const SPACE: char = '.';

#[derive(Debug, Clone, Copy)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

impl Direction {
    fn new(direction: char) -> Option<Self> {
        match direction {
            '^' => Some(Direction::Top),
            '>' => Some(Direction::Right),
            'v' => Some(Direction::Bottom),
            '<' => Some(Direction::Left),
            _ => None,
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

fn inside(matrix: &Vec<Vec<char>>, position: (i32, i32)) -> bool {
    let rows = matrix.len() as i32;
    let cols = matrix[0].len() as i32;
    let (row, col) = position;

    !(row < 0 || row >= rows || col < 0 || col >= cols)
}

// Basically reach the end, and if can be moved. move them. Recurisevely kinda like backtracking
fn move_horizontal(
    matrix: &mut Vec<Vec<char>>,
    position: (i32, i32),
    direction: Direction,
) -> Option<(i32, i32)> {
    let (dy, dx) = direction.to_offset();
    let (nr, nc) = (position.0 + dy, position.1 + dx);

    let curr_ch = matrix[position.0 as usize][position.1 as usize];
    let next_ch = matrix[nr as usize][nc as usize];

    if next_ch == WALL {
        // can't move ahead
        return None;
    }

    // move horizontally if possible
    if next_ch == BOX_START || next_ch == BOX_END {
        if move_horizontal(matrix, (nr, nc), direction).is_none() {
            return None;
        }
    }

    // Either next item was box, which is already moved ahead or is empty, so move this one
    matrix[nr as usize][nc as usize] = curr_ch;
    matrix[position.0 as usize][position.1 as usize] = SPACE;
    Some((nr, nc))
}

fn move_vertical(
    matrix: &mut Vec<Vec<char>>,
    position: (i32, i32),
    direction: Direction,
) -> (i32, i32) {
    let (dy, dx) = direction.to_offset();
    let (nr, nc) = (position.0 + dy, position.1 + dx);

    let curr_ch = matrix[position.0 as usize][position.1 as usize];
    let next_ch = matrix[nr as usize][nc as usize];

    if next_ch == BOX_START || next_ch == BOX_END {
        let other_end_col = if next_ch == BOX_START { nc + 1 } else { nc - 1 };

        move_vertical(matrix, (nr, nc), direction);
        move_vertical(matrix, (nr, other_end_col), direction);
    }

    // Either next place is empty or a box which is already moved, so move this one
    matrix[nr as usize][nc as usize] = curr_ch;
    matrix[position.0 as usize][position.1 as usize] = SPACE;
    (nr, nc)
}

fn can_move_vertical(
    matrix: &mut Vec<Vec<char>>,
    position: (i32, i32),
    direction: Direction,
) -> bool {
    let (dy, dx) = direction.to_offset();
    let (nr, nc) = (position.0 + dy, position.1 + dx);

    let next_ch = matrix[nr as usize][nc as usize];

    if next_ch == WALL {
        // can't move ahead
        return false;
    }

    if next_ch == BOX_START || next_ch == BOX_END {
        let other_end_col = if next_ch == BOX_START { nc + 1 } else { nc - 1 };

        return can_move_vertical(matrix, (nr, nc), direction)
            && can_move_vertical(matrix, (nr, other_end_col), direction);
    }

    // Either next is empty space or a box sice which can further moved, so return true
    true
}

fn move_robot_in_matrix(
    matrix: &mut Vec<Vec<char>>,
    movements: Vec<char>,
    mut robot_position: (i32, i32),
) {
    for movement in movements {
        let next_direction = Direction::new(movement);
        if let Some(direction) = next_direction {
            let (dy, dx) = direction.to_offset();
            let (nr, nc) = (robot_position.0 + dy, robot_position.1 + dx);

            // if next position out of bounds, continue
            if !inside(&matrix, (nr, nc)) {
                continue;
            }

            let n_char = matrix[nr as usize][nc as usize];
            if n_char == WALL {
                continue;
            }

            if n_char == SPACE {
                // move user to that
                matrix[nr as usize][nc as usize] = ROBOT;
                matrix[robot_position.0 as usize][robot_position.1 as usize] = SPACE;
                robot_position = (nr, nc);
                continue;
            }

            let moved_pos = match direction {
                Direction::Left | Direction::Right => {
                    move_horizontal(matrix, robot_position, direction)
                }
                Direction::Bottom | Direction::Top => {
                    // Here first we need to check if boxes can be moved in vertical position
                    // as one box might move the other 2, and each of these 2 can move the 2 more.
                    // As each box can move 2 more. Kinda like exponential thing. It's like some kind of binary tree UI
                    // Once verified all the boxes in the area can be moved. Move them
                    if can_move_vertical(matrix, robot_position, direction) {
                        let next_pos = move_vertical(matrix, robot_position, direction);
                        Some(next_pos)
                    } else {
                        None
                    }
                }
            };

            if let Some((nr, nc)) = moved_pos {
                robot_position = (nr, nc);
            }
        }
    }
}

pub fn warehouse_woes(file_path: &str) {
    if let Ok(file) = File::open(file_path) {
        let mut result = 0;

        let mut movements: Vec<char> = Vec::new();
        let mut matrix: Vec<Vec<char>> = Vec::new();
        let lines = io::BufReader::new(file).lines();

        let mut is_reading_matrix = true;
        let mut robot_position: (i32, i32) = (0, 0);

        for line in lines.flatten() {
            if line.trim().is_empty() {
                is_reading_matrix = false;
                continue;
            }

            if is_reading_matrix {
                let mut row: Vec<char> = Vec::new();
                for ch in line.chars() {
                    match ch {
                        WALL | SPACE => {
                            row.push(ch);
                            row.push(ch);
                        }
                        BOX => {
                            row.push(BOX_START);
                            row.push(BOX_END);
                        }
                        ROBOT => {
                            robot_position = (matrix.len() as i32, row.len() as i32);
                            row.push(ch);

                            row.push(SPACE);
                        }
                        _ => (),
                    }
                }
                matrix.push(row);
            } else {
                for movement in line.chars() {
                    movements.push(movement);
                }
            }
        }

        move_robot_in_matrix(&mut matrix, movements, robot_position);

        let rows = matrix.len();
        let cols = matrix[0].len();

        for r in 0..rows {
            for c in 0..cols {
                if BOX_START == matrix[r][c] {
                    result += 100 * (r - 0) + (c - 0);
                }
            }
        }
        println!("The anser for the Challenge 15: {:?}", result);
    }
}
