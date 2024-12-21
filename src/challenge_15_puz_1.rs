use std::{
    fs::File,
    io::{self, BufRead},
};

const USER: char = '@';
const BOX: char = 'O';
const WALL: char = '#';
const SPACE: char = '.';

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

fn move_robot_in_matrix(
    matrix: &mut Vec<Vec<char>>,
    movements: Vec<char>,
    mut robot_position: (i32, i32),
) {
    for movement in movements {
        let next_direction = Direction::new(movement);
        if let Some(direction) = next_direction {
            let (dy, dx) = direction.to_offset();
            let next_offsets = (robot_position.0 + dy, robot_position.1 + dx);
            let (mut nr, mut nc) = next_offsets;

            // if next position out of bounds, continue
            if !inside(&matrix, next_offsets) {
                continue;
            }

            let n_char = matrix[nr as usize][nc as usize];
            if n_char == WALL {
                continue;
            }

            if n_char == SPACE {
                // move user to that
                matrix[nr as usize][nc as usize] = USER;
                matrix[robot_position.0 as usize][robot_position.1 as usize] = SPACE;
                robot_position = (nr, nc);
                continue;
            }

            // If next is box, iterate either we reach end or find empty space.
            let mut space_pos: Option<(i32, i32)> = None;

            loop {
                nr += dy;
                nc += dx;

                if !inside(&matrix, (nr, nc)) {
                    break;
                }

                // if we encounter wall, break the loop.
                let n_char = matrix[nr as usize][nc as usize];
                if n_char == WALL {
                    break;
                }

                if n_char == SPACE {
                    space_pos = Some((nr, nc));
                    break;
                }
            }

            if let Some(space) = space_pos {
                // get the original updated position after 1 movement;
                let (nr, nc) = next_offsets;

                // Move Box to that space, move user once in that direction,
                // and replace user's old position with empty SPACe
                matrix[space.0 as usize][space.1 as usize] = BOX;
                matrix[nr as usize][nc as usize] = USER;
                matrix[robot_position.0 as usize][robot_position.1 as usize] = SPACE;
                robot_position = (nr, nc);
            }
        }
    }
}

pub fn warehouse_woes_puz_1(file_path: &str) {
    if let Ok(file) = File::open(file_path) {
        let mut result = 0;

        let mut movements: Vec<char> = Vec::new();
        let mut matrix: Vec<Vec<char>> = Vec::new();
        let lines = io::BufReader::new(file).lines();

        let mut is_reading_matrix = true;
        let mut robot_position: (i32, i32) = (0, 0);

        for (r, line) in lines.flatten().enumerate() {
            if line.trim().is_empty() {
                is_reading_matrix = false;
                continue;
            }

            if is_reading_matrix {
                let mut row: Vec<char> = Vec::new();
                for (c, ch) in line.chars().enumerate() {
                    if ch == USER {
                        robot_position = (r as i32, c as i32);
                    }
                    row.push(ch);
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
                if BOX == matrix[r][c] {
                    result += 100 * (r - 0) + (c - 0);
                }
            }
        }

        println!("The anser for the Challenge 15 puz 1: {:?}", result);
    }
}
