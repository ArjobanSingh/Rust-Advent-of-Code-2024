use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: Coord,
    velocity: Coord,
}

enum Quarter {
    TopRight,
    BottomRight,
    BottomLeft,
    TopLeft,
}

impl Quarter {
    fn get_quarter(&self) -> i32 {
        match self {
            Quarter::TopRight => 0,
            Quarter::BottomRight => 1,
            Quarter::BottomLeft => 2,
            Quarter::TopLeft => 3,
        }
    }
}

const H: i32 = 103; // no. of rows
const W: i32 = 101; // no. of cols
const MOV_COUNT: i32 = 100;

const MID_H: i32 = H / 2;
const MID_W: i32 = W / 2;

// Formulae Pos_new = ((Pos + Velocity * MOV_COUNT) % MAX + MAX) % MAX
fn get_value(pos: i32, velocity: i32, max: i32) -> i32 {
    // the formulae to get the new value after MOVE_COUNT moves with respect to negative values
    let value = ((pos + velocity * MOV_COUNT) % max + max) % max;
    value
}

// this gives the accurate values considering that pos is not in the center of the matrix
fn get_matrix_quarter(pos: Coord) -> Quarter {
    let x = pos.x - MID_W;
    let y = pos.y - MID_H;
    if y < 0 && x > 0 {
        Quarter::TopRight
    } else if y > 0 && x > 0 {
        Quarter::BottomRight
    } else if y > 0 && x < 0 {
        Quarter::BottomLeft
    } else {
        Quarter::TopLeft
    }
}

pub fn restroom_redoubt_v1(file_path: &str) {
    // read lines from the file in the file_path
    if let Ok(file) = File::open(file_path) {
        let re = Regex::new(r"-?\d+").unwrap();
        let mut count_per_quarter = [0, 0, 0, 0];

        let lines = io::BufReader::new(file).lines();

        for line in lines.flatten() {
            let nums: Vec<i32> = re
                .captures_iter(&line)
                .filter_map(|cap| cap.get(0)?.as_str().parse().ok()) // Only parse valid numbers
                .collect();

            let [px, py, vx, vy] = nums.try_into().unwrap();
            let mut robot = Robot {
                position: Coord { x: px, y: py },
                velocity: Coord { x: vx, y: vy },
            };

            robot.position.x = get_value(robot.position.x, robot.velocity.x, W);
            robot.position.y = get_value(robot.position.y, robot.velocity.y, H);

            // ignore the robots in the center row or column
            if robot.position.x == MID_W || robot.position.y == MID_H {
                continue;
            }

            let quarter = get_matrix_quarter(robot.position);
            count_per_quarter[quarter.get_quarter() as usize] += 1;

            // println!("{:?}", robot.position);
        }

        println!(
            "The anser for the Challenge 14 puz 1: {:?}",
            count_per_quarter.iter().fold(1, |acc, x| acc * x)
        );
    }
}
