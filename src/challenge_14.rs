use regex::Regex;
use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, Write},
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

const H: i32 = 103; // no. of rows
const W: i32 = 101; // no. of cols

fn get_value(pos: i32, velocity: i32, max: i32) -> i32 {
    let mut value = pos + velocity;
    if value < 0 {
        // if value is less than 0, wrap around
        value = max + value;
    } else {
        // if value is greater or equal to max, wrap around else return value
        value = value % max;
    }
    value
}

fn move_robot(robot: &mut Robot) {
    robot.position.x = get_value(robot.position.x, robot.velocity.x, W);
    robot.position.y = get_value(robot.position.y, robot.velocity.y, H);
}

fn print_robot_matrix(robots: &Vec<Robot>, index: i32) {
    let mut matrix = vec![vec!['.'; W as usize]; H as usize];
    for robot in robots.iter() {
        matrix[robot.position.y as usize][robot.position.x as usize] = '#';
    }

    // check if any row has consecutive X for more than 10 times
    let mut count = 0;

    // Instead of rendering every frame, we can check if the matrix has 10 consecutive cells in a row has X
    // Bit hacky but it works
    for row in matrix.iter() {
        for col in row.iter() {
            if count > 10 {
                break;
            }
            if *col == '#' {
                count += 1;
            } else {
                count = 0;
            }
        }
    }

    if count < 10 {
        return;
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("src/output/challenge_14_christmas_tree.txt")
        .unwrap();

    writeln!(file, "After: {} seconds", index + 1).unwrap();
    for row in matrix {
        let row_str: String = row.into_iter().collect();
        writeln!(file, "{}", row_str).unwrap();
    }
    writeln!(file, "\n\n").unwrap();
}

pub fn restroom_redoubt(file_path: &str) {
    // read lines from the file in the file_path
    if let Ok(file) = File::open(file_path) {
        let re = Regex::new(r"-?\d+").unwrap();

        let lines = io::BufReader::new(file).lines();
        let mut robots: Vec<Robot> = lines
            .flatten()
            .map(|line| {
                let nums: Vec<i32> = re
                    .captures_iter(&line)
                    .filter_map(|cap| cap.get(0)?.as_str().parse().ok()) // Only parse valid numbers
                    .collect();

                let [px, py, vx, vy] = nums.try_into().unwrap();
                Robot {
                    position: Coord { x: px, y: py },
                    velocity: Coord { x: vx, y: vy },
                }
            })
            .collect();

        for index in 0..(H * W) {
            for robot in robots.iter_mut() {
                move_robot(robot);
            }
            print_robot_matrix(&robots, index);
        }
    }
}
